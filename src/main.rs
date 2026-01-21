use clap::Parser;
use reqwest;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use tokio;

mod chains;
use chains::{Chain, Pallet};

/// Polkadot REST API checker - test endpoint responses across block ranges
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Chain to test (polkadot, kusama, asset-hub-polkadot, asset-hub-kusama)
    #[arg(short, long, default_value = "polkadot")]
    chain: String,

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

    /// Filter to specific pallet name (case-insensitive)
    #[arg(short, long)]
    pallet: Option<String>,
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

    // Parse the chain argument
    let chain: Chain = args.chain.parse().map_err(|e: String| {
        eprintln!("Error: {}", e);
        eprintln!("\nAvailable chains:");
        for c in Chain::all() {
            eprintln!("  - {}", c);
        }
        e
    })?;

    let rust_url = &args.url;
    let sidecar_url = &args.sidecar_url;
    let start_block = args.start;
    let batch_size = args.batch_size;
    let delay_between_batches = Duration::from_millis(args.delay);

    println!("Starting Polkadot REST API checker...");
    println!("Chain: {}", chain);
    println!("Rust API URL: {}", rust_url);
    println!("Sidecar API URL: {}", sidecar_url);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    // Get end block - either from args or fetch latest
    let end_block = match args.end {
        Some(end) => end,
        None => get_latest_block(&client, rust_url).await?,
    };

    println!("Block range: {} - {}", start_block, end_block);
    println!("Batch size: {}", batch_size);

    // Get pallets for the selected chain
    let all_pallets = chain.pallets();

    // Filter pallets if specified
    let pallets: Vec<&Pallet> = if let Some(ref pallet_filter) = args.pallet {
        let filter_lower = pallet_filter.to_lowercase();
        all_pallets
            .iter()
            .filter(|p| p.name.to_lowercase().contains(&filter_lower))
            .collect()
    } else {
        all_pallets.iter().collect()
    };

    if pallets.is_empty() {
        println!("No pallets match the filter '{}'", args.pallet.unwrap_or_default());
        return Ok(());
    }

    println!("Pallets to scan: {}", pallets.len());
    if args.pallet.is_some() {
        for p in &pallets {
            println!("  - {} (index: {})", p.name, p.index);
        }
    }

    // Track results per pallet
    let mut pallet_results: Vec<PalletResult> = Vec::new();

    for pallet in pallets {
        println!("\n{}", "=".repeat(60));
        println!("Scanning pallet: {} (index: {})", pallet.name, pallet.index);
        println!("{}", "=".repeat(60));

        let endpoint_path = format!("/pallets/{}/consts", pallet.name);

        // Create error log file for this pallet and block range
        let error_filename = format!("errors_{}_{}-{}_{}.log", chain, start_block, end_block, pallet.name);
        let mut error_file = File::create(&error_filename)?;
        writeln!(error_file, "# Error/Mismatch log for chain: {}, pallet: {} (index: {})", chain, pallet.name, pallet.index)?;
        writeln!(error_file, "# Block range: {} - {}", start_block, end_block)?;
        writeln!(error_file, "# Rust API: {}{}", rust_url, endpoint_path)?;
        writeln!(error_file, "# Sidecar API: {}{}", sidecar_url, endpoint_path)?;
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
                let rust_api_url = format!("{}{}?at={}", rust_url, endpoint_path, block_num);
                let sidecar_api_url = format!("{}{}?at={}", sidecar_url, endpoint_path, block_num);

                println!("  Block {}: {} vs {}", block_num, rust_api_url, sidecar_api_url);

                tasks.push(tokio::spawn(async move {
                    test_block_compare(client_clone, rust_api_url, sidecar_api_url, block_num).await
                }));
            }

            for task in tasks {
                let (block_num, result) = task.await?;
                match result {
                    TestResult::Match => {
                        matched += 1;
                    }
                    TestResult::Mismatch { rust_response, sidecar_response } => {
                        mismatched += 1;
                        let msg = format!("MISMATCH - Responses differ");
                        writeln!(error_file, "Block {}: {}", block_num, msg)?;
                        writeln!(error_file, "  Rust API response: {}", serde_json::to_string_pretty(&rust_response).unwrap_or_default())?;
                        writeln!(error_file, "  Sidecar response: {}", serde_json::to_string_pretty(&sidecar_response).unwrap_or_default())?;
                        writeln!(error_file, "")?;
                        issues.push((block_num, msg));
                    }
                    TestResult::RustError(err) => {
                        rust_errors += 1;
                        let msg = format!("RUST API ERROR: {}", err);
                        writeln!(error_file, "Block {}: {}", block_num, msg)?;
                        issues.push((block_num, msg));
                    }
                    TestResult::SidecarError(err) => {
                        sidecar_errors += 1;
                        let msg = format!("SIDECAR ERROR: {}", err);
                        writeln!(error_file, "Block {}: {}", block_num, msg)?;
                        issues.push((block_num, msg));
                    }
                    TestResult::BothError { rust_error, sidecar_error } => {
                        both_errors += 1;
                        let msg = format!("BOTH ERRORS - Rust: {}, Sidecar: {}", rust_error, sidecar_error);
                        writeln!(error_file, "Block {}: {}", block_num, msg)?;
                        issues.push((block_num, msg));
                    }
                }
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

        println!("  {} - Matched: {}/{} ({:.2}%), Mismatched: {}, Rust Errors: {}, Sidecar Errors: {}, Both Errors: {}",
            pallet.name, matched, total, match_rate, mismatched, rust_errors, sidecar_errors, both_errors);

        let has_issues = mismatched > 0 || rust_errors > 0 || sidecar_errors > 0 || both_errors > 0;
        if has_issues {
            println!("  Issues saved to: {}", error_filename);
        } else {
            // Remove empty error file if no issues
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
    }

    // Print final summary
    print_summary(&pallet_results, &chain, start_block, end_block);

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

fn print_summary(results: &[PalletResult], chain: &Chain, start_block: u32, end_block: u32) {
    println!("\n\n{}", "=".repeat(90));
    println!("                                    FINAL SUMMARY");
    println!("{}", "=".repeat(90));
    println!("Chain: {}", chain);
    println!("Block range: {} - {}", start_block, end_block);
    println!("Total pallets scanned: {}\n", results.len());

    println!("{:<25} {:>8} {:>10} {:>10} {:>10} {:>10} {:>8}",
        "Pallet", "Matched", "Mismatch", "RustErr", "SidecarErr", "BothErr", "Rate");
    println!("{}", "-".repeat(90));

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

        println!("{:<25} {:>8} {:>10} {:>10} {:>10} {:>10} {:>7.2}%",
            result.name, result.matched, result.mismatched, result.rust_errors,
            result.sidecar_errors, result.both_errors, rate);

        total_matched += result.matched;
        total_mismatched += result.mismatched;
        total_rust_errors += result.rust_errors;
        total_sidecar_errors += result.sidecar_errors;
        total_both_errors += result.both_errors;
    }

    println!("{}", "-".repeat(90));
    let overall_total = total_matched + total_mismatched + total_rust_errors + total_sidecar_errors + total_both_errors;
    let overall_rate = if overall_total > 0 {
        (total_matched as f64 / overall_total as f64) * 100.0
    } else {
        0.0
    };
    println!("{:<25} {:>8} {:>10} {:>10} {:>10} {:>10} {:>7.2}%",
        "TOTAL", total_matched, total_mismatched, total_rust_errors,
        total_sidecar_errors, total_both_errors, overall_rate);

    // Print issue summary for pallets with problems
    let pallets_with_issues: Vec<_> = results.iter()
        .filter(|r| !r.issues.is_empty())
        .collect();

    if !pallets_with_issues.is_empty() {
        println!("\n\n{}", "=".repeat(90));
        println!("                                   ISSUE DETAILS");
        println!("{}", "=".repeat(90));

        for result in pallets_with_issues {
            println!("\n{} (index {}):", result.name, result.index);
            // Only show first 10 issues per pallet in summary
            for (i, (block, error)) in result.issues.iter().enumerate() {
                if i >= 10 {
                    println!("  ... and {} more issues (see log file)", result.issues.len() - 10);
                    break;
                }
                println!("  Block {}: {}", block, error);
            }
        }
    }
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
    block_num: u32
) -> (u32, TestResult) {
    // Fetch from both APIs concurrently
    let (rust_result, sidecar_result) = tokio::join!(
        fetch_json(&client, &rust_url),
        fetch_json(&client, &sidecar_url)
    );

    let result = match (rust_result, sidecar_result) {
        (Ok(rust_json), Ok(sidecar_json)) => {
            // Both succeeded - compare responses
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

/// Compare two JSON values for equality, ignoring field order
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
        _ => a == b,
    }
}
