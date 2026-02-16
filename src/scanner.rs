use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

use crate::chains::Chain;
use crate::coverage::CoverageData;
use crate::endpoints::EndpointType;
use crate::http::{fetch_json, test_block_compare, TestResult};
use crate::reporting::{
    print_account_summary, print_block_summary, print_pallet_summary,
    write_account_mismatch_report, write_block_mismatch_report, write_pallet_mismatch_report,
    AccountResult, PalletResult,
};

/// Print inline diff details for non-match results
fn log_result_inline(display_id: &str, result: &TestResult) {
    match result {
        TestResult::Match => {}
        TestResult::Mismatch { diffs, .. } => {
            println!(
                "    {}: MISMATCH ({} diff{})",
                display_id,
                diffs.len(),
                if diffs.len() == 1 { "" } else { "s" }
            );
            // Show first 3 diffs inline for quick debugging
            for diff in diffs.iter().take(3) {
                println!("      - {}", diff);
            }
            if diffs.len() > 3 {
                println!("      ... and {} more", diffs.len() - 3);
            }
        }
        TestResult::RustError(e) => {
            println!("    {}: Rust Error - {}", display_id, e)
        }
        TestResult::SidecarError(e) => {
            println!("    {}: Sidecar Error - {}", display_id, e)
        }
        TestResult::BothError {
            rust_error,
            sidecar_error,
        } => {
            if rust_error == sidecar_error {
                // Same error = silent match
            } else {
                println!(
                    "    {}: Both APIs Error (different codes: rust={}, sidecar={})",
                    display_id, rust_error, sidecar_error
                );
            }
        }
    }
}

