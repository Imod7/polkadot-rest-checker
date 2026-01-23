use clap::Parser;
use reqwest;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use tokio;

mod chains;
mod coverage;
mod endpoints;

use chains::{Chain, Pallet};
use coverage::CoverageData;
use endpoints::EndpointType;

/// Polkadot REST API checker - test endpoint responses across block ranges
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Chain to test (polkadot, kusama, asset-hub-polkadot, asset-hub-kusama)
    #[arg(short, long, default_value = "polkadot")]
    chain: String,

    /// Endpoint type to test (consts, storage, dispatchables, errors, events, block, block-header, block-extrinsics, para-inclusions, runtime-spec, runtime-metadata, tx-material, node-version, node-network)
    #[arg(long, default_value = "consts")]
    endpoint: String,

    /// Start block number (default: 0)
    #[arg(short, long, default_value_t = 0)]
    start: u32,

    /// End block number (default: latest block)
    #[arg(short, long)]
    end: Option<u32>,

    /// Batch size for concurrent requests
    #[arg(short, long, default_value_t = 100)]
    batch_size: u32,

    /// Base URL for the new Rust API
    #[arg(short = 'u', long, default_value = "http://localhost:8080/v1")]
    url: String,

    /// Base URL for the old Sidecar API (for comparison)
    #[arg(long, default_value = "http://localhost:8045")]
    sidecar_url: String,

    /// Delay between batches in milliseconds
    #[arg(short, long, default_value_t = 100)]
    delay: u64,

    /// Filter to specific pallet name (case-insensitive, only for pallet endpoints)
    #[arg(short, long)]
    pallet: Option<String>,

    /// Path to coverage data file
    #[arg(long, default_value = "coverage/coverage.json")]
    coverage_file: String,

    /// Show coverage report and exit
    #[arg(long)]
    coverage_report: bool,
}

