//! Coverage tracking for API endpoint testing.
//!
//! This module tracks which endpoints, pallets, and block ranges have been tested
//! across multiple runs of the checker.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Coverage data for a single endpoint + pallet combination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalletCoverage {
    /// Pallet name
    pub pallet: String,
    /// Block ranges that have been tested (start, end)
    pub block_ranges: Vec<(u32, u32)>,
    /// Total blocks tested
    pub total_blocks_tested: u32,
    /// Number of matched (successful) comparisons
    pub matched: u32,
    /// Number of mismatched comparisons
    pub mismatched: u32,
    /// Number of Rust API errors
    pub rust_errors: u32,
    /// Number of Sidecar API errors
    pub sidecar_errors: u32,
    /// Number of errors from both APIs
    pub both_errors: u32,
    /// Last tested timestamp
    pub last_tested: String,
}

impl PalletCoverage {
    pub fn new(pallet: &str) -> Self {
        Self {
            pallet: pallet.to_string(),
            block_ranges: Vec::new(),
            total_blocks_tested: 0,
            matched: 0,
            mismatched: 0,
            rust_errors: 0,
            sidecar_errors: 0,
            both_errors: 0,
            last_tested: String::new(),
        }
    }

    /// Add a test run result
    pub fn add_run(
        &mut self,
        start_block: u32,
        end_block: u32,
        matched: u32,
        mismatched: u32,
        rust_errors: u32,
        sidecar_errors: u32,
        both_errors: u32,
    ) {
        // Add block range (merge overlapping ranges later if needed)
        self.block_ranges.push((start_block, end_block));
        self.merge_block_ranges();

        // Update totals
        let blocks_in_run = end_block.saturating_sub(start_block) + 1;
        self.total_blocks_tested += blocks_in_run;
        self.matched += matched;
        self.mismatched += mismatched;
        self.rust_errors += rust_errors;
        self.sidecar_errors += sidecar_errors;
        self.both_errors += both_errors;

        // Update timestamp
        self.last_tested = chrono::Utc::now().to_rfc3339();
    }

    /// Merge overlapping block ranges
    fn merge_block_ranges(&mut self) {
        if self.block_ranges.is_empty() {
            return;
        }

        // Sort by start block
        self.block_ranges.sort_by_key(|r| r.0);

        let mut merged = Vec::new();
        let mut current = self.block_ranges[0];

        for &(start, end) in &self.block_ranges[1..] {
            if start <= current.1 + 1 {
                // Overlapping or adjacent, merge
                current.1 = current.1.max(end);
            } else {
                // Not overlapping, push current and start new
                merged.push(current);
                current = (start, end);
            }
        }
        merged.push(current);

        self.block_ranges = merged;
    }

    /// Get pass rate as percentage
    pub fn pass_rate(&self) -> f64 {
        let total = self.matched + self.mismatched + self.rust_errors + self.sidecar_errors + self.both_errors;
        if total == 0 {
            0.0
        } else {
            (self.matched as f64 / total as f64) * 100.0
        }
    }
}

/// Coverage data for a single endpoint (across all pallets)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointCoverage {
    /// Endpoint name
    pub endpoint: String,
    /// Per-pallet coverage (for pallet endpoints) or None for block/runtime endpoints
    pub pallets: Option<HashMap<String, PalletCoverage>>,
    /// Block ranges tested (for block endpoints)
    pub block_ranges: Vec<(u32, u32)>,
    /// Stats for non-pallet endpoints
    pub matched: u32,
    pub mismatched: u32,
    pub rust_errors: u32,
    pub sidecar_errors: u32,
    pub both_errors: u32,
    /// Whether this endpoint has been tested at all
    pub tested: bool,
    /// Last tested timestamp
    pub last_tested: String,
}