/// Scan pallet-based endpoints (iterates over pallets and blocks)
pub async fn scan_pallet_endpoint(
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
    create_logs: bool,
    create_report: bool,
) -> Result<(), Box<dyn Error>> {
    // Get pallets for the selected chain
    let all_pallets = chain.pallets();

    // Filter pallets if specified
    let pallets: Vec<&crate::chains::Pallet> = if let Some(filter) = pallet_filter {
        let filter_lower = filter.to_lowercase();
        all_pallets
            .iter()
            .filter(|p| p.name.to_lowercase().contains(&filter_lower))
            .collect()
    } else {
        all_pallets.iter().collect()
    };

    if pallets.is_empty() {
        println!(
            "No pallets match the filter '{}'",
            pallet_filter.unwrap_or("")
        );
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
        println!(
            "Scanning pallet: {} (index: {}) - {}",
            pallet.name, pallet.index, endpoint_type
        );
        println!("{}", "=".repeat(60));

        // Create error log file (only if --logs flag is set)
        let error_filename = format!(
            "errors_{}_{}-{}_{}_{}.log",
            chain, start_block, end_block, endpoint_type, pallet.name
        );
        let mut error_file: Option<File> = if create_logs {
            let mut f = File::create(&error_filename)?;
            writeln!(
                f,
                "# Error/Mismatch log for chain: {}, endpoint: {}, pallet: {} (index: {})",
                chain, endpoint_type, pallet.name, pallet.index
            )?;
            writeln!(f, "# Block range: {} - {}", start_block, end_block)?;
            writeln!(f, "# Rust API: {}", rust_url)?;
            writeln!(f, "# Sidecar API: {}", sidecar_url)?;
            writeln!(f, "#")?;
            Some(f)
        } else {
            None
        };

        let mut current_block = start_block;
        let mut matched = 0u32;
        let mut mismatched = 0u32;
        let mut rust_errors = 0u32;
        let mut sidecar_errors = 0u32;
        let mut both_errors = 0u32;
        let mut issues: Vec<(u64, String)> = Vec::new();

        while current_block <= end_block {
            let batch_end = std::cmp::min(current_block + batch_size, end_block + 1);
            let blocks: Vec<u32> = (current_block..batch_end).collect();

            if current_block % 1000 == 0 || current_block == start_block {
                println!(
                    "  Processing blocks {} to {}...",
                    current_block,
                    batch_end - 1
                );
            }

            let mut tasks = Vec::new();
            for block_num in blocks {
                let client_clone = client.clone();
                let rust_path = endpoint_type.path(Some(pallet.name), Some(block_num));
                let sidecar_path = endpoint_type.path(Some(pallet.name), Some(block_num));
                let rust_api_url = format!("{}{}", rust_url, rust_path);
                let sidecar_api_url = format!("{}{}", sidecar_url, sidecar_path);

                println!(
                    "  Block {}: {} vs {}",
                    block_num, rust_api_url, sidecar_api_url
                );

                tasks.push(tokio::spawn(async move {
                    test_block_compare(
                        client_clone,
                        rust_api_url,
                        sidecar_api_url,
                        block_num as u64,
                    )
                    .await
                }));
            }

            for task in tasks {
                let (block_id, result) = task.await?;
                log_result_inline(&format!("Block {}", block_id), &result);
                process_result(
                    block_id,
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
        if create_logs && has_issues {
            println!("  Issues saved to: {}", error_filename);
        } else if create_logs {
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
        let endpoint_coverage = chain_coverage.get_endpoint(&endpoint_type.to_string(), true);
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
    print_pallet_summary(
        &pallet_results,
        chain,
        endpoint_type,
        start_block,
        end_block,
        create_logs,
    );

    // Write markdown mismatch report (only with --report flag)
    if create_report {
        write_pallet_mismatch_report(
            &pallet_results,
            chain,
            endpoint_type,
            start_block,
            end_block,
            rust_url,
            sidecar_url,
        );
    }

    Ok(())
}

/// Scan block-based endpoints (iterates over blocks only)
pub async fn scan_block_endpoint(
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
    create_logs: bool,
    create_report: bool,
) -> Result<(), Box<dyn Error>> {
    println!("\n{}", "=".repeat(60));
    println!("Scanning endpoint: {}", endpoint_type);
    println!("{}", "=".repeat(60));

    // Create error log file (only if --logs flag is set)
    let error_filename = format!(
        "errors_{}_{}-{}_{}.log",
        chain, start_block, end_block, endpoint_type
    );
    let mut error_file: Option<File> = if create_logs {
        let mut f = File::create(&error_filename)?;
        writeln!(
            f,
            "# Error/Mismatch log for chain: {}, endpoint: {}",
            chain, endpoint_type
        )?;
        writeln!(f, "# Block range: {} - {}", start_block, end_block)?;
        writeln!(f, "# Rust API: {}", rust_url)?;
        writeln!(f, "# Sidecar API: {}", sidecar_url)?;
        writeln!(f, "#")?;
        Some(f)
    } else {
        None
    };

    let mut current_block = start_block;
    let mut matched = 0u32;
    let mut mismatched = 0u32;
    let mut rust_errors = 0u32;
    let mut sidecar_errors = 0u32;
    let mut both_errors = 0u32;
    let mut issues: Vec<(u64, String)> = Vec::new();

    // Check if this is a special extrinsic index endpoint that needs extrinsic iteration
    let is_extrinsic_idx_endpoint = matches!(
        endpoint_type,
        EndpointType::RcBlockExtrinsicsIdx
            | EndpointType::BlockExtrinsicsIdx
            | EndpointType::BlockExtrinsicsIdxRcBlock
    );

    while current_block <= end_block {
        let batch_end = std::cmp::min(current_block + batch_size, end_block + 1);
        let blocks: Vec<u32> = (current_block..batch_end).collect();

        if current_block % 1000 == 0 || current_block == start_block {
            println!(
                "  Processing blocks {} to {}...",
                current_block,
                batch_end - 1
            );
        }

        let mut tasks = Vec::new();

        if is_extrinsic_idx_endpoint {
            // Special handling: fetch extrinsics count first, then test each index
            for block_num in blocks {
                // Fetch extrinsics list to get count - use appropriate URL based on endpoint type
                let extrinsics_url = match endpoint_type {
                    // /blocks/{blockId}/extrinsics/{index}
                    EndpointType::BlockExtrinsicsIdx => {
                        format!("{}/blocks/{}/extrinsics-raw", rust_url, block_num)
                    }
                    // /blocks/{blockId}/extrinsics/{index}?useRcBlock=true
                    EndpointType::BlockExtrinsicsIdxRcBlock => {
                        format!(
                            "{}/blocks/{}/extrinsics-raw?useRcBlock=true",
                            rust_url, block_num
                        )
                    }
                    // /rc/blocks/{blockId}/extrinsics/{index}
                    _ => {
                        format!("{}/rc/blocks/{}/extrinsics-raw", rust_url, block_num)
                    }
                };
                let extrinsics_count = match fetch_json(client, &extrinsics_url).await {
                    Ok(json) => {
                        // Response structure may vary - try "extrinsics" field first, then check for array at root
                        if let Some(arr) = json.get("extrinsics").and_then(|v| v.as_array()) {
                            arr.len()
                        } else if let Some(arr) = json.as_array() {
                            // Response might be a direct array
                            arr.len()
                        } else {
                            // Debug: print the response keys to understand structure
                            let keys: Vec<&str> = json
                                .as_object()
                                .map(|obj| obj.keys().map(|k| k.as_str()).collect())
                                .unwrap_or_default();
                            println!(
                                "    Block {}: Failed to parse extrinsics from response (keys: {:?}), skipping",
                                block_num, keys
                            );
                            continue;
                        }
                    }
                    Err(e) => {
                        println!(
                            "    Block {}: Failed to fetch extrinsics: {}, skipping",
                            block_num, e
                        );
                        rust_errors += 1;
                        issues.push((
                            block_num as u64,
                            format!("Failed to fetch extrinsics: {}", e),
                        ));
                        continue;
                    }
                };

                println!(
                    "    Block {}: Found {} extrinsics",
                    block_num, extrinsics_count
                );

                // Create tasks for each extrinsic index
                for ext_idx in 0..extrinsics_count {
                    let client_clone = client.clone();
                    let rust_path = endpoint_type.path_with_extrinsic(
                        pallet_filter,
                        Some(block_num),
                        None,
                        Some(ext_idx as u32),
                    );
                    let sidecar_path = endpoint_type.path_with_extrinsic(
                        pallet_filter,
                        Some(block_num),
                        None,
                        Some(ext_idx as u32),
                    );
                    let rust_api_url = format!("{}{}", rust_url, rust_path);
                    let sidecar_api_url = format!("{}{}", sidecar_url, sidecar_path);

                    // Use a composite identifier: block_num * 10000 + ext_idx for tracking
                    // Use u64 to avoid overflow with large block numbers (e.g., 1,000,000 * 10000)
                    let composite_id = block_num as u64 * 10000 + ext_idx as u64;

                    tasks.push(tokio::spawn(async move {
                        test_block_compare(
                            client_clone,
                            rust_api_url,
                            sidecar_api_url,
                            composite_id,
                        )
                        .await
                    }));
                }
            }
        } else {
            // Standard handling for other endpoints
            for block_num in blocks {
                let client_clone = client.clone();
                let rust_path = endpoint_type.path(pallet_filter, Some(block_num));
                let sidecar_path = endpoint_type.path(pallet_filter, Some(block_num));
                let rust_api_url = format!("{}{}", rust_url, rust_path);
                let sidecar_api_url = format!("{}{}", sidecar_url, sidecar_path);

                tasks.push(tokio::spawn(async move {
                    test_block_compare(
                        client_clone,
                        rust_api_url,
                        sidecar_api_url,
                        block_num as u64,
                    )
                    .await
                }));
            }
        }

        for task in tasks {
            let (id, result) = task.await?;
            // For extrinsic endpoints, decode the composite ID for better logging
            let display_id = if is_extrinsic_idx_endpoint {
                let block = id / 10000;
                let ext_idx = id % 10000;
                format!("Block {} Ext {}", block, ext_idx)
            } else {
                format!("Block {}", id)
            };

            log_result_inline(&display_id, &result);

            process_result(
                id,
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
    if create_logs && has_issues {
        println!("Issues saved to: {}", error_filename);
    } else if create_logs {
        std::fs::remove_file(&error_filename).ok();
    }

    // Record coverage
    let chain_coverage = coverage.get_chain(&chain.to_string(), total_pallets);
    let endpoint_coverage = chain_coverage.get_endpoint(&endpoint_type.to_string(), false);
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
    print_block_summary(
        endpoint_type,
        chain,
        start_block,
        end_block,
        matched,
        mismatched,
        rust_errors,
        sidecar_errors,
        both_errors,
        &issues,
        create_logs,
    );

    // Write markdown mismatch report (only with --report flag)
    if create_report {
        write_block_mismatch_report(
            endpoint_type,
            chain,
            start_block,
            end_block,
            rust_url,
            sidecar_url,
            matched,
            mismatched,
            rust_errors,
            sidecar_errors,
            both_errors,
            &issues,
        );
    }

    Ok(())
}

/// Scan runtime endpoints (single request, no iteration)
pub async fn scan_runtime_endpoint(
    client: &reqwest::Client,
    chain: &Chain,
    endpoint_type: &EndpointType,
    rust_url: &str,
    sidecar_url: &str,
    coverage: &mut CoverageData,
    total_pallets: usize,
    create_logs: bool,
) -> Result<(), Box<dyn Error>> {
    // Create summary log file (only if --logs flag is set)
    let summary_filename = format!("summary_{}_{}.log", chain, endpoint_type);
    let mut summary_file: Option<File> = if create_logs {
        Some(File::create(&summary_filename)?)
    } else {
        None
    };

    // Helper macro to print to both console and optionally file
    macro_rules! log_line {
        ($($arg:tt)*) => {
            println!($($arg)*);
            if let Some(ref mut f) = summary_file {
                writeln!(f, $($arg)*).ok();
            }
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

    let (_, result) = test_block_compare(
        client.clone(),
        rust_api_url.clone(),
        sidecar_api_url.clone(),
        0,
    )
    .await;

    // Track coverage result
    let chain_coverage = coverage.get_chain(&chain.to_string(), total_pallets);
    let endpoint_coverage = chain_coverage.get_endpoint(&endpoint_type.to_string(), false);

    match result {
        TestResult::Match => {
            log_line!("\n  Result: MATCH - Both APIs returned identical responses");
            endpoint_coverage.add_runtime_run(true, None);
        }
        TestResult::Mismatch {
            rust_response,
            sidecar_response,
            diffs,
        } => {
            log_line!("\n  Result: MISMATCH - {} difference(s) found", diffs.len());
            // Show first few diffs in console
            for (i, diff) in diffs.iter().take(5).enumerate() {
                log_line!("    {}. {}", i + 1, diff);
            }
            if diffs.len() > 5 {
                log_line!("    ... and {} more", diffs.len() - 5);
            }
            endpoint_coverage.add_runtime_run(false, None);

            if create_logs {
                let error_filename = format!("errors_{}_{}.log", chain, endpoint_type);
                let mut error_file = File::create(&error_filename)?;
                writeln!(
                    error_file,
                    "# Mismatch log for chain: {}, endpoint: {}",
                    chain, endpoint_type
                )?;
                writeln!(error_file, "# Rust API: {}", rust_api_url)?;
                writeln!(error_file, "# Sidecar API: {}", sidecar_api_url)?;
                writeln!(error_file, "#")?;
                writeln!(
                    error_file,
                    "MISMATCH - {} difference(s) found:",
                    diffs.len()
                )?;
                for diff in &diffs {
                    writeln!(error_file, "  - {}", diff)?;
                }
                writeln!(error_file)?;
                writeln!(
                    error_file,
                    "Rust API response: {}",
                    serde_json::to_string_pretty(&rust_response)?
                )?;
                writeln!(
                    error_file,
                    "Sidecar response: {}",
                    serde_json::to_string_pretty(&sidecar_response)?
                )?;

                log_line!("  Details saved to: {}", error_filename);
            }
        }
        TestResult::RustError(ref err) => {
            log_line!("\n  Result: RUST API ERROR - {}", err);
            endpoint_coverage.add_runtime_run(false, Some(err));
        }
        TestResult::SidecarError(ref err) => {
            log_line!("\n  Result: SIDECAR ERROR - {}", err);
            endpoint_coverage.add_runtime_run(false, Some(err));
        }
        TestResult::BothError {
            ref rust_error,
            ref sidecar_error,
        } => {
            log_line!("\n  Result: BOTH APIS ERROR");
            log_line!("    Rust: {}", rust_error);
            log_line!("    Sidecar: {}", sidecar_error);
            endpoint_coverage.add_runtime_run(false, Some(rust_error));
        }
    }

    if create_logs {
        println!("\nSummary saved to: {}", summary_filename);
    }

    Ok(())
}

/// Scan account-based endpoints (iterates over accounts and blocks)
pub async fn scan_account_endpoint(
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
    create_logs: bool,
    create_report: bool,
) -> Result<(), Box<dyn Error>> {
    // Get test accounts for the selected chain (use stash accounts for staking endpoints)
    let accounts = if endpoint_type.is_staking() {
        if !chain.has_staking_accounts() {
            eprintln!(
                "\nWARNING: No staking/stash accounts defined for chain '{}'. \
                Staking endpoints require stash accounts to return meaningful results.",
                chain
            );
            eprintln!(
                "Falling back to regular test accounts. Add staking accounts in chains.rs for '{}'.",
                chain
            );
            eprint!("Continue with regular accounts? [y/N] ");
            use std::io::{self, BufRead};
            let mut input = String::new();
            io::stdin().lock().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("Aborted.");
                return Ok(());
            }
        }
        chain.staking_test_accounts()
    } else {
        chain.test_accounts()
    };

    if accounts.is_empty() {
        println!("No test accounts configured for chain '{}'", chain);
        return Ok(());
    }

    println!("Test accounts to scan: {}", accounts.len());
    for acc in accounts {
        println!("  - {} ({})", acc.label, acc.address);
    }

    // Track results per account
    let mut account_results: Vec<AccountResult> = Vec::new();

    for account in accounts {
        println!("\n{}", "=".repeat(60));
        println!(
            "Scanning account: {} ({}) - {}",
            account.label, account.address, endpoint_type
        );
        println!("{}", "=".repeat(60));

        // Create error log file (only if --logs flag is set)
        let error_filename = format!(
            "errors_{}_{}-{}_{}_account_{}.log",
            chain,
            start_block,
            end_block,
            endpoint_type,
            account.label.replace(" ", "_")
        );
        let mut error_file: Option<File> = if create_logs {
            let mut f = File::create(&error_filename)?;
            writeln!(
                f,
                "# Error/Mismatch log for chain: {}, endpoint: {}, account: {} ({})",
                chain, endpoint_type, account.label, account.address
            )?;
            writeln!(f, "# Block range: {} - {}", start_block, end_block)?;
            writeln!(f, "# Rust API: {}", rust_url)?;
            writeln!(f, "# Sidecar API: {}", sidecar_url)?;
            writeln!(f, "#")?;
            Some(f)
        } else {
            None
        };

        let mut current_block = start_block;
        let mut matched = 0u32;
        let mut mismatched = 0u32;
        let mut rust_errors = 0u32;
        let mut sidecar_errors = 0u32;
        let mut both_errors = 0u32;
        let mut issues: Vec<(u64, String)> = Vec::new();

        while current_block <= end_block {
            let batch_end = std::cmp::min(current_block + batch_size, end_block + 1);
            let blocks: Vec<u32> = (current_block..batch_end).collect();

            if current_block % 1000 == 0 || current_block == start_block {
                println!(
                    "  Processing blocks {} to {}...",
                    current_block,
                    batch_end - 1
                );
            }

            let mut tasks = Vec::new();
            for block_num in blocks {
                let client_clone = client.clone();
                let rust_path =
                    endpoint_type.path_with_account(None, Some(block_num), Some(account.address));
                let sidecar_path =
                    endpoint_type.path_with_account(None, Some(block_num), Some(account.address));
                let rust_api_url = format!("{}{}", rust_url, rust_path);
                let sidecar_api_url = format!("{}{}", sidecar_url, sidecar_path);

                println!(
                    "  Block {}: {} vs {}",
                    block_num, rust_api_url, sidecar_api_url
                );

                tasks.push(tokio::spawn(async move {
                    test_block_compare(
                        client_clone,
                        rust_api_url,
                        sidecar_api_url,
                        block_num as u64,
                    )
                    .await
                }));
            }

            for task in tasks {
                let (block_id, result) = task.await?;
                log_result_inline(&format!("Block {}", block_id), &result);
                process_result(
                    block_id,
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
            account.label, matched, total, match_rate, mismatched, rust_errors, sidecar_errors, both_errors
        );

        let has_issues = mismatched > 0 || rust_errors > 0 || sidecar_errors > 0 || both_errors > 0;
        if create_logs && has_issues {
            println!("  Issues saved to: {}", error_filename);
        } else if create_logs {
            std::fs::remove_file(&error_filename).ok();
        }

        account_results.push(AccountResult {
            label: account.label.to_string(),
            address: account.address.to_string(),
            matched,
            mismatched,
            rust_errors,
            sidecar_errors,
            both_errors,
            issues,
        });

        // Record coverage for this account
        let chain_coverage = coverage.get_chain(&chain.to_string(), total_pallets);
        let endpoint_coverage = chain_coverage.get_endpoint(&endpoint_type.to_string(), false);
        endpoint_coverage.add_account_run(
            account.address,
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
    print_account_summary(
        &account_results,
        chain,
        endpoint_type,
        start_block,
        end_block,
        create_logs,
    );

    // Write markdown mismatch report (only with --report flag)
    if create_report {
        write_account_mismatch_report(
            &account_results,
            chain,
            endpoint_type,
            start_block,
            end_block,
            rust_url,
            sidecar_url,
        );
    }

    Ok(())
}

/// Process a test result and update counters
fn process_result(
    block_num: u64,
    result: TestResult,
    matched: &mut u32,
    mismatched: &mut u32,
    rust_errors: &mut u32,
    sidecar_errors: &mut u32,
    both_errors: &mut u32,
    issues: &mut Vec<(u64, String)>,
    error_file: &mut Option<File>,
) -> Result<(), Box<dyn Error>> {
    match result {
        TestResult::Match => {
            *matched += 1;
        }
        TestResult::Mismatch {
            rust_response,
            sidecar_response,
            diffs,
        } => {
            *mismatched += 1;
            // Create a summary of the differences
            let diff_summary = if diffs.is_empty() {
                "unknown differences".to_string()
            } else if diffs.len() == 1 {
                format!("1 difference: {}", diffs[0])
            } else if diffs.len() <= 5 {
                format!(
                    "{} differences:\n    - {}",
                    diffs.len(),
                    diffs
                        .iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<_>>()
                        .join("\n    - ")
                )
            } else {
                format!(
                    "{} differences (showing first 10):\n    - {}",
                    diffs.len(),
                    diffs
                        .iter()
                        .take(10)
                        .map(|d| d.to_string())
                        .collect::<Vec<_>>()
                        .join("\n    - ")
                )
            };
            let msg = format!("MISMATCH - {}", diff_summary);
            if let Some(ref mut f) = error_file {
                writeln!(f, "Block {}: MISMATCH", block_num)?;
                writeln!(f, "  Differences ({}):", diffs.len())?;
                for diff in &diffs {
                    writeln!(f, "    - {}", diff)?;
                }
                writeln!(f)?;
                writeln!(
                    f,
                    "  Rust API response: {}",
                    serde_json::to_string_pretty(&rust_response)?
                )?;
                writeln!(
                    f,
                    "  Sidecar response: {}",
                    serde_json::to_string_pretty(&sidecar_response)?
                )?;
                writeln!(f)?;
            }
            issues.push((block_num, msg));
        }
        TestResult::RustError(err) => {
            *rust_errors += 1;
            let msg = format!("RUST API ERROR: {}", err);
            if let Some(ref mut f) = error_file {
                writeln!(f, "Block {}: {}", block_num, msg)?;
            }
            issues.push((block_num, msg));
        }
        TestResult::SidecarError(err) => {
            *sidecar_errors += 1;
            let msg = format!("SIDECAR ERROR: {}", err);
            if let Some(ref mut f) = error_file {
                writeln!(f, "Block {}: {}", block_num, msg)?;
            }
            issues.push((block_num, msg));
        }
        TestResult::BothError {
            rust_error,
            sidecar_error,
        } => {
            // Both APIs erroring counts as a match in summary
            *matched += 1;
            // Only record in details when error codes differ
            if rust_error != sidecar_error {
                *both_errors += 1;
                let msg = format!(
                    "BOTH ERRORS (different codes) - Rust: {}, Sidecar: {}",
                    rust_error, sidecar_error
                );
                if let Some(ref mut f) = error_file {
                    writeln!(f, "Block {}: {}", block_num, msg)?;
                }
                issues.push((block_num, msg));
            }
        }
    }
    Ok(())
}
