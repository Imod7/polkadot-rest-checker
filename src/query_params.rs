//! Query parameter definitions for testing API endpoints with optional flags.
//!
//! Each `QueryParam` maps to a URL query parameter supported by polkadot-rest-api.
//! The `supported_query_params()` method on `EndpointType` declares which params
//! each endpoint accepts, and `append_query_params()` appends them to URL paths.

use std::fmt;
use std::fs;
use std::path::Path;

use crate::endpoints::EndpointType;

/// Query parameters that can be appended to API requests for testing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryParam {
    /// eventDocs=true — include documentation for events
    EventDocs,
    /// extrinsicDocs=true — include documentation for extrinsics
    ExtrinsicDocs,
    /// noFees=true — skip fee calculation
    NoFees,
    /// onlyIds=true — return only item names (pallet list endpoints)
    OnlyIds,
    /// metadata=true — include full metadata (pallet item endpoints)
    Metadata,
    /// finalized=false — return non-finalized head
    Finalized,
    /// decodedXcmMsgs=true — decode XCM messages
    DecodedXcmMsgs,
    /// paraId=<id> — filter XCM messages by parachain ID
    ParaId(u32),
    /// denominated=true — denominate balances using chain decimals
    Denominated,
    /// includeFee=true — include fee details in transaction pool
    IncludeFee,
    /// includeClaimedRewards=true — include claimed rewards in staking info
    IncludeClaimedRewards,
    /// useEvmFormat=true — convert AccountId32 to EVM format
    UseEvmFormat,
    /// noMeta=true — exclude metadata from transaction material
    NoMeta,
    /// finalizedKey=false — exclude finalized status from block response
    FinalizedKey,
    /// useRcBlock=true — treat blockId as Relay Chain block and return Asset Hub blocks
    UseRcBlock,
}

impl QueryParam {
    /// Convert to URL query string fragment (e.g. "eventDocs=true").
    pub fn to_query_string(&self) -> String {
        match self {
            QueryParam::EventDocs => "eventDocs=true".to_string(),
            QueryParam::ExtrinsicDocs => "extrinsicDocs=true".to_string(),
            QueryParam::NoFees => "noFees=true".to_string(),
            QueryParam::OnlyIds => "onlyIds=true".to_string(),
            QueryParam::Metadata => "metadata=true".to_string(),
            QueryParam::Finalized => "finalized=false".to_string(),
            QueryParam::DecodedXcmMsgs => "decodedXcmMsgs=true".to_string(),
            QueryParam::ParaId(id) => format!("paraId={}", id),
            QueryParam::Denominated => "denominated=true".to_string(),
            QueryParam::IncludeFee => "includeFee=true".to_string(),
            QueryParam::IncludeClaimedRewards => "includeClaimedRewards=true".to_string(),
            QueryParam::UseEvmFormat => "useEvmFormat=true".to_string(),
            QueryParam::NoMeta => "noMeta=true".to_string(),
            QueryParam::FinalizedKey => "finalizedKey=false".to_string(),
            QueryParam::UseRcBlock => "useRcBlock=true".to_string(),
        }
    }

    /// Canonical name used for CLI parsing and display.
    pub fn name(&self) -> &'static str {
        match self {
            QueryParam::EventDocs => "eventDocs",
            QueryParam::ExtrinsicDocs => "extrinsicDocs",
            QueryParam::NoFees => "noFees",
            QueryParam::OnlyIds => "onlyIds",
            QueryParam::Metadata => "metadata",
            QueryParam::Finalized => "finalized",
            QueryParam::DecodedXcmMsgs => "decodedXcmMsgs",
            QueryParam::ParaId(_) => "paraId",
            QueryParam::Denominated => "denominated",
            QueryParam::IncludeFee => "includeFee",
            QueryParam::IncludeClaimedRewards => "includeClaimedRewards",
            QueryParam::UseEvmFormat => "useEvmFormat",
            QueryParam::NoMeta => "noMeta",
            QueryParam::FinalizedKey => "finalizedKey",
            QueryParam::UseRcBlock => "useRcBlock",
        }
    }

    /// All known query param variants (with default values for parameterized ones).
    pub fn all_variants() -> Vec<QueryParam> {
        vec![
            QueryParam::EventDocs,
            QueryParam::ExtrinsicDocs,
            QueryParam::NoFees,
            QueryParam::OnlyIds,
            QueryParam::Metadata,
            QueryParam::Finalized,
            QueryParam::DecodedXcmMsgs,
            QueryParam::ParaId(1000),
            QueryParam::Denominated,
            QueryParam::IncludeFee,
            QueryParam::IncludeClaimedRewards,
            QueryParam::UseEvmFormat,
            QueryParam::NoMeta,
            QueryParam::FinalizedKey,
            QueryParam::UseRcBlock,
        ]
    }
}