impl EndpointCoverage {
    pub fn new(endpoint: &str, is_pallet_endpoint: bool) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            pallets: if is_pallet_endpoint { Some(HashMap::new()) } else { None },
            block_ranges: Vec::new(),
            matched: 0,
            mismatched: 0,
            rust_errors: 0,
            sidecar_errors: 0,
            both_errors: 0,
            tested: false,
            last_tested: String::new(),
        }
    }

    /// Add pallet coverage result
    pub fn add_pallet_run(
        &mut self,
        pallet: &str,
        start_block: u32,
        end_block: u32,
        matched: u32,
        mismatched: u32,
        rust_errors: u32,
        sidecar_errors: u32,
        both_errors: u32,
    ) {
        self.tested = true;
        self.last_tested = chrono::Utc::now().to_rfc3339();

        if let Some(ref mut pallets) = self.pallets {
            let coverage = pallets
                .entry(pallet.to_string())
                .or_insert_with(|| PalletCoverage::new(pallet));
            coverage.add_run(start_block, end_block, matched, mismatched, rust_errors, sidecar_errors, both_errors);
        }
    }

    /// Add block endpoint coverage result
    pub fn add_block_run(
        &mut self,
        start_block: u32,
        end_block: u32,
        matched: u32,
        mismatched: u32,
        rust_errors: u32,
        sidecar_errors: u32,
        both_errors: u32,
    ) {
        self.tested = true;
        self.last_tested = chrono::Utc::now().to_rfc3339();

        self.block_ranges.push((start_block, end_block));
        self.merge_block_ranges();

        self.matched += matched;
        self.mismatched += mismatched;
        self.rust_errors += rust_errors;
        self.sidecar_errors += sidecar_errors;
        self.both_errors += both_errors;
    }

    /// Add runtime endpoint coverage result
    pub fn add_runtime_run(
        &mut self,
        matched: bool,
        error: Option<&str>,
    ) {
        self.tested = true;
        self.last_tested = chrono::Utc::now().to_rfc3339();

        if matched {
            self.matched += 1;
        } else if let Some(_) = error {
            self.rust_errors += 1;
        } else {
            self.mismatched += 1;
        }
    }

    /// Add account endpoint coverage result
    pub fn add_account_run(
        &mut self,
        _account: &str,
        start_block: u32,
        end_block: u32,
        matched: u32,
        mismatched: u32,
        rust_errors: u32,
        sidecar_errors: u32,
        both_errors: u32,
    ) {
        self.tested = true;
        self.last_tested = chrono::Utc::now().to_rfc3339();

        self.block_ranges.push((start_block, end_block));
        self.merge_block_ranges();

        self.matched += matched;
        self.mismatched += mismatched;
        self.rust_errors += rust_errors;
        self.sidecar_errors += sidecar_errors;
        self.both_errors += both_errors;
    }

    /// Merge overlapping block ranges
    fn merge_block_ranges(&mut self) {
        if self.block_ranges.is_empty() {
            return;
        }

        self.block_ranges.sort_by_key(|r| r.0);

        let mut merged = Vec::new();
        let mut current = self.block_ranges[0];

        for &(start, end) in &self.block_ranges[1..] {
            if start <= current.1 + 1 {
                current.1 = current.1.max(end);
            } else {
                merged.push(current);
                current = (start, end);
            }
        }
        merged.push(current);

        self.block_ranges = merged;
    }

    /// Get pass rate
    pub fn pass_rate(&self) -> f64 {
        if let Some(ref pallets) = self.pallets {
            let total_matched: u32 = pallets.values().map(|p| p.matched).sum();
            let total_tests: u32 = pallets.values().map(|p| {
                p.matched + p.mismatched + p.rust_errors + p.sidecar_errors + p.both_errors
            }).sum();
            if total_tests == 0 {
                0.0
            } else {
                (total_matched as f64 / total_tests as f64) * 100.0
            }
        } else {
            let total = self.matched + self.mismatched + self.rust_errors + self.sidecar_errors + self.both_errors;
            if total == 0 {
                0.0
            } else {
                (self.matched as f64 / total as f64) * 100.0
            }
        }
    }
}

/// Coverage data for a chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainCoverage {
    /// Chain name
    pub chain: String,
    /// Total pallets in this chain
    pub total_pallets: usize,
    /// Endpoint coverage
    pub endpoints: HashMap<String, EndpointCoverage>,
    /// Last updated timestamp
    pub last_updated: String,
}

impl ChainCoverage {
    pub fn new(chain: &str, total_pallets: usize) -> Self {
        Self {
            chain: chain.to_string(),
            total_pallets,
            endpoints: HashMap::new(),
            last_updated: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Get or create endpoint coverage
    pub fn get_endpoint(&mut self, endpoint: &str, is_pallet_endpoint: bool) -> &mut EndpointCoverage {
        self.last_updated = chrono::Utc::now().to_rfc3339();
        self.endpoints
            .entry(endpoint.to_string())
            .or_insert_with(|| EndpointCoverage::new(endpoint, is_pallet_endpoint))
    }
}

/// Root coverage data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CoverageData {
    /// Version of the coverage format
    pub version: String,
    /// Coverage per chain
    pub chains: HashMap<String, ChainCoverage>,
}

impl CoverageData {
    pub fn new() -> Self {
        Self {
            version: "1.0".to_string(),
            chains: HashMap::new(),
        }
    }