/// Result of testing a block against both APIs
#[derive(Debug)]
enum TestResult {
    /// Both APIs returned success with matching responses
    Match,
    /// Both APIs returned success but responses differ
    Mismatch { rust_response: Value, sidecar_response: Value },
    /// Rust API error
    RustError(String),
    /// Sidecar API error
    SidecarError(String),
    /// Both APIs returned errors
    BothError { rust_error: String, sidecar_error: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let coverage_path = Path::new(&args.coverage_file);

    // Load existing coverage data
    let mut coverage = CoverageData::load(coverage_path)?;

    // If --coverage-report flag is set, show report and exit
    if args.coverage_report {
        println!("{}", coverage.generate_report());
        return Ok(());
    }

    // Parse the chain argument
    let chain: Chain = args.chain.parse().map_err(|e: String| {
        eprintln!("Error: {}", e);
        eprintln!("\nAvailable chains:");
        for c in Chain::all() {
            eprintln!("  - {}", c);
        }
        e
    })?;

    // Parse the endpoint argument
    let endpoint_type: EndpointType = args.endpoint.parse().map_err(|e: String| {
        eprintln!("Error: {}", e);
        e
    })?;

    let rust_url = &args.url;
    let sidecar_url = &args.sidecar_url;
    let start_block = args.start;
    let batch_size = args.batch_size;
    let delay_between_batches = Duration::from_millis(args.delay);

    println!("Starting Polkadot REST API checker...");
    println!("Chain: {}", chain);
    println!("Endpoint: {}", endpoint_type);
    println!("Rust API URL: {}", rust_url);
    println!("Sidecar API URL: {}", sidecar_url);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    // Determine if we need blocks
    let end_block = if endpoint_type.requires_block() {
        match args.end {
            Some(end) => end,
            None => get_latest_block(&client, rust_url).await?,
        }
    } else {
        0 // Not used for non-block endpoints
    };

    if endpoint_type.requires_block() {
        println!("Block range: {} - {}", start_block, end_block);
        println!("Batch size: {}", batch_size);
    }

    // Get total pallets for this chain (for coverage tracking)
    let total_pallets = chain.pallets().len();

    // Route to appropriate scanning function based on endpoint type
    if endpoint_type.requires_pallet() {
        scan_pallet_endpoint(
            &client,
            &chain,
            &endpoint_type,
            rust_url,
            sidecar_url,
            start_block,
            end_block,
            batch_size,
            delay_between_batches,
            args.pallet.as_deref(),
            &mut coverage,
            total_pallets,
        )
        .await?;
    } else if endpoint_type.requires_block() {
        scan_block_endpoint(
            &client,
            &chain,
            &endpoint_type,
            rust_url,
            sidecar_url,
            start_block,
            end_block,
            batch_size,
            delay_between_batches,
            &mut coverage,
            total_pallets,
        )
        .await?;
    } else {
        scan_runtime_endpoint(
            &client,
            &chain,
            &endpoint_type,
            rust_url,
            sidecar_url,
            &mut coverage,
            total_pallets,
        )
        .await?;
    }

    // Save coverage data
    coverage.save(coverage_path)?;
    println!("\nCoverage data saved to: {}", args.coverage_file);

    // Save markdown report
    let markdown_path = Path::new("coverage/COVERAGE.md");
    coverage.save_markdown_report(markdown_path)?;
    println!("Coverage report saved to: coverage/COVERAGE.md");

    Ok(())
}

/// Scan pallet-based endpoints (iterates over pallets and blocks)
async fn scan_pallet_endpoint(
    client: &reqwest::Client,
    chain: &Chain,
    endpoint_type: &EndpointType,
    rust_url: &str,
    sidecar_url: &str,
    start_block: u32,
    end_block: u32,
    batch_size: u32,
    delay_between_batches: Duration,
    pallet_filter: Option<&str>,
    coverage: &mut CoverageData,
    total_pallets: usize,
) -> Result<(), Box<dyn Error>> {
    // Get pallets for the selected chain
    let all_pallets = chain.pallets();

    // Filter pallets if specified
    let pallets: Vec<&Pallet> = if let Some(filter) = pallet_filter {
        let filter_lower = filter.to_lowercase();
        all_pallets
            .iter()
            .filter(|p| p.name.to_lowercase().contains(&filter_lower))
            .collect()
    } else {
        all_pallets.iter().collect()
    };

    if pallets.is_empty() {
        println!("No pallets match the filter '{}'", pallet_filter.unwrap_or(""));
        return Ok(());
    }

    println!("Pallets to scan: {}", pallets.len());
    if pallet_filter.is_some() {
        for p in &pallets {
            println!("  - {} (index: {})", p.name, p.index);
        }
    }

    // Track results per pallet
    let mut pallet_results: Vec<PalletResult> = Vec::new();

    for pallet in pallets {
        println!("\n{}", "=".repeat(60));
        println!("Scanning pallet: {} (index: {}) - {}", pallet.name, pallet.index, endpoint_type);
        println!("{}", "=".repeat(60));

        // Create error log file
        let error_filename = format!(
            "errors_{}_{}-{}_{}_{}.log",
            chain, start_block, end_block, endpoint_type.short_name(), pallet.name
        );
        let mut error_file = File::create(&error_filename)?;
        writeln!(error_file, "# Error/Mismatch log for chain: {}, endpoint: {}, pallet: {} (index: {})",
            chain, endpoint_type, pallet.name, pallet.index)?;
        writeln!(error_file, "# Block range: {} - {}", start_block, end_block)?;
        writeln!(error_file, "# Rust API: {}", rust_url)?;
        writeln!(error_file, "# Sidecar API: {}", sidecar_url)?;
        writeln!(error_file, "#")?;

        let mut current_block = start_block;
        let mut matched = 0u32;
        let mut mismatched = 0u32;
        let mut rust_errors = 0u32;
        let mut sidecar_errors = 0u32;
        let mut both_errors = 0u32;
        let mut issues: Vec<(u32, String)> = Vec::new();

        while current_block <= end_block {
            let batch_end = std::cmp::min(current_block + batch_size, end_block + 1);
            let blocks: Vec<u32> = (current_block..batch_end).collect();

            if current_block % 1000 == 0 || current_block == start_block {
                println!("  Processing blocks {} to {}...", current_block, batch_end - 1);
            }

            let mut tasks = Vec::new();
            for block_num in blocks {
                let client_clone = client.clone();
                let rust_path = endpoint_type.path(Some(pallet.name), Some(block_num));
                let sidecar_path = endpoint_type.path(Some(pallet.name), Some(block_num));
                let rust_api_url = format!("{}{}", rust_url, rust_path);
                let sidecar_api_url = format!("{}{}", sidecar_url, sidecar_path);

                println!("  Block {}: {} vs {}", block_num, rust_api_url, sidecar_api_url);

                tasks.push(tokio::spawn(async move {
                    test_block_compare(client_clone, rust_api_url, sidecar_api_url, block_num).await
                }));
            }

            for task in tasks {
                let (block_num, result) = task.await?;
                process_result(
                    block_num,
                    result,
                    &mut matched,
                    &mut mismatched,
                    &mut rust_errors,
                    &mut sidecar_errors,
                    &mut both_errors,
                    &mut issues,
                    &mut error_file,
                )?;
            }

            current_block = batch_end;

            if current_block <= end_block {
                tokio::time::sleep(delay_between_batches).await;
            }
        }

        let total = matched + mismatched + rust_errors + sidecar_errors + both_errors;
        let match_rate = if total > 0 {
            (matched as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        println!(
            "  {} - Matched: {}/{} ({:.2}%), Mismatched: {}, Rust Errors: {}, Sidecar Errors: {}, Both Errors: {}",
            pallet.name, matched, total, match_rate, mismatched, rust_errors, sidecar_errors, both_errors
        );

        let has_issues = mismatched > 0 || rust_errors > 0 || sidecar_errors > 0 || both_errors > 0;
        if has_issues {
            println!("  Issues saved to: {}", error_filename);
        } else {
            std::fs::remove_file(&error_filename).ok();
        }

        pallet_results.push(PalletResult {
            name: pallet.name.to_string(),
            index: pallet.index,
            matched,
            mismatched,
            rust_errors,
            sidecar_errors,
            both_errors,
            issues,
        });

        // Record coverage for this pallet
        let chain_coverage = coverage.get_chain(&chain.to_string(), total_pallets);
        let endpoint_coverage = chain_coverage.get_endpoint(endpoint_type.short_name(), true);
        endpoint_coverage.add_pallet_run(
            pallet.name,
            start_block,
            end_block,
            matched,
            mismatched,
            rust_errors,
            sidecar_errors,
            both_errors,
        );
    }

    // Print final summary
    print_pallet_summary(&pallet_results, chain, endpoint_type, start_block, end_block);

    Ok(())
}

/// Scan block-based endpoints (iterates over blocks only)
async fn scan_block_endpoint(
    client: &reqwest::Client,
    chain: &Chain,
    endpoint_type: &EndpointType,
    rust_url: &str,
    sidecar_url: &str,
    start_block: u32,
    end_block: u32,
    batch_size: u32,
    delay_between_batches: Duration,
    coverage: &mut CoverageData,
    total_pallets: usize,
) -> Result<(), Box<dyn Error>> {
    println!("\n{}", "=".repeat(60));
    println!("Scanning endpoint: {}", endpoint_type);
    println!("{}", "=".repeat(60));

    // Create error log file
    let error_filename = format!(
        "errors_{}_{}-{}_{}.log",
        chain, start_block, end_block, endpoint_type.short_name()
    );
    let mut error_file = File::create(&error_filename)?;
    writeln!(error_file, "# Error/Mismatch log for chain: {}, endpoint: {}", chain, endpoint_type)?;
    writeln!(error_file, "# Block range: {} - {}", start_block, end_block)?;
    writeln!(error_file, "# Rust API: {}", rust_url)?;
    writeln!(error_file, "# Sidecar API: {}", sidecar_url)?;
    writeln!(error_file, "#")?;

    let mut current_block = start_block;
    let mut matched = 0u32;
    let mut mismatched = 0u32;
    let mut rust_errors = 0u32;
    let mut sidecar_errors = 0u32;
    let mut both_errors = 0u32;
    let mut issues: Vec<(u32, String)> = Vec::new();

    while current_block <= end_block {
        let batch_end = std::cmp::min(current_block + batch_size, end_block + 1);
        let blocks: Vec<u32> = (current_block..batch_end).collect();

        if current_block % 1000 == 0 || current_block == start_block {
            println!("  Processing blocks {} to {}...", current_block, batch_end - 1);
        }

        let mut tasks = Vec::new();
        for block_num in blocks {
            let client_clone = client.clone();
            let rust_path = endpoint_type.path(None, Some(block_num));
            let sidecar_path = endpoint_type.path(None, Some(block_num));
            let rust_api_url = format!("{}{}", rust_url, rust_path);
            let sidecar_api_url = format!("{}{}", sidecar_url, sidecar_path);

            println!("  Block {}: {} vs {}", block_num, rust_api_url, sidecar_api_url);

            tasks.push(tokio::spawn(async move {
                test_block_compare(client_clone, rust_api_url, sidecar_api_url, block_num).await
            }));
        }

        for task in tasks {
            let (block_num, result) = task.await?;
            process_result(
                block_num,
                result,
                &mut matched,
                &mut mismatched,
                &mut rust_errors,
                &mut sidecar_errors,
                &mut both_errors,
                &mut issues,
                &mut error_file,
            )?;
        }

        current_block = batch_end;

        if current_block <= end_block {
            tokio::time::sleep(delay_between_batches).await;
        }
    }

    let total = matched + mismatched + rust_errors + sidecar_errors + both_errors;
    let match_rate = if total > 0 {
        (matched as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    println!(
        "\n{} - Matched: {}/{} ({:.2}%), Mismatched: {}, Rust Errors: {}, Sidecar Errors: {}, Both Errors: {}",
        endpoint_type, matched, total, match_rate, mismatched, rust_errors, sidecar_errors, both_errors
    );

    let has_issues = mismatched > 0 || rust_errors > 0 || sidecar_errors > 0 || both_errors > 0;
    if has_issues {
        println!("Issues saved to: {}", error_filename);
    } else {
        std::fs::remove_file(&error_filename).ok();
    }

    // Record coverage
    let chain_coverage = coverage.get_chain(&chain.to_string(), total_pallets);
    let endpoint_coverage = chain_coverage.get_endpoint(endpoint_type.short_name(), false);
    endpoint_coverage.add_block_run(
        start_block,
        end_block,
        matched,
        mismatched,
        rust_errors,
        sidecar_errors,
        both_errors,
    );

    // Print summary
    print_block_summary(endpoint_type, chain, start_block, end_block, matched, mismatched, rust_errors, sidecar_errors, both_errors, &issues);

    Ok(())
}

/// Scan runtime endpoints (single request, no iteration)
async fn scan_runtime_endpoint(
    client: &reqwest::Client,
    chain: &Chain,
    endpoint_type: &EndpointType,
    rust_url: &str,
    sidecar_url: &str,
    coverage: &mut CoverageData,
    total_pallets: usize,
) -> Result<(), Box<dyn Error>> {
    // Create summary log file
    let summary_filename = format!("summary_{}_{}.log", chain, endpoint_type.short_name());
    let mut summary_file = File::create(&summary_filename)?;

    // Helper macro to print to both console and file
    macro_rules! log_line {
        ($($arg:tt)*) => {
            println!($($arg)*);
            writeln!(summary_file, $($arg)*).ok();
        };
    }

    log_line!("\n{}", "=".repeat(60));
    log_line!("Testing endpoint: {}", endpoint_type);
    log_line!("{}", "=".repeat(60));

    let rust_path = endpoint_type.path(None, None);
    let sidecar_path = endpoint_type.path(None, None);
    let rust_api_url = format!("{}{}", rust_url, rust_path);
    let sidecar_api_url = format!("{}{}", sidecar_url, sidecar_path);

    log_line!("  Rust API: {}", rust_api_url);
    log_line!("  Sidecar API: {}", sidecar_api_url);

    let (_, result) = test_block_compare(client.clone(), rust_api_url.clone(), sidecar_api_url.clone(), 0).await;

    // Track coverage result
    let chain_coverage = coverage.get_chain(&chain.to_string(), total_pallets);
    let endpoint_coverage = chain_coverage.get_endpoint(endpoint_type.short_name(), false);

    match result {
        TestResult::Match => {
            log_line!("\n  Result: MATCH - Both APIs returned identical responses");
            endpoint_coverage.add_runtime_run(true, None);
        }
        TestResult::Mismatch { rust_response, sidecar_response } => {
            log_line!("\n  Result: MISMATCH - Responses differ");
            endpoint_coverage.add_runtime_run(false, None);

            let error_filename = format!("errors_{}_{}.log", chain, endpoint_type.short_name());
            let mut error_file = File::create(&error_filename)?;
            writeln!(error_file, "# Mismatch log for chain: {}, endpoint: {}", chain, endpoint_type)?;
            writeln!(error_file, "# Rust API: {}", rust_api_url)?;
            writeln!(error_file, "# Sidecar API: {}", sidecar_api_url)?;
            writeln!(error_file, "#")?;
            writeln!(error_file, "MISMATCH - Responses differ")?;
            writeln!(error_file, "Rust API response: {}", serde_json::to_string_pretty(&rust_response)?)?;
            writeln!(error_file, "Sidecar response: {}", serde_json::to_string_pretty(&sidecar_response)?)?;

            log_line!("  Details saved to: {}", error_filename);
        }
        TestResult::RustError(ref err) => {
            log_line!("\n  Result: RUST API ERROR - {}", err);
            endpoint_coverage.add_runtime_run(false, Some(err));
        }
        TestResult::SidecarError(ref err) => {
            log_line!("\n  Result: SIDECAR ERROR - {}", err);
            endpoint_coverage.add_runtime_run(false, Some(err));
        }
        TestResult::BothError { ref rust_error, ref sidecar_error } => {
            log_line!("\n  Result: BOTH APIS ERROR");
            log_line!("    Rust: {}", rust_error);
            log_line!("    Sidecar: {}", sidecar_error);
            endpoint_coverage.add_runtime_run(false, Some(rust_error));
        }
    }

    println!("\nSummary saved to: {}", summary_filename);

    Ok(())
}

/// Process a test result and update counters
fn process_result(
    block_num: u32,
    result: TestResult,
    matched: &mut u32,
    mismatched: &mut u32,
    rust_errors: &mut u32,
    sidecar_errors: &mut u32,
    both_errors: &mut u32,
    issues: &mut Vec<(u32, String)>,
    error_file: &mut File,
) -> Result<(), Box<dyn Error>> {
    match result {
        TestResult::Match => {
            *matched += 1;
        }
        TestResult::Mismatch { rust_response, sidecar_response } => {
            *mismatched += 1;
            let msg = "MISMATCH - Responses differ".to_string();
            writeln!(error_file, "Block {}: {}", block_num, msg)?;
            writeln!(error_file, "  Rust API response: {}", serde_json::to_string_pretty(&rust_response)?)?;
            writeln!(error_file, "  Sidecar response: {}", serde_json::to_string_pretty(&sidecar_response)?)?;
            writeln!(error_file)?;
            issues.push((block_num, msg));
        }
        TestResult::RustError(err) => {
            *rust_errors += 1;
            let msg = format!("RUST API ERROR: {}", err);
            writeln!(error_file, "Block {}: {}", block_num, msg)?;
            issues.push((block_num, msg));
        }
        TestResult::SidecarError(err) => {
            *sidecar_errors += 1;
            let msg = format!("SIDECAR ERROR: {}", err);
            writeln!(error_file, "Block {}: {}", block_num, msg)?;
            issues.push((block_num, msg));
        }
        TestResult::BothError { rust_error, sidecar_error } => {
            *both_errors += 1;
            let msg = format!("BOTH ERRORS - Rust: {}, Sidecar: {}", rust_error, sidecar_error);
            writeln!(error_file, "Block {}: {}", block_num, msg)?;
            issues.push((block_num, msg));
        }
    }
    Ok(())
}

struct PalletResult {
    name: String,
    index: u8,
    matched: u32,
    mismatched: u32,
    rust_errors: u32,
    sidecar_errors: u32,
    both_errors: u32,
    issues: Vec<(u32, String)>,
}

fn print_pallet_summary(
    results: &[PalletResult],
    chain: &Chain,
    endpoint_type: &EndpointType,
    start_block: u32,
    end_block: u32,
) {
    // Create summary log file
    let summary_filename = format!(
        "summary_{}_{}-{}_{}.log",
        chain, start_block, end_block, endpoint_type.short_name()
    );
    let mut summary_file = File::create(&summary_filename).ok();

    // Helper macro to print to both console and file
    macro_rules! log_line {
        ($($arg:tt)*) => {
            println!($($arg)*);
            if let Some(ref mut f) = summary_file {
                writeln!(f, $($arg)*).ok();
            }
        };
    }

    log_line!("\n\n{}", "=".repeat(90));
    log_line!("                                    FINAL SUMMARY");
    log_line!("{}", "=".repeat(90));
    log_line!("Chain: {}", chain);
    log_line!("Endpoint: {}", endpoint_type);
    log_line!("Block range: {} - {}", start_block, end_block);
    log_line!("Total pallets scanned: {}\n", results.len());

    log_line!(
        "{:<25} {:>8} {:>10} {:>10} {:>10} {:>10} {:>8}",
        "Pallet", "Matched", "Mismatch", "RustErr", "SidecarErr", "BothErr", "Rate"
    );
    log_line!("{}", "-".repeat(90));

    let mut total_matched = 0u32;
    let mut total_mismatched = 0u32;
    let mut total_rust_errors = 0u32;
    let mut total_sidecar_errors = 0u32;
    let mut total_both_errors = 0u32;

    for result in results {
        let total = result.matched + result.mismatched + result.rust_errors + result.sidecar_errors + result.both_errors;
        let rate = if total > 0 {
            (result.matched as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        log_line!(
            "{:<25} {:>8} {:>10} {:>10} {:>10} {:>10} {:>7.2}%",
            result.name, result.matched, result.mismatched, result.rust_errors,
            result.sidecar_errors, result.both_errors, rate
        );

        total_matched += result.matched;
        total_mismatched += result.mismatched;
        total_rust_errors += result.rust_errors;
        total_sidecar_errors += result.sidecar_errors;
        total_both_errors += result.both_errors;
    }

    log_line!("{}", "-".repeat(90));
    let overall_total = total_matched + total_mismatched + total_rust_errors + total_sidecar_errors + total_both_errors;
    let overall_rate = if overall_total > 0 {
        (total_matched as f64 / overall_total as f64) * 100.0
    } else {
        0.0
    };
    log_line!(
        "{:<25} {:>8} {:>10} {:>10} {:>10} {:>10} {:>7.2}%",
        "TOTAL", total_matched, total_mismatched, total_rust_errors,
        total_sidecar_errors, total_both_errors, overall_rate
    );

    // Print issue summary
    let pallets_with_issues: Vec<_> = results.iter().filter(|r| !r.issues.is_empty()).collect();

    if !pallets_with_issues.is_empty() {
        log_line!("\n\n{}", "=".repeat(90));
        log_line!("                                   ISSUE DETAILS");
        log_line!("{}", "=".repeat(90));

        for result in &pallets_with_issues {
            log_line!("\n{} (index {}):", result.name, result.index);
            for (i, (block, error)) in result.issues.iter().enumerate() {
                if i >= 10 {
                    log_line!("  ... and {} more issues (see error log file)", result.issues.len() - 10);
                    break;
                }
                log_line!("  Block {}: {}", block, error);
            }
        }
    }

    println!("\nSummary saved to: {}", summary_filename);
}

fn print_block_summary(
    endpoint_type: &EndpointType,
    chain: &Chain,
    start_block: u32,
    end_block: u32,
    matched: u32,
    mismatched: u32,
    rust_errors: u32,
    sidecar_errors: u32,
    both_errors: u32,
    issues: &[(u32, String)],
) {
    // Create summary log file
    let summary_filename = format!(
        "summary_{}_{}-{}_{}.log",
        chain, start_block, end_block, endpoint_type.short_name()
    );
    let mut summary_file = File::create(&summary_filename).ok();

    // Helper macro to print to both console and file
    macro_rules! log_line {
        ($($arg:tt)*) => {
            println!($($arg)*);
            if let Some(ref mut f) = summary_file {
                writeln!(f, $($arg)*).ok();
            }
        };
    }

    log_line!("\n\n{}", "=".repeat(90));
    log_line!("                                    FINAL SUMMARY");
    log_line!("{}", "=".repeat(90));
    log_line!("Chain: {}", chain);
    log_line!("Endpoint: {}", endpoint_type);
    log_line!("Block range: {} - {}\n", start_block, end_block);

    let total = matched + mismatched + rust_errors + sidecar_errors + both_errors;
    let rate = if total > 0 {
        (matched as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    log_line!("Matched:        {} / {} ({:.2}%)", matched, total, rate);
    log_line!("Mismatched:     {}", mismatched);
    log_line!("Rust Errors:    {}", rust_errors);
    log_line!("Sidecar Errors: {}", sidecar_errors);
    log_line!("Both Errors:    {}", both_errors);

    if !issues.is_empty() {
        log_line!("\n{}", "=".repeat(90));
        log_line!("                                   ISSUE DETAILS");
        log_line!("{}", "=".repeat(90));

        for (i, (block, error)) in issues.iter().enumerate() {
            if i >= 20 {
                log_line!("  ... and {} more issues (see error log file)", issues.len() - 20);
                break;
            }
            log_line!("  Block {}: {}", block, error);
        }
    }

    println!("\nSummary saved to: {}", summary_filename);
}

async fn get_latest_block(client: &reqwest::Client, base_url: &str) -> Result<u32, Box<dyn Error>> {
    let url = format!("{}/blocks/head", base_url);

    let response = client.get(&url).send().await
        .map_err(|e| format!("Failed to connect to {}: {}", url, e))?;

    if !response.status().is_success() {
        return Err(format!("Server returned HTTP {}: {}", response.status(), url).into());
    }

    let json: Value = response.json().await
        .map_err(|e| format!("Invalid JSON response from {}: {}", url, e))?;

    json["number"]
        .as_str()
        .and_then(|s| s.parse::<u32>().ok())
        .ok_or_else(|| format!("Missing or invalid 'number' field in response from {}", url).into())
}

/// Fetch JSON from a URL, returning Ok(Value) on success or Err(String) on failure
async fn fetch_json(client: &reqwest::Client, url: &str) -> Result<Value, String> {
    match client.get(url).send().await {
        Ok(response) => {
            let status = response.status();
            if status.is_success() {
                response.json::<Value>().await
                    .map_err(|e| format!("Invalid JSON: {}", e))
            } else {
                Err(format!("HTTP {}", status))
            }
        }
        Err(e) => Err(format!("Request failed: {}", e))
    }
}

/// Test a block by comparing responses from both APIs
async fn test_block_compare(
    client: reqwest::Client,
    rust_url: String,
    sidecar_url: String,
    block_num: u32,
) -> (u32, TestResult) {
    // Fetch from both APIs concurrently
    let (rust_result, sidecar_result) = tokio::join!(
        fetch_json(&client, &rust_url),
        fetch_json(&client, &sidecar_url)
    );

    let result = match (rust_result, sidecar_result) {
        (Ok(rust_json), Ok(sidecar_json)) => {
            if json_equal(&rust_json, &sidecar_json) {
                TestResult::Match
            } else {
                TestResult::Mismatch {
                    rust_response: rust_json,
                    sidecar_response: sidecar_json,
                }
            }
        }
        (Err(rust_err), Ok(_)) => TestResult::RustError(rust_err),
        (Ok(_), Err(sidecar_err)) => TestResult::SidecarError(sidecar_err),
        (Err(rust_err), Err(sidecar_err)) => TestResult::BothError {
            rust_error: rust_err,
            sidecar_error: sidecar_err,
        },
    };

    (block_num, result)
}

/// Compare two JSON values for equality, ignoring field order and string case
fn json_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Object(a_map), Value::Object(b_map)) => {
            if a_map.len() != b_map.len() {
                return false;
            }
            a_map.iter().all(|(key, a_val)| {
                b_map.get(key).map_or(false, |b_val| json_equal(a_val, b_val))
            })
        }
        (Value::Array(a_arr), Value::Array(b_arr)) => {
            if a_arr.len() != b_arr.len() {
                return false;
            }
            a_arr.iter().zip(b_arr.iter()).all(|(a_val, b_val)| json_equal(a_val, b_val))
        }
        // Case-insensitive string comparison
        (Value::String(a_str), Value::String(b_str)) => {
            a_str.to_lowercase() == b_str.to_lowercase()
        }
        _ => a == b,
    }
}