impl fmt::Display for QueryParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryParam::ParaId(id) => write!(f, "paraId={}", id),
            other => f.write_str(other.name()),
        }
    }
}

impl std::str::FromStr for QueryParam {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        // Handle paraId=NNN format
        if lower.starts_with("paraid=") {
            let val = &s[7..];
            let id: u32 = val
                .parse()
                .map_err(|_| format!("Invalid paraId value '{}', expected a number", val))?;
            return Ok(QueryParam::ParaId(id));
        }
        match lower.as_str() {
            "eventdocs" => Ok(QueryParam::EventDocs),
            "extrinsicdocs" => Ok(QueryParam::ExtrinsicDocs),
            "nofees" => Ok(QueryParam::NoFees),
            "onlyids" => Ok(QueryParam::OnlyIds),
            "metadata" => Ok(QueryParam::Metadata),
            "finalized" => Ok(QueryParam::Finalized),
            "decodedxcmmsgs" => Ok(QueryParam::DecodedXcmMsgs),
            "paraid" => Ok(QueryParam::ParaId(1000)),
            "denominated" => Ok(QueryParam::Denominated),
            "includefee" => Ok(QueryParam::IncludeFee),
            "includeclaimedrewards" => Ok(QueryParam::IncludeClaimedRewards),
            "useevmformat" => Ok(QueryParam::UseEvmFormat),
            "nometa" => Ok(QueryParam::NoMeta),
            "finalizedkey" => Ok(QueryParam::FinalizedKey),
            "usercblock" => Ok(QueryParam::UseRcBlock),
            _ => {
                let all_names: Vec<&str> = QueryParam::all_variants()
                    .iter()
                    .map(|p| p.name())
                    .collect();
                Err(format!(
                    "Unknown query param '{}'. Valid options:\n  {}",
                    s,
                    all_names.join(", ")
                ))
            }
        }
    }
}

