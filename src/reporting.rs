use std::fs::File;
use std::io::Write;

use crate::chains::Chain;
use crate::endpoints::EndpointType;

pub struct PalletResult {
    pub name: String,
    pub index: u8,
    pub matched: u32,
    pub mismatched: u32,
    pub rust_errors: u32,
    pub sidecar_errors: u32,
    pub both_errors: u32,
    pub issues: Vec<(u64, String)>,
}

pub struct AccountResult {
    pub label: String,
    pub address: String,
    pub matched: u32,
    pub mismatched: u32,
    pub rust_errors: u32,
    pub sidecar_errors: u32,
    pub both_errors: u32,
    pub issues: Vec<(u64, String)>,
}

pub fn print_pallet_summary(
    results: &[PalletResult],
    chain: &Chain,
    endpoint_type: &EndpointType,
    start_block: u32,
    end_block: u32,
    create_logs: bool,
) {
    // Create summary log file (only if --logs flag is set)
    let summary_filename = format!(
        "summary_{}_{}-{}_{}.log",
        chain,
        start_block,
        end_block,
        endpoint_type.short_name()
    );
    let mut summary_file = if create_logs {
        File::create(&summary_filename).ok()
    } else {
        None
    };

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
        "Pallet",
        "Matched",
        "Mismatch",
        "RustErr",
        "SidecarErr",
        "BothErr",
        "Rate"
    );
    log_line!("{}", "-".repeat(90));

    let mut total_matched = 0u32;
    let mut total_mismatched = 0u32;
    let mut total_rust_errors = 0u32;
    let mut total_sidecar_errors = 0u32;
    let mut total_both_errors = 0u32;

    for result in results {
        let total = result.matched
            + result.mismatched
            + result.rust_errors
            + result.sidecar_errors
            + result.both_errors;
        let rate = if total > 0 {
            (result.matched as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        log_line!(
            "{:<25} {:>8} {:>10} {:>10} {:>10} {:>10} {:>7.2}%",
            result.name,
            result.matched,
            result.mismatched,
            result.rust_errors,
            result.sidecar_errors,
            result.both_errors,
            rate
        );

        total_matched += result.matched;
        total_mismatched += result.mismatched;
        total_rust_errors += result.rust_errors;
        total_sidecar_errors += result.sidecar_errors;
        total_both_errors += result.both_errors;
    }

    log_line!("{}", "-".repeat(90));
    let overall_total = total_matched
        + total_mismatched
        + total_rust_errors
        + total_sidecar_errors
        + total_both_errors;
    let overall_rate = if overall_total > 0 {
        (total_matched as f64 / overall_total as f64) * 100.0
    } else {
        0.0
    };
    log_line!(
        "{:<25} {:>8} {:>10} {:>10} {:>10} {:>10} {:>7.2}%",
        "TOTAL",
        total_matched,
        total_mismatched,
        total_rust_errors,
        total_sidecar_errors,
        total_both_errors,
        overall_rate
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
                    log_line!(
                        "  ... and {} more issues (see error log file)",
                        result.issues.len() - 10
                    );
                    break;
                }
                log_line!("  Block {}: {}", block, error);
            }
        }
    }

    if create_logs {
        println!("\nSummary saved to: {}", summary_filename);
    }
}

pub fn print_block_summary(
    endpoint_type: &EndpointType,
    chain: &Chain,
    start_block: u32,
    end_block: u32,
    matched: u32,
    mismatched: u32,
    rust_errors: u32,
    sidecar_errors: u32,
    both_errors: u32,
    issues: &[(u64, String)],
    create_logs: bool,
) {
    // Create summary log file (only if --logs flag is set)
    let summary_filename = format!(
        "summary_{}_{}-{}_{}.log",
        chain,
        start_block,
        end_block,
        endpoint_type.short_name()
    );
    let mut summary_file = if create_logs {
        File::create(&summary_filename).ok()
    } else {
        None
    };

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
                log_line!(
                    "  ... and {} more issues (see error log file)",
                    issues.len() - 20
                );
                break;
            }
            log_line!("  Block {}: {}", block, error);
        }
    }

    if create_logs {
        println!("\nSummary saved to: {}", summary_filename);
    }
}

pub fn print_account_summary(
    results: &[AccountResult],
    chain: &Chain,
    endpoint_type: &EndpointType,
    start_block: u32,
    end_block: u32,
    create_logs: bool,
) {
    // Create summary log file (only if --logs flag is set)
    let summary_filename = format!(
        "summary_{}_{}-{}_{}_accounts.log",
        chain,
        start_block,
        end_block,
        endpoint_type.short_name()
    );
    let mut summary_file = if create_logs {
        File::create(&summary_filename).ok()
    } else {
        None
    };

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
    log_line!("Total accounts scanned: {}\n", results.len());

    log_line!(
        "{:<15} {:>8} {:>10} {:>10} {:>10} {:>10} {:>8}",
        "Account",
        "Matched",
        "Mismatch",
        "RustErr",
        "SidecarErr",
        "BothErr",
        "Rate"
    );
    log_line!("{}", "-".repeat(90));

    let mut total_matched = 0u32;
    let mut total_mismatched = 0u32;
    let mut total_rust_errors = 0u32;
    let mut total_sidecar_errors = 0u32;
    let mut total_both_errors = 0u32;

    for result in results {
        let total = result.matched
            + result.mismatched
            + result.rust_errors
            + result.sidecar_errors
            + result.both_errors;
        let rate = if total > 0 {
            (result.matched as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        log_line!(
            "{:<15} {:>8} {:>10} {:>10} {:>10} {:>10} {:>7.2}%",
            result.label,
            result.matched,
            result.mismatched,
            result.rust_errors,
            result.sidecar_errors,
            result.both_errors,
            rate
        );

        total_matched += result.matched;
        total_mismatched += result.mismatched;
        total_rust_errors += result.rust_errors;
        total_sidecar_errors += result.sidecar_errors;
        total_both_errors += result.both_errors;
    }

    log_line!("{}", "-".repeat(90));
    let overall_total = total_matched
        + total_mismatched
        + total_rust_errors
        + total_sidecar_errors
        + total_both_errors;
    let overall_rate = if overall_total > 0 {
        (total_matched as f64 / overall_total as f64) * 100.0
    } else {
        0.0
    };
    log_line!(
        "{:<15} {:>8} {:>10} {:>10} {:>10} {:>10} {:>7.2}%",
        "TOTAL",
        total_matched,
        total_mismatched,
        total_rust_errors,
        total_sidecar_errors,
        total_both_errors,
        overall_rate
    );

    // Print issue summary
    let accounts_with_issues: Vec<_> = results.iter().filter(|r| !r.issues.is_empty()).collect();

    if !accounts_with_issues.is_empty() {
        log_line!("\n\n{}", "=".repeat(90));
        log_line!("                                   ISSUE DETAILS");
        log_line!("{}", "=".repeat(90));

        for result in &accounts_with_issues {
            log_line!("\n{} ({}):", result.label, result.address);
            for (i, (block, error)) in result.issues.iter().enumerate() {
                if i >= 10 {
                    log_line!(
                        "  ... and {} more issues (see error log file)",
                        result.issues.len() - 10
                    );
                    break;
                }
                log_line!("  Block {}: {}", block, error);
            }
        }
    }

    if create_logs {
        println!("\nSummary saved to: {}", summary_filename);
    }
}
