use clap::Parser;
use std::error::Error;
use std::path::Path;
use std::time::Duration;

mod chains;
mod coverage;
mod diff;
mod endpoints;
mod http;
mod memory;
mod reporting;
mod scanner;

use chains::Chain;
use coverage::CoverageData;
use endpoints::EndpointType;
use http::get_latest_block;
use scanner::{
    scan_account_endpoint, scan_block_endpoint, scan_pallet_endpoint, scan_runtime_endpoint,
};

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
    #[arg(long, default_value = "reports/coverage.json")]
    coverage_file: String,

    /// Show coverage report and exit
    #[arg(long)]
    coverage_report: bool,

    /// Create detailed log files for errors and summaries
    #[arg(long)]
    logs: bool,

    /// Generate markdown mismatch report files (report_*.md)
    #[arg(long)]
    report: bool,

    /// Monitor memory consumption of both server processes
    #[arg(long)]
    memory: bool,

    /// Memory sampling interval in milliseconds (default: 1000)
    #[arg(long, default_value_t = 1000)]
    memory_interval: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Capture the original command for the memory report
    let cli_command: String = std::env::args().collect::<Vec<_>>().join(" ");

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

    // Start memory monitoring if --memory flag is set
    let memory_monitor = if args.memory {
        println!("Memory monitoring enabled (interval: {}ms)", args.memory_interval);
        memory::MemoryMonitor::start(rust_url, sidecar_url, args.memory_interval).await
    } else {
        None
    };

    // Get total pallets for this chain (for coverage tracking)
    let total_pallets = chain.pallets().len();

    // Route to appropriate scanning function based on endpoint type
    if endpoint_type.requires_account() {
        scan_account_endpoint(
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
            args.logs,
            args.report,
        )
        .await?;
    } else if endpoint_type.requires_pallet() {
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
            args.logs,
            args.report,
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
            args.pallet.as_deref(),
            &mut coverage,
            total_pallets,
            args.logs,
            args.report,
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
            args.logs,
        )
        .await?;
    }

    // Stop memory monitoring and print report
    if let Some(monitor) = memory_monitor {
        let memory_report = monitor.stop().await;
        memory_report.print_summary();

        // Append memory report to MEMORY.md
        let mem_filename = "reports/MEMORY.md";
        let is_new = !Path::new(mem_filename).exists();
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(mem_filename)
        {
            use std::io::Write;
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            if is_new {
                writeln!(f, "# Memory Consumption Report\n")?;
            }
            writeln!(f, "---\n")?;
            writeln!(f, "## {} — {} `{}`\n", timestamp, chain, endpoint_type)?;
            writeln!(f, "- **Chain**: {}", chain)?;
            writeln!(f, "- **Endpoint**: `{}`", endpoint_type)?;
            writeln!(f, "- **Path**: `{}`", endpoint_type.path_pattern())?;
            if endpoint_type.requires_block() {
                writeln!(f, "- **Block range**: {} - {}", start_block, end_block)?;
            }
            writeln!(f, "- **Rust API**: {}", rust_url)?;
            writeln!(f, "- **Sidecar API**: {}", sidecar_url)?;
            writeln!(f, "\n### Command\n")?;
            writeln!(f, "```bash\n{}\n```\n", cli_command)?;
            write!(f, "{}", memory_report.to_markdown())?;
            println!("Memory report appended to: {}", mem_filename);
        }
    }

    // Save coverage data
    coverage.save(coverage_path)?;
    println!("Coverage data saved to: {}", args.coverage_file);

    // Save markdown reports (summary + details)
    let markdown_path = Path::new("reports/COVERAGE_SUMMARY.md");
    coverage.save_markdown_report(markdown_path)?;
    println!("Coverage reports saved to: reports/COVERAGE_SUMMARY.md + coverage/COVERAGE_DETAILS.md");

    Ok(())
}