/// Returns the query params supported by a given endpoint type.
pub fn supported_query_params(endpoint: &EndpointType) -> &'static [QueryParamKind] {
    use EndpointType::*;
    match endpoint {
        // Block detail endpoints: eventDocs, extrinsicDocs, noFees, decodedXcmMsgs, paraId, useEvmFormat, finalizedKey
        Block => &[
            QueryParamKind::EventDocs,
            QueryParamKind::ExtrinsicDocs,
            QueryParamKind::NoFees,
            QueryParamKind::DecodedXcmMsgs,
            QueryParamKind::ParaId,
            QueryParamKind::UseEvmFormat,
            QueryParamKind::FinalizedKey,
            QueryParamKind::UseRcBlock,
        ],

        RcBlocksBlockId => &[
            QueryParamKind::EventDocs,
            QueryParamKind::ExtrinsicDocs,
            QueryParamKind::NoFees,
            QueryParamKind::DecodedXcmMsgs,
            QueryParamKind::ParaId,
            QueryParamKind::UseEvmFormat,
            QueryParamKind::FinalizedKey,
        ],

        // Blocks head: all block params + finalized + useRcBlock
        BlocksHead => &[
            QueryParamKind::EventDocs,
            QueryParamKind::ExtrinsicDocs,
            QueryParamKind::NoFees,
            QueryParamKind::DecodedXcmMsgs,
            QueryParamKind::ParaId,
            QueryParamKind::UseEvmFormat,
            QueryParamKind::Finalized,
            QueryParamKind::UseRcBlock,
        ],

        BlocksHeadRcBlock => &[
            QueryParamKind::EventDocs,
            QueryParamKind::ExtrinsicDocs,
            QueryParamKind::NoFees,
            QueryParamKind::DecodedXcmMsgs,
            QueryParamKind::ParaId,
            QueryParamKind::UseEvmFormat,
            QueryParamKind::Finalized,
        ],

        // Block header: useRcBlock
        BlocksHeader => &[
            QueryParamKind::UseRcBlock,
        ],

        // Block extrinsics (info/raw list): eventDocs, extrinsicDocs, noFees, useEvmFormat
        BlockExtrinsics => &[
            QueryParamKind::EventDocs,
            QueryParamKind::ExtrinsicDocs,
            QueryParamKind::NoFees,
            QueryParamKind::UseEvmFormat,
        ],

        // Block extrinsics raw: + useRcBlock
        BlockExtrinsicsRaw => &[
            QueryParamKind::UseRcBlock,
        ],

        // Block extrinsic by index: eventDocs, extrinsicDocs, noFees, useEvmFormat, useRcBlock
        BlockExtrinsicsIdx => &[
            QueryParamKind::EventDocs,
            QueryParamKind::ExtrinsicDocs,
            QueryParamKind::NoFees,
            QueryParamKind::UseEvmFormat,
            QueryParamKind::UseRcBlock,
        ],

        BlockExtrinsicsIdxRcBlock | RcBlockExtrinsicsIdx => &[
            QueryParamKind::EventDocs,
            QueryParamKind::ExtrinsicDocs,
            QueryParamKind::NoFees,
            QueryParamKind::UseEvmFormat,
        ],

        // Range blocks: eventDocs, extrinsicDocs, noFees, useEvmFormat, useRcBlock
        RcBlocksRange => &[
            QueryParamKind::EventDocs,
            QueryParamKind::ExtrinsicDocs,
            QueryParamKind::NoFees,
            QueryParamKind::UseEvmFormat,
        ],

        // Pallet list endpoints: onlyIds
        PalletConsts | PalletStorage | RcPalletStorage | PalletDispatchables
        | RcPalletDispatchables | PalletErrors | RcPalletErrors | PalletEvents
        | RcPalletEvents => &[QueryParamKind::OnlyIds],

        // Pallet item endpoint: metadata
        PalletConstsConstantItem => &[QueryParamKind::Metadata],

        // Account balance: denominated
        AccountBalanceInfo | RcAccountBalanceInfo => &[QueryParamKind::Denominated],

        // Account staking info: includeClaimedRewards
        AccountStakingInfo => &[QueryParamKind::IncludeClaimedRewards],

        // Block para-inclusions: paraId
        BlockParaInclusions => &[QueryParamKind::ParaId],

        // Transaction material: noMeta
        TransactionMaterial => &[QueryParamKind::NoMeta],

        // Node transaction pool: includeFee (not currently an endpoint in checker, but ready)

        // Everything else: no extra query params
        _ => &[],
    }
}

/// Param kind identifiers used in the supported_query_params registry.
/// This avoids needing to construct full `QueryParam` values for static slices.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryParamKind {
    EventDocs,
    ExtrinsicDocs,
    NoFees,
    OnlyIds,
    Metadata,
    Finalized,
    DecodedXcmMsgs,
    ParaId,
    Denominated,
    IncludeFee,
    IncludeClaimedRewards,
    UseEvmFormat,
    NoMeta,
    FinalizedKey,
    UseRcBlock,
}

impl QueryParam {
    /// Get the kind identifier for this param (ignoring any carried value).
    pub fn kind(&self) -> QueryParamKind {
        match self {
            QueryParam::EventDocs => QueryParamKind::EventDocs,
            QueryParam::ExtrinsicDocs => QueryParamKind::ExtrinsicDocs,
            QueryParam::NoFees => QueryParamKind::NoFees,
            QueryParam::OnlyIds => QueryParamKind::OnlyIds,
            QueryParam::Metadata => QueryParamKind::Metadata,
            QueryParam::Finalized => QueryParamKind::Finalized,
            QueryParam::DecodedXcmMsgs => QueryParamKind::DecodedXcmMsgs,
            QueryParam::ParaId(_) => QueryParamKind::ParaId,
            QueryParam::Denominated => QueryParamKind::Denominated,
            QueryParam::IncludeFee => QueryParamKind::IncludeFee,
            QueryParam::IncludeClaimedRewards => QueryParamKind::IncludeClaimedRewards,
            QueryParam::UseEvmFormat => QueryParamKind::UseEvmFormat,
            QueryParam::NoMeta => QueryParamKind::NoMeta,
            QueryParam::FinalizedKey => QueryParamKind::FinalizedKey,
            QueryParam::UseRcBlock => QueryParamKind::UseRcBlock,
        }
    }

