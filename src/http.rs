use serde_json::Value;

use crate::diff::{json_diff, json_equal, JsonDiff};

/// Result of testing a block against both APIs
#[derive(Debug)]
pub enum TestResult {
    /// Both APIs returned success with matching responses
    Match,
    /// Both APIs returned success but responses differ
    Mismatch {
        rust_response: Value,
        sidecar_response: Value,
        diffs: Vec<JsonDiff>,
    },
    /// Rust API error
    RustError(String),
    /// Sidecar API error
    SidecarError(String),
    /// Both APIs returned errors
    BothError {
        rust_error: String,
        sidecar_error: String,
    },
}

pub async fn get_latest_block(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<u32, Box<dyn std::error::Error>> {
    let url = format!("{}/blocks/head", base_url);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to {}: {}", url, e))?;

    if !response.status().is_success() {
        return Err(format!("Server returned HTTP {}: {}", response.status(), url).into());
    }

    let json: Value = response
        .json()
        .await
        .map_err(|e| format!("Invalid JSON response from {}: {}", url, e))?;

    json["number"]
        .as_str()
        .and_then(|s| s.parse::<u32>().ok())
        .ok_or_else(|| format!("Missing or invalid 'number' field in response from {}", url).into())
}

/// Fetch JSON from a URL, returning Ok(Value) on success or Err(String) on failure
pub async fn fetch_json(client: &reqwest::Client, url: &str) -> Result<Value, String> {
    match client.get(url).send().await {
        Ok(response) => {
            let status = response.status();
            if status.is_success() {
                response
                    .json::<Value>()
                    .await
                    .map_err(|e| format!("Invalid JSON: {}", e))
            } else {
                Err(format!("HTTP {}", status))
            }
        }
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

/// Test a block by comparing responses from both APIs
pub async fn test_block_compare(
    client: reqwest::Client,
    rust_url: String,
    sidecar_url: String,
    block_num: u64,
) -> (u64, TestResult) {
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
                let diffs = json_diff(&rust_json, &sidecar_json);
                TestResult::Mismatch {
                    rust_response: rust_json,
                    sidecar_response: sidecar_json,
                    diffs,
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