    /// Load coverage data from file
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let data: CoverageData = serde_json::from_str(&content)?;
            Ok(data)
        } else {
            Ok(Self::new())
        }
    }

    /// Save coverage data to file
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Get or create chain coverage
    pub fn get_chain(&mut self, chain: &str, total_pallets: usize) -> &mut ChainCoverage {
        self.chains
            .entry(chain.to_string())
            .or_insert_with(|| ChainCoverage::new(chain, total_pallets))
    }

    /// Generate coverage report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str(&format!("\n{}\n", "=".repeat(80)));
        report.push_str("                           API COVERAGE REPORT\n");
        report.push_str(&format!("{}\n\n", "=".repeat(80)));

        // All possible endpoints
        let pallet_endpoints = vec!["consts", "consts-item", "storage", "dispatchables", "errors", "events"];
        let block_endpoints = vec!["block", "block-header", "block-extrinsics", "block-extrinsics-raw", "rc-block-extrinsics-raw", "rc-block-extrinsics-idx", "block-para-inclusions", "staking-validators", "rc-staking-validators", "coretime-leases", "coretime-reservations", "coretime-regions"];
        let account_endpoints = vec!["account-balance-info"];
        let standalone_endpoints = vec!["runtime-spec", "runtime-metadata", "tx-material", "node-version", "node-network"];

        for (chain_name, chain) in &self.chains {
            report.push_str(&format!("Chain: {}\n", chain_name));
            report.push_str(&format!("Total pallets: {}\n", chain.total_pallets));
            report.push_str(&format!("{}\n\n", "-".repeat(80)));

            // Pallet endpoints
            report.push_str("PALLET ENDPOINTS:\n");
            for endpoint in &pallet_endpoints {
                if let Some(ep_cov) = chain.endpoints.get(*endpoint) {
                    if ep_cov.tested {
                        let pallets_tested = ep_cov.pallets.as_ref().map(|p| p.len()).unwrap_or(0);
                        let pass_rate = ep_cov.pass_rate();
                        report.push_str(&format!(
                            "  [✓] {:<20} {:>3}/{:<3} pallets tested ({:.1}% pass rate)\n",
                            endpoint, pallets_tested, chain.total_pallets, pass_rate
                        ));

                        // Show block ranges for each pallet
                        if let Some(ref pallets) = ep_cov.pallets {
                            for (pallet_name, pallet_cov) in pallets {
                                let ranges: Vec<String> = pallet_cov.block_ranges.iter()
                                    .map(|(s, e)| format!("{}-{}", s, e))
                                    .collect();
                                report.push_str(&format!(
                                    "      - {}: blocks [{}] ({:.1}% pass)\n",
                                    pallet_name,
                                    ranges.join(", "),
                                    pallet_cov.pass_rate()
                                ));
                            }
                        }
                    } else {
                        report.push_str(&format!("  [ ] {:<20} not tested\n", endpoint));
                    }
                } else {
                    report.push_str(&format!("  [ ] {:<20} not tested\n", endpoint));
                }
            }

            // Block endpoints
            report.push_str("\nBLOCK ENDPOINTS:\n");
            for endpoint in &block_endpoints {
                if let Some(ep_cov) = chain.endpoints.get(*endpoint) {
                    if ep_cov.tested {
                        let ranges: Vec<String> = ep_cov.block_ranges.iter()
                            .map(|(s, e)| format!("{}-{}", s, e))
                            .collect();
                        let pass_rate = ep_cov.pass_rate();
                        report.push_str(&format!(
                            "  [✓] {:<20} blocks [{}] ({:.1}% pass rate)\n",
                            endpoint,
                            if ranges.is_empty() { "none".to_string() } else { ranges.join(", ") },
                            pass_rate
                        ));
                    } else {
                        report.push_str(&format!("  [ ] {:<20} not tested\n", endpoint));
                    }
                } else {
                    report.push_str(&format!("  [ ] {:<20} not tested\n", endpoint));
                }
            }

            // Account endpoints
            report.push_str("\nACCOUNT ENDPOINTS:\n");
            for endpoint in &account_endpoints {
                if let Some(ep_cov) = chain.endpoints.get(*endpoint) {
                    if ep_cov.tested {
                        let ranges: Vec<String> = ep_cov.block_ranges.iter()
                            .map(|(s, e)| format!("{}-{}", s, e))
                            .collect();
                        let pass_rate = ep_cov.pass_rate();
                        report.push_str(&format!(
                            "  [✓] {:<20} blocks [{}] ({:.1}% pass rate)\n",
                            endpoint,
                            if ranges.is_empty() { "none".to_string() } else { ranges.join(", ") },
                            pass_rate
                        ));
                    } else {
                        report.push_str(&format!("  [ ] {:<20} not tested\n", endpoint));
                    }
                } else {
                    report.push_str(&format!("  [ ] {:<20} not tested\n", endpoint));
                }
            }

            // Runtime endpoints
            report.push_str("\nRUNTIME ENDPOINTS:\n");
            for endpoint in &standalone_endpoints {
                if let Some(ep_cov) = chain.endpoints.get(*endpoint) {
                    if ep_cov.tested {
                        let status = if ep_cov.matched > 0 { "PASS" } else { "FAIL" };
                        report.push_str(&format!("  [✓] {:<20} tested ({})\n", endpoint, status));
                    } else {
                        report.push_str(&format!("  [ ] {:<20} not tested\n", endpoint));
                    }
                } else {
                    report.push_str(&format!("  [ ] {:<20} not tested\n", endpoint));
                }
            }

            // Summary
            let total_endpoints = pallet_endpoints.len() + block_endpoints.len() + account_endpoints.len() + standalone_endpoints.len();
            let tested_endpoints = chain.endpoints.values().filter(|e| e.tested).count();

            report.push_str(&format!("\nSUMMARY:\n"));
            report.push_str(&format!("  Endpoints tested: {}/{}\n", tested_endpoints, total_endpoints));

            // Calculate overall pass rate
            let mut total_matched = 0u32;
            let mut total_tests = 0u32;
            for ep in chain.endpoints.values() {
                if let Some(ref pallets) = ep.pallets {
                    for p in pallets.values() {
                        total_matched += p.matched;
                        total_tests += p.matched + p.mismatched + p.rust_errors + p.sidecar_errors + p.both_errors;
                    }
                } else {
                    total_matched += ep.matched;
                    total_tests += ep.matched + ep.mismatched + ep.rust_errors + ep.sidecar_errors + ep.both_errors;
                }
            }
            let overall_pass_rate = if total_tests > 0 {
                (total_matched as f64 / total_tests as f64) * 100.0
            } else {
                0.0
            };
            report.push_str(&format!("  Overall pass rate: {:.2}% ({}/{})\n", overall_pass_rate, total_matched, total_tests));
            report.push_str(&format!("  Last updated: {}\n", chain.last_updated));

            report.push_str(&format!("\n{}\n", "=".repeat(80)));
        }

        if self.chains.is_empty() {
            report.push_str("No coverage data recorded yet.\n");
            report.push_str("Run some tests to start tracking coverage.\n");
        }

        report
    }

    /// Generate markdown coverage report
    pub fn generate_markdown_report(&self) -> String {
        let mut report = String::new();

        report.push_str("# Coverage Tracking\n\n");
        report.push_str("This file is auto-generated from test results. Run tests to update coverage data.\n\n");

        // All possible endpoints
        let pallet_endpoints = vec!["consts", "consts-item", "storage", "dispatchables", "errors", "events"];
        let block_endpoints = vec!["block", "block-header", "block-extrinsics", "block-extrinsics-raw", "rc-block-extrinsics-raw", "rc-block-extrinsics-idx", "block-para-inclusions", "staking-validators", "rc-staking-validators", "coretime-leases", "coretime-reservations", "coretime-regions"];
        let account_endpoints = vec!["account-balance-info"];
        let standalone_endpoints = vec!["runtime-spec", "runtime-metadata", "tx-material", "node-version", "node-network"];

        report.push_str("## How it Works\n\n");
        report.push_str("- Every test run automatically saves results to a coverage file (default: `coverage/coverage.json`)\n");
        report.push_str("- Coverage data accumulates across runs, tracking:\n");
        report.push_str("  - Which endpoints have been tested\n");
        report.push_str("  - Which pallets have been tested for each endpoint\n");
        report.push_str("  - Which block ranges have been covered\n");
        report.push_str("  - Pass/fail rates for each endpoint and pallet\n\n");

        report.push_str("## Viewing Coverage Report\n\n");
        report.push_str("```bash\n");
        report.push_str("# Show coverage report\n");
        report.push_str("cargo run -- --coverage-report\n\n");
        report.push_str("# Use a custom coverage file\n");
        report.push_str("cargo run -- --coverage-file coverage/my-coverage.json --coverage-report\n");
        report.push_str("```\n\n");

        if self.chains.is_empty() {
            report.push_str("## Current Coverage\n\n");
            report.push_str("No coverage data recorded yet. Run some tests to start tracking coverage.\n\n");
            return report;
        }

        report.push_str("## Current Coverage\n\n");

        for (chain_name, chain) in &self.chains {
            report.push_str(&format!("### Chain: {}\n\n", chain_name));
            report.push_str(&format!("- **Total pallets:** {}\n", chain.total_pallets));
            report.push_str(&format!("- **Last updated:** {}\n\n", chain.last_updated));

            // Summary stats
            let total_endpoints = pallet_endpoints.len() + block_endpoints.len() + account_endpoints.len() + standalone_endpoints.len();
            let tested_endpoints = chain.endpoints.values().filter(|e| e.tested).count();

            let mut total_matched = 0u32;
            let mut total_tests = 0u32;
            for ep in chain.endpoints.values() {
                if let Some(ref pallets) = ep.pallets {
                    for p in pallets.values() {
                        total_matched += p.matched;
                        total_tests += p.matched + p.mismatched + p.rust_errors + p.sidecar_errors + p.both_errors;
                    }
                } else {
                    total_matched += ep.matched;
                    total_tests += ep.matched + ep.mismatched + ep.rust_errors + ep.sidecar_errors + ep.both_errors;
                }
            }
            let overall_pass_rate = if total_tests > 0 {
                (total_matched as f64 / total_tests as f64) * 100.0
            } else {
                0.0
            };

            report.push_str("| Metric | Value |\n");
            report.push_str("|--------|-------|\n");
            report.push_str(&format!("| Endpoints tested | {}/{} |\n", tested_endpoints, total_endpoints));
            report.push_str(&format!("| Overall pass rate | {:.2}% ({}/{}) |\n\n", overall_pass_rate, total_matched, total_tests));

            // Pallet endpoints table
            report.push_str("#### Pallet Endpoints\n\n");
            report.push_str("| Endpoint | Status | Pallets Tested | Pass Rate |\n");
            report.push_str("|----------|--------|----------------|------------|\n");
            for endpoint in &pallet_endpoints {
                if let Some(ep_cov) = chain.endpoints.get(*endpoint) {
                    if ep_cov.tested {
                        let pallets_tested = ep_cov.pallets.as_ref().map(|p| p.len()).unwrap_or(0);
                        let pass_rate = ep_cov.pass_rate();
                        report.push_str(&format!(
                            "| {} | ✅ | {}/{} | {:.1}% |\n",
                            endpoint, pallets_tested, chain.total_pallets, pass_rate
                        ));
                    } else {
                        report.push_str(&format!("| {} | ❌ | - | - |\n", endpoint));
                    }
                } else {
                    report.push_str(&format!("| {} | ❌ | - | - |\n", endpoint));
                }
            }
            report.push_str("\n");

            // Block endpoints table
            report.push_str("#### Block Endpoints\n\n");
            report.push_str("| Endpoint | Status | Block Ranges | Pass Rate |\n");
            report.push_str("|----------|--------|--------------|------------|\n");
            for endpoint in &block_endpoints {
                if let Some(ep_cov) = chain.endpoints.get(*endpoint) {
                    if ep_cov.tested {
                        let ranges: Vec<String> = ep_cov.block_ranges.iter()
                            .map(|(s, e)| format!("{}-{}", s, e))
                            .collect();
                        let pass_rate = ep_cov.pass_rate();
                        report.push_str(&format!(
                            "| {} | ✅ | {} | {:.1}% |\n",
                            endpoint,
                            if ranges.is_empty() { "none".to_string() } else { ranges.join(", ") },
                            pass_rate
                        ));
                    } else {
                        report.push_str(&format!("| {} | ❌ | - | - |\n", endpoint));
                    }
                } else {
                    report.push_str(&format!("| {} | ❌ | - | - |\n", endpoint));
                }
            }
            report.push_str("\n");

            // Account endpoints table
            report.push_str("#### Account Endpoints\n\n");
            report.push_str("| Endpoint | Status | Block Ranges | Pass Rate |\n");
            report.push_str("|----------|--------|--------------|------------|\n");
            for endpoint in &account_endpoints {
                if let Some(ep_cov) = chain.endpoints.get(*endpoint) {
                    if ep_cov.tested {
                        let ranges: Vec<String> = ep_cov.block_ranges.iter()
                            .map(|(s, e)| format!("{}-{}", s, e))
                            .collect();
                        let pass_rate = ep_cov.pass_rate();
                        report.push_str(&format!(
                            "| {} | ✅ | {} | {:.1}% |\n",
                            endpoint,
                            if ranges.is_empty() { "none".to_string() } else { ranges.join(", ") },
                            pass_rate
                        ));
                    } else {
                        report.push_str(&format!("| {} | ❌ | - | - |\n", endpoint));
                    }
                } else {
                    report.push_str(&format!("| {} | ❌ | - | - |\n", endpoint));
                }
            }
            report.push_str("\n");

            // Standalone endpoints table
            report.push_str("#### Standalone Endpoints\n\n");
            report.push_str("| Endpoint | Status | Result |\n");
            report.push_str("|----------|--------|--------|\n");
            for endpoint in &standalone_endpoints {
                if let Some(ep_cov) = chain.endpoints.get(*endpoint) {
                    if ep_cov.tested {
                        let status = if ep_cov.matched > 0 { "PASS" } else { "FAIL" };
                        report.push_str(&format!("| {} | ✅ | {} |\n", endpoint, status));
                    } else {
                        report.push_str(&format!("| {} | ❌ | - |\n", endpoint));
                    }
                } else {
                    report.push_str(&format!("| {} | ❌ | - |\n", endpoint));
                }
            }
            report.push_str("\n");

            // Detailed pallet coverage if available
            let has_pallet_details = chain.endpoints.values()
                .filter(|e| e.tested && e.pallets.is_some())
                .any(|e| e.pallets.as_ref().map(|p| !p.is_empty()).unwrap_or(false));

            if has_pallet_details {
                report.push_str("#### Detailed Pallet Coverage\n\n");

                for endpoint in &pallet_endpoints {
                    if let Some(ep_cov) = chain.endpoints.get(*endpoint) {
                        if ep_cov.tested {
                            if let Some(ref pallets) = ep_cov.pallets {
                                if !pallets.is_empty() {
                                    report.push_str(&format!("**{}:**\n\n", endpoint));
                                    report.push_str("| Pallet | Block Ranges | Matched | Mismatched | Errors | Pass Rate |\n");
                                    report.push_str("|--------|--------------|---------|------------|--------|------------|\n");

                                    let mut sorted_pallets: Vec<_> = pallets.iter().collect();
                                    sorted_pallets.sort_by(|a, b| a.0.cmp(b.0));

                                    for (pallet_name, pallet_cov) in sorted_pallets {
                                        let ranges: Vec<String> = pallet_cov.block_ranges.iter()
                                            .map(|(s, e)| format!("{}-{}", s, e))
                                            .collect();
                                        let total_errors = pallet_cov.rust_errors + pallet_cov.sidecar_errors + pallet_cov.both_errors;
                                        report.push_str(&format!(
                                            "| {} | {} | {} | {} | {} | {:.1}% |\n",
                                            pallet_name,
                                            ranges.join(", "),
                                            pallet_cov.matched,
                                            pallet_cov.mismatched,
                                            total_errors,
                                            pallet_cov.pass_rate()
                                        ));
                                    }
                                    report.push_str("\n");
                                }
                            }
                        }
                    }
                }
            }
        }

        report.push_str("## Coverage File Format\n\n");
        report.push_str("Coverage data is stored in JSON format (`coverage/coverage.json`) and can be analyzed programmatically.\n");

        report
    }

    /// Save markdown report to file
    pub fn save_markdown_report(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = self.generate_markdown_report();
        fs::write(path, content)?;
        Ok(())
    }
}