    /// Check if this param is supported by the given endpoint.
    pub fn is_supported_by(&self, endpoint: &EndpointType) -> bool {
        supported_query_params(endpoint).contains(&self.kind())
    }
}

/// Append query params to an existing URL path string.
pub fn append_query_params(mut url: String, query_params: &[QueryParam]) -> String {
    for param in query_params {
        let separator = if url.contains('?') { '&' } else { '?' };
        url.push(separator);
        url.push_str(&param.to_query_string());
    }
    url
}

/// Parse a comma-separated query params string (e.g. "eventDocs,noFees,paraId=2000").
/// The special value "all" returns all params supported by the given endpoint.
pub fn parse_query_params(
    input: &str,
    endpoint: &EndpointType,
) -> Result<Vec<QueryParam>, String> {
    if input.eq_ignore_ascii_case("all") {
        let supported = supported_query_params(endpoint);
        let params: Vec<QueryParam> = QueryParam::all_variants()
            .into_iter()
            .filter(|p| supported.contains(&p.kind()))
            .collect();
        if params.is_empty() {
            eprintln!(
                "Warning: endpoint '{}' has no supported query params",
                endpoint
            );
        }
        return Ok(params);
    }

    let mut params = Vec::new();
    let supported = supported_query_params(endpoint);

    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let param: QueryParam = part.parse()?;
        if !supported.contains(&param.kind()) {
            eprintln!(
                "Warning: '{}' is not supported by endpoint '{}', skipping",
                param.name(),
                endpoint
            );
            continue;
        }
        params.push(param);
    }

    Ok(params)
}

// ---------------------------------------------------------------------------
// Persistent query params coverage data (JSON-backed, like CoverageData)
// ---------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Merge overlapping or adjacent block ranges in-place.
fn merge_ranges(ranges: &mut Vec<(u32, u32)>) {
    if ranges.is_empty() {
        return;
    }
    ranges.sort_by_key(|r| r.0);
    let mut merged = Vec::new();
    let mut current = ranges[0];
    for &(start, end) in &ranges[1..] {
        if start <= current.1 + 1 {
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);
    *ranges = merged;
}

fn format_ranges(ranges: &[(u32, u32)]) -> String {
    if ranges.is_empty() {
        "-".to_string()
    } else {
        ranges
            .iter()
            .map(|(s, e)| format!("{}-{}", s, e))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn pass_rate(matched: u32, total: u32) -> f64 {
    if total == 0 {
        0.0
    } else {
        (matched as f64 / total as f64) * 100.0
    }
}

/// Coverage for a specific (endpoint, query-params-combo) pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QpEndpointCoverage {
    /// The query params that were used (e.g. "eventDocs, noFees")
    pub query_params: String,
    /// Block ranges tested
    pub block_ranges: Vec<(u32, u32)>,
    pub matched: u32,
    pub mismatched: u32,
    pub rust_errors: u32,
    pub sidecar_errors: u32,
    pub both_errors: u32,
    pub last_tested: String,
    /// Individual issues: (block_id, description)
    #[serde(default)]
    pub issues: Vec<(u64, String)>,
}

impl QpEndpointCoverage {
    fn total(&self) -> u32 {
        self.matched + self.mismatched + self.rust_errors + self.sidecar_errors + self.both_errors
    }
    fn pass_rate(&self) -> f64 {
        pass_rate(self.matched, self.total())
    }
    fn has_issues(&self) -> bool {
        self.mismatched > 0 || self.rust_errors > 0 || self.sidecar_errors > 0 || self.both_errors > 0
    }
}

/// Coverage for a single chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QpChainCoverage {
    /// chain name
    pub chain: String,
    /// endpoint name -> param combo key -> coverage
    /// The param combo key is a sorted, comma-joined list of param names.
    pub endpoints: HashMap<String, HashMap<String, QpEndpointCoverage>>,
    pub last_updated: String,
}

/// Root data structure, persisted as JSON.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QpCoverageData {
    pub version: String,
    pub chains: HashMap<String, QpChainCoverage>,
}

impl QpCoverageData {
    pub fn new() -> Self {
        Self {
            version: "1.0".to_string(),
            chains: HashMap::new(),
        }
    }

    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let data: QpCoverageData = serde_json::from_str(&content)?;
            Ok(data)
        } else {
            Ok(Self::new())
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Record a test run.
    pub fn add_run(&mut self, info: &QpRunInfo) {
        let chain_cov = self
            .chains
            .entry(info.chain.to_string())
            .or_insert_with(|| QpChainCoverage {
                chain: info.chain.to_string(),
                endpoints: HashMap::new(),
                last_updated: String::new(),
            });
        chain_cov.last_updated = chrono::Utc::now().to_rfc3339();

        let param_key = info.param_key();
        let ep_map = chain_cov
            .endpoints
            .entry(info.endpoint_name.to_string())
            .or_insert_with(HashMap::new);

        let cov = ep_map.entry(param_key.clone()).or_insert_with(|| {
            QpEndpointCoverage {
                query_params: param_key,
                block_ranges: Vec::new(),
                matched: 0,
                mismatched: 0,
                rust_errors: 0,
                sidecar_errors: 0,
                both_errors: 0,
                last_tested: String::new(),
                issues: Vec::new(),
            }
        });

        if let (Some(start), Some(end)) = (info.start_block, info.end_block) {
            cov.block_ranges.push((start, end));
            merge_ranges(&mut cov.block_ranges);
        }
        cov.matched += info.matched;
        cov.mismatched += info.mismatched;
        cov.rust_errors += info.rust_errors;
        cov.sidecar_errors += info.sidecar_errors;
        cov.both_errors += info.both_errors;
        cov.issues.extend_from_slice(&info.issues);
        cov.last_tested = chrono::Utc::now().to_rfc3339();
    }

    /// Generate and save `QUERY_PARAMS_SUMMARY.md` + `QUERY_PARAMS_DETAILS.md`.
    pub fn save_markdown_reports(&self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(dir)?;

        let summary = self.generate_summary();
        fs::write(dir.join("QUERY_PARAMS_SUMMARY.md"), summary)?;

        let details = self.generate_details();
        fs::write(dir.join("QUERY_PARAMS_DETAILS.md"), details)?;

        Ok(())
    }

    fn generate_summary(&self) -> String {
        let mut r = String::new();
        r.push_str("# Query Params Coverage Summary\n\n");
        r.push_str("Auto-generated from `--query-params` test runs. Accumulates across runs.\n\n");
        r.push_str("- **Details**: [QUERY_PARAMS_DETAILS.md](QUERY_PARAMS_DETAILS.md)\n\n");

        // Reference table
        r.push_str("## Supported Query Params by Endpoint\n\n");
        r.push_str("| Endpoint | Supported Query Params |\n");
        r.push_str("|----------|------------------------|\n");
        for (name, params) in &all_endpoints_with_params() {
            if params.is_empty() {
                continue; // skip endpoints with no params in the reference
            }
            let param_strs: Vec<&str> = params.iter().map(|k| query_param_kind_name(*k)).collect();
            r.push_str(&format!("| `{}` | {} |\n", name, param_strs.join(", ")));
        }
        r.push('\n');

        if self.chains.is_empty() {
            r.push_str("No query param test runs recorded yet.\n");
            return r;
        }

        // Per-chain tables
        for (chain_name, chain_cov) in &self.chains {
            r.push_str(&format!("## Chain: {}\n\n", chain_name));
            r.push_str(&format!("- **Last updated**: {}\n\n", chain_cov.last_updated));

            // Count tested vs all endpoints with params
            let all_ep = all_endpoints_with_params();
            let ep_with_params: Vec<&str> = all_ep.iter().filter(|(_, p)| !p.is_empty()).map(|(n, _)| *n).collect();
            let tested_count = ep_with_params.iter().filter(|n| chain_cov.endpoints.contains_key(**n)).count();

            r.push_str(&format!(
                "| Endpoints with query params tested | {}/{} |\n",
                tested_count,
                ep_with_params.len()
            ));
            r.push_str("|---|---|\n\n");

            r.push_str("| Endpoint | Query Params | Block Ranges | Pass Rate | Matched | Mismatched | Rust Err | Sidecar Err | Both Err |\n");
            r.push_str("|----------|-------------|--------------|-----------|---------|------------|----------|-------------|----------|\n");

            // Sort endpoints for deterministic output
            let mut ep_names: Vec<&String> = chain_cov.endpoints.keys().collect();
            ep_names.sort();

            for ep_name in ep_names {
                let combos = &chain_cov.endpoints[ep_name];
                let mut combo_keys: Vec<&String> = combos.keys().collect();
                combo_keys.sort();

                for key in combo_keys {
                    let cov = &combos[key];
                    let status = if cov.has_issues() {
                        format!("[{}](QUERY_PARAMS_DETAILS.md#{}---{})", ep_name, ep_name, key.replace(", ", "-"))
                    } else {
                        ep_name.to_string()
                    };
                    r.push_str(&format!(
                        "| {} | {} | {} | {:.1}% | {} | {} | {} | {} | {} |\n",
                        status,
                        cov.query_params,
                        format_ranges(&cov.block_ranges),
                        cov.pass_rate(),
                        cov.matched,
                        cov.mismatched,
                        cov.rust_errors,
                        cov.sidecar_errors,
                        cov.both_errors,
                    ));
                }
            }
            r.push('\n');
        }

        r
    }

    fn generate_details(&self) -> String {
        let mut r = String::new();
        r.push_str("# Query Params Coverage Details\n\n");
        r.push_str("Detailed breakdown of query param test runs.\n\n");
        r.push_str("- **Summary**: [QUERY_PARAMS_SUMMARY.md](QUERY_PARAMS_SUMMARY.md)\n\n");

        if self.chains.is_empty() {
            r.push_str("No query param test runs recorded yet.\n");
            return r;
        }

        for (chain_name, chain_cov) in &self.chains {
            r.push_str(&format!("## Chain: {}\n\n", chain_name));

            let mut ep_names: Vec<&String> = chain_cov.endpoints.keys().collect();
            ep_names.sort();

            for ep_name in ep_names {
                let combos = &chain_cov.endpoints[ep_name];
                let mut combo_keys: Vec<&String> = combos.keys().collect();
                combo_keys.sort();

                for key in combo_keys {
                    let cov = &combos[key];
                    r.push_str(&format!("### {} — {}\n\n", ep_name, cov.query_params));
                    r.push_str(&format!("- **Block ranges**: {}\n", format_ranges(&cov.block_ranges)));
                    r.push_str(&format!("- **Last tested**: {}\n", cov.last_tested));
                    r.push_str(&format!("- **Pass rate**: {:.1}%\n\n", cov.pass_rate()));

                    r.push_str("| Matched | Mismatched | Rust Err | Sidecar Err | Both Err (diff codes) |\n");
                    r.push_str("|---------|------------|----------|-------------|----------------------|\n");
                    r.push_str(&format!(
                        "| {} | {} | {} | {} | {} |\n\n",
                        cov.matched, cov.mismatched, cov.rust_errors, cov.sidecar_errors, cov.both_errors
                    ));

                    if !cov.issues.is_empty() {
                        r.push_str("#### Issues\n\n");
                        for (block_id, description) in &cov.issues {
                            r.push_str(&format!("- **Block {}**: `{}`\n", block_id, description));
                        }
                        r.push('\n');
                    }
                }
            }
        }

        r
    }
}

/// Info needed to record a query params test run.
pub struct QpRunInfo<'a> {
    pub chain: &'a str,
    pub endpoint_name: &'a str,
    pub query_params: &'a [QueryParam],
    pub start_block: Option<u32>,
    pub end_block: Option<u32>,
    pub matched: u32,
    pub mismatched: u32,
    pub rust_errors: u32,
    pub sidecar_errors: u32,
    pub both_errors: u32,
    pub issues: Vec<(u64, String)>,
}

impl<'a> QpRunInfo<'a> {
    /// Sorted, comma-joined param key (e.g. "eventDocs, noFees").
    pub fn param_key(&self) -> String {
        let mut names: Vec<String> = self.query_params.iter().map(|p| p.to_string()).collect();
        names.sort();
        names.join(", ")
    }
}

/// Get all endpoints paired with their supported query param kinds.
fn all_endpoints_with_params() -> Vec<(&'static str, Vec<QueryParamKind>)> {
    use EndpointType::*;
    let all_endpoints: Vec<(EndpointType, &str)> = vec![
        (Block, "block"),
        (BlocksHead, "blocks-head"),
        (BlocksHeadRcBlock, "blocks-head-rcblock"),
        (BlocksHeader, "blocks-header"),
        (RcBlocksBlockId, "rc-blocks-blockid"),
        (RcBlocksRange, "rc-blocks-range"),
        (BlockExtrinsics, "block-extrinsics"),
        (BlockExtrinsicsRaw, "block-extrinsics-raw"),
        (BlockExtrinsicsRawRcBlock, "block-extrinsics-raw-rcblock"),
        (BlockExtrinsicsIdx, "block-extrinsics-idx"),
        (BlockExtrinsicsIdxRcBlock, "block-extrinsics-idx-rcblock"),
        (RcBlockExtrinsicsRaw, "rc-block-extrinsics-raw"),
        (RcBlockExtrinsicsIdx, "rc-block-extrinsics-idx"),
        (BlockParaInclusions, "blocks-para-inclusions"),
        (AccountBalanceInfo, "account-balance-info"),
        (RcAccountBalanceInfo, "rc-account-balance-info"),
        (AccountForeignAssetBalances, "account-foreign-asset-balance"),
        (AccountStakingPayouts, "account-staking-payouts"),
        (AccountStakingInfo, "account-staking-info"),
        (AccountVestingInfo, "account-vesting-info"),
        (RcAccountVestingInfo, "rc-account-vesting-info"),
        (PalletConsts, "pallet-consts"),
        (PalletConstsConstantItem, "pallet-consts-item"),
        (PalletStorage, "pallet-storage"),
        (RcPalletStorage, "rc-pallet-storage"),
        (PalletDispatchables, "pallet-dispatchables"),
        (RcPalletDispatchables, "rc-pallet-dispatchables"),
        (PalletErrors, "pallet-errors"),
        (RcPalletErrors, "rc-pallet-errors"),
        (PalletEvents, "pallet-events"),
        (RcPalletEvents, "rc-pallet-events"),
        (PalletStakingValidators, "staking-validators"),
        (RcPalletStakingValidators, "rc-staking-validators"),
        (CoretimeInfo, "coretime-info"),
        (CoretimeOverview, "coretime-overview"),
        (CoretimeLeases, "coretime-leases"),
        (CoretimeReservations, "coretime-reservations"),
        (CoretimeRegions, "coretime-regions"),
        (RuntimeSpec, "runtime-spec"),
        (RuntimeMetadata, "runtime-metadata"),
        (TransactionMaterial, "tx-material"),
        (NodeVersion, "node-version"),
        (NodeNetwork, "node-network"),
    ];

    all_endpoints
        .into_iter()
        .map(|(ep, name)| {
            let params = supported_query_params(&ep).to_vec();
            (name, params)
        })
        .collect()
}

/// Human-readable name for a QueryParamKind.
fn query_param_kind_name(kind: QueryParamKind) -> &'static str {
    match kind {
        QueryParamKind::EventDocs => "eventDocs",
        QueryParamKind::ExtrinsicDocs => "extrinsicDocs",
        QueryParamKind::NoFees => "noFees",
        QueryParamKind::OnlyIds => "onlyIds",
        QueryParamKind::Metadata => "metadata",
        QueryParamKind::Finalized => "finalized",
        QueryParamKind::DecodedXcmMsgs => "decodedXcmMsgs",
        QueryParamKind::ParaId => "paraId",
        QueryParamKind::Denominated => "denominated",
        QueryParamKind::IncludeFee => "includeFee",
        QueryParamKind::IncludeClaimedRewards => "includeClaimedRewards",
        QueryParamKind::UseEvmFormat => "useEvmFormat",
        QueryParamKind::NoMeta => "noMeta",
        QueryParamKind::FinalizedKey => "finalizedKey",
        QueryParamKind::UseRcBlock => "useRcBlock",
    }
}
