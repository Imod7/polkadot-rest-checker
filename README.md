# Polkadot REST Checker

A CLI tool for testing and comparing responses between the new Rust-based `polkadot-rest-api` and the TypeScript-based `substrate-api-sidecar`. It scans various endpoints across block ranges and reports any differences or errors.

## Features

- Test multiple endpoint types: pallet, block, and runtime endpoints
- Compare responses between two API implementations
- Support for multiple chains: Polkadot, Kusama, Asset Hub Polkadot, Asset Hub Kusama
- Concurrent batch processing for fast scanning
- Detailed error and mismatch logging to files
- Summary reports with match rates

## Prerequisites

You need two API servers running:

1. **polkadot-rest-api** (Rust) - The new implementation being tested
2. **substrate-api-sidecar** (TypeScript) - The reference implementation

Both must be connected to the same chain and synced to the same blocks.

## Setup

### 1. Start polkadot-rest-api (default port: 8080)

```bash
cd polkadot-rest-api

# For Polkadot relay chain
SAS_SUBSTRATE_URL=wss://rpc.polkadot.io cargo run --release

# For Kusama relay chain
SAS_SUBSTRATE_URL=wss://kusama-rpc.polkadot.io cargo run --release

# For Asset Hub Polkadot
SAS_SUBSTRATE_URL=wss://polkadot-asset-hub-rpc.polkadot.io cargo run --release

# For Asset Hub Kusama
SAS_SUBSTRATE_URL=wss://kusama-asset-hub-rpc.polkadot.io cargo run --release

# Custom port
SAS_EXPRESS_PORT=8080 SAS_SUBSTRATE_URL=wss://rpc.polkadot.io cargo run --release
```

### 2. Start substrate-api-sidecar (default port: 8045)

```bash
cd substrate-api-sidecar

# Install dependencies
yarn install

# For Polkadot relay chain
SAS_SUBSTRATE_URL=wss://rpc.polkadot.io SAS_EXPRESS_PORT=8045 yarn start

# For Kusama relay chain
SAS_SUBSTRATE_URL=wss://kusama-rpc.polkadot.io SAS_EXPRESS_PORT=8045 yarn start

# For Asset Hub Polkadot
SAS_SUBSTRATE_URL=wss://polkadot-asset-hub-rpc.polkadot.io SAS_EXPRESS_PORT=8045 yarn start

# For Asset Hub Kusama
SAS_SUBSTRATE_URL=wss://kusama-asset-hub-rpc.polkadot.io SAS_EXPRESS_PORT=8045 yarn start
```

### 3. Build the checker

```bash
cd polkadot-rest-checker
cargo build --release
```

## Usage

```bash
polkadot-rest-checker [OPTIONS]
```

### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--chain` | `-c` | Chain to test | `polkadot` |
| `--endpoint` | | Endpoint type to test | `consts` |
| `--start` | `-s` | Start block number | `0` |
| `--end` | `-e` | End block number | latest block |
| `--batch-size` | `-b` | Concurrent requests per batch | `100` |
| `--url` | `-u` | Rust API base URL | `http://localhost:8080/v1` |
| `--sidecar-url` | | Sidecar API base URL | `http://localhost:8045` |
| `--delay` | `-d` | Delay between batches (ms) | `100` |
| `--pallet` | `-p` | Filter to specific pallet (pallet endpoints only) | all pallets |
| `--coverage-file` | | Path to coverage data file | `coverage.json` |
| `--coverage-report` | | Show coverage report and exit | - |

### Supported Chains

| Chain | Aliases |
|-------|---------|
| `polkadot` | `dot` |
| `kusama` | `ksm` |
| `asset-hub-polkadot` | `ahp`, `statemint` |
| `asset-hub-kusama` | `ahk`, `statemine` |

### Supported Endpoints

#### Pallet Endpoints (iterate over pallets and blocks)

| Endpoint | Aliases | API Path |
|----------|---------|----------|
| `pallet-consts` | `consts` | `/pallets/{pallet}/consts?at={block}` |
| `pallet-storage` | `storage` | `/pallets/{pallet}/storage?at={block}` |
| `pallet-dispatchables` | `dispatchables` | `/pallets/{pallet}/dispatchables?at={block}` |
| `pallet-errors` | `errors` | `/pallets/{pallet}/errors?at={block}` |
| `pallet-events` | `events` | `/pallets/{pallet}/events?at={block}` |

#### Block Endpoints (iterate over blocks only)

| Endpoint | Aliases | API Path |
|----------|---------|----------|
| `block` | `blocks` | `/blocks/{block}` |
| `block-header` | `header` | `/blocks/{block}/header` |
| `block-extrinsics` | `extrinsics` | `/blocks/{block}/extrinsics-info` |

#### Runtime Endpoints (single request, no iteration)

| Endpoint | Aliases | API Path |
|----------|---------|----------|
| `runtime-spec` | `spec` | `/runtime/spec` |
| `runtime-metadata` | `metadata` | `/runtime/metadata` |
| `tx-material` | `transaction-material` | `/transaction/material` |
| `node-version` | `version` | `/node/version` |
| `node-network` | `network` | `/node/network` |

## Examples

### Quick Start - Test pallet constants

```bash
# Test all pallets on Polkadot (default)
cargo run -- --start 0 --end 100

# Test only the System pallet
cargo run -- --pallet System --start 0 --end 100
```

### Pallet Endpoint Examples

```bash
# Test pallet constants
cargo run -- --endpoint consts --start 0 --end 1000

# Test pallet storage metadata
cargo run -- --endpoint storage --pallet Balances --start 0 --end 500

# Test pallet dispatchables (callable functions)
cargo run -- --endpoint dispatchables --start 0 --end 100

# Test pallet errors
cargo run -- --endpoint errors --pallet Staking --start 0 --end 100

# Test pallet events
cargo run -- --endpoint events --start 0 --end 100
```

### Block Endpoint Examples

```bash
# Test full block data
cargo run -- --endpoint block --start 1000000 --end 1001000

# Test block headers only (faster)
cargo run -- --endpoint block-header --start 0 --end 10000

# Test block extrinsics info
cargo run -- --endpoint extrinsics --start 1000000 --end 1000100
```

### Runtime Endpoint Examples

```bash
# Test runtime spec (single request, no block iteration)
cargo run -- --endpoint runtime-spec

# Test runtime metadata
cargo run -- --endpoint metadata

# Test transaction material
cargo run -- --endpoint tx-material

# Test node version
cargo run -- --endpoint node-version

# Test node network info
cargo run -- --endpoint node-network
```

### Chain-Specific Examples

```bash
# Test on Kusama
cargo run -- --chain kusama --endpoint block-header --start 0 --end 1000

# Test on Asset Hub Polkadot
cargo run -- --chain asset-hub-polkadot --endpoint consts --pallet Assets --start 0 --end 500

# Test on Asset Hub Kusama (using alias)
cargo run -- --chain ahk --endpoint storage --start 0 --end 100
```

### Custom Configuration Examples

```bash
# Use custom API URLs
cargo run -- \
  --url http://localhost:9080/v1 \
  --sidecar-url http://localhost:9045 \
  --endpoint block \
  --start 1000000 \
  --end 1001000

# Slower batch processing for rate-limited connections
cargo run -- --endpoint block --batch-size 10 --delay 500 --start 0 --end 100

# Large batch for fast local testing
cargo run -- --endpoint consts --batch-size 500 --delay 50 --start 0 --end 10000
```

### Workflow Examples

```bash
# 1. Quick sanity check with runtime endpoints
cargo run -- --endpoint runtime-spec
cargo run -- --endpoint node-version

# 2. Test a specific pallet thoroughly
cargo run -- --endpoint consts --pallet System --start 0 --end 100000
cargo run -- --endpoint storage --pallet System --start 0 --end 100000
cargo run -- --endpoint errors --pallet System --start 0 --end 100000

# 3. Full block comparison across a range
cargo run -- --endpoint block --start 20000000 --end 20001000
```

## Output

### Console Output

The tool prints progress and a summary:

```
Starting Polkadot REST API checker...
Chain: polkadot
Endpoint: pallet-consts
Rust API URL: http://localhost:8080/v1
Sidecar API URL: http://localhost:8045
Block range: 0 - 1000
Batch size: 100

============================================================
Scanning pallet: System (index: 0) - pallet-consts
============================================================
  Processing blocks 0 to 99...
  Block 0: http://localhost:8080/v1/pallets/System/consts?at=0 vs http://localhost:8045/pallets/System/consts?at=0
  ...

==========================================================================================
                                    FINAL SUMMARY
==========================================================================================
Chain: polkadot
Endpoint: pallet-consts
Block range: 0 - 1000
Total pallets scanned: 47

Pallet                    Matched   Mismatch    RustErr SidecarErr    BothErr     Rate
------------------------------------------------------------------------------------------
System                       1001          0          0          0          0  100.00%
Balances                     1001          0          0          0          0  100.00%
...
------------------------------------------------------------------------------------------
TOTAL                       47047          0          0          0          0  100.00%
```

### Log Files

**Summary logs** are always created with the final results:

```
# Pallet endpoints
summary_polkadot_0-1000_consts.log

# Block endpoints
summary_polkadot_0-1000_block.log

# Runtime endpoints
summary_polkadot_runtime-spec.log
```

**Error logs** are created when issues are found:

```
# Pallet endpoints (per pallet)
errors_polkadot_0-1000_consts_System.log
errors_polkadot_0-1000_storage_Balances.log

# Block endpoints
errors_polkadot_0-1000_block.log
errors_polkadot_0-1000_block-header.log

# Runtime endpoints
errors_polkadot_runtime-spec.log
```

Log file contents for mismatches include both responses:

```
# Error/Mismatch log for chain: polkadot, endpoint: pallet-consts, pallet: System (index: 0)
# Block range: 0 - 1000
# Rust API: http://localhost:8080/v1
# Sidecar API: http://localhost:8045
#
Block 42: MISMATCH - Responses differ
  Rust API response: {
    "at": { ... },
    "pallet": "System",
    ...
  }
  Sidecar response: {
    "at": { ... },
    "pallet": "System",
    ...
  }
```

## Result Categories

| Category | Description |
|----------|-------------|
| **Matched** | Both APIs returned identical responses |
| **Mismatch** | Both APIs succeeded but responses differ |
| **RustErr** | Rust API returned an error, Sidecar succeeded |
| **SidecarErr** | Sidecar returned an error, Rust API succeeded |
| **BothErr** | Both APIs returned errors |

## Coverage Tracking

The checker automatically tracks which endpoints, pallets, and block ranges have been tested across multiple runs. This helps you understand your overall API testing coverage.

### How it Works

- Every test run automatically saves results to a coverage file (default: `coverage.json`)
- Coverage data accumulates across runs, tracking:
  - Which endpoints have been tested
  - Which pallets have been tested for each endpoint
  - Which block ranges have been covered
  - Pass/fail rates for each endpoint and pallet

### Viewing Coverage Report

```bash
# Show coverage report
cargo run -- --coverage-report

# Use a custom coverage file
cargo run -- --coverage-file my-coverage.json --coverage-report
```

### Coverage Report Output

```
================================================================================
                           API COVERAGE REPORT
================================================================================

Chain: polkadot
Total pallets: 47
--------------------------------------------------------------------------------

PALLET ENDPOINTS:
  [✓] consts               47/47  pallets tested (100.0% pass rate)
      - System: blocks [0-1000, 5000-6000] (100.0% pass)
      - Balances: blocks [0-1000] (99.5% pass)
  [ ] storage              not tested
  [ ] dispatchables        not tested
  [ ] errors               not tested
  [ ] events               not tested

BLOCK ENDPOINTS:
  [✓] block                blocks [1000000-1001000] (98.5% pass rate)
  [ ] block-header         not tested
  [ ] block-extrinsics     not tested

RUNTIME ENDPOINTS:
  [✓] runtime-spec         tested (PASS)
  [✓] node-version         tested (PASS)
  [ ] runtime-metadata     not tested
  [ ] tx-material          not tested
  [ ] node-network         not tested

SUMMARY:
  Endpoints tested: 4/13
  Overall pass rate: 99.21% (12345/12443)
  Last updated: 2024-01-15T10:30:00Z

================================================================================
```

### Coverage File Format

Coverage data is stored in JSON format and can be analyzed programmatically:

```json
{
  "version": "1.0",
  "chains": {
    "polkadot": {
      "chain": "polkadot",
      "total_pallets": 47,
      "endpoints": {
        "consts": {
          "endpoint": "consts",
          "tested": true,
          "pallets": {
            "System": {
              "pallet": "System",
              "block_ranges": [[0, 1000], [5000, 6000]],
              "matched": 2001,
              "mismatched": 0,
              "rust_errors": 0,
              "sidecar_errors": 0,
              "both_errors": 0
            }
          }
        }
      }
    }
  }
}
```

### Multiple Chain Coverage

You can track coverage across multiple chains in the same file:

```bash
# Test Polkadot
cargo run -- --chain polkadot --endpoint consts --start 0 --end 1000

# Test Kusama (coverage accumulates)
cargo run -- --chain kusama --endpoint consts --start 0 --end 1000

# View combined coverage
cargo run -- --coverage-report
```

## Tips

1. **Start with a small block range** to verify both APIs are working before running large scans.

2. **Use `--pallet` filter** to focus on specific functionality when debugging pallet endpoints.

3. **Check log files** for detailed mismatch information - the console only shows the first 10-20 issues.

4. **Adjust `--batch-size` and `--delay`** if you're hitting rate limits or timeouts.

5. **Ensure both APIs connect to the same RPC endpoint** for accurate comparisons.

6. **Use runtime endpoints** for quick sanity checks before running longer scans.

## Troubleshooting

### "Failed to connect" errors

- Verify both API servers are running
- Check the URLs and ports are correct
- Ensure the servers have finished initializing

### Timeouts

- Reduce `--batch-size` to decrease concurrent load
- Increase `--delay` between batches
- Check if the RPC node is responding slowly

### All blocks show errors

- Verify the chain type matches the connected RPC
- Check if the block range exists on the chain
- Ensure both APIs are synced to the requested blocks

### "Unknown endpoint" error

Run with an invalid endpoint to see the list of valid options:
```bash
cargo run -- --endpoint invalid
# Error: Unknown endpoint 'invalid'. Valid options:
#   Pallet: consts, storage, dispatchables, errors, events
#   Block: block, block-header, block-extrinsics
#   Runtime: runtime-spec, runtime-metadata, tx-material
#   Node: node-version, node-network
```

## Adding New Endpoints

To add support for new endpoints, edit `src/endpoints.rs`:

### 1. Add a new variant to `EndpointType`

```rust
pub enum EndpointType {
    // ... existing variants ...

    // Add your new endpoint
    MyNewEndpoint,
}
```

### 2. Add path generation in the `path()` method

```rust
fn path(&self, pallet: Option<&str>, block: Option<u32>) -> String {
    match self {
        // ... existing matches ...

        EndpointType::MyNewEndpoint => {
            match block {
                Some(b) => format!("/my/endpoint?at={}", b),
                None => "/my/endpoint".to_string(),
            }
        }
    }
}
```

### 3. Add the short name for log files

```rust
fn short_name(&self) -> &'static str {
    match self {
        // ... existing matches ...
        EndpointType::MyNewEndpoint => "my-endpoint",
    }
}
```

### 4. Update `requires_pallet()` and `requires_block()` if needed

```rust
fn requires_pallet(&self) -> bool {
    // Return true if the endpoint needs to iterate over pallets
    matches!(self, EndpointType::PalletConsts | ...)
}

fn requires_block(&self) -> bool {
    // Return true if the endpoint needs block numbers
    matches!(self, EndpointType::Block | EndpointType::MyNewEndpoint | ...)
}
```

### 5. Add parsing in `FromStr` implementation

```rust
impl std::str::FromStr for EndpointType {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            // ... existing matches ...
            "my-endpoint" | "myendpoint" => Ok(EndpointType::MyNewEndpoint),
            _ => Err(...)
        }
    }
}
```

### 6. Add display implementation

```rust
impl fmt::Display for EndpointType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // ... existing matches ...
            EndpointType::MyNewEndpoint => write!(f, "my-endpoint"),
        }
    }
}
```

## Adding New Chains

To add support for new chains, edit `src/chains.rs`:

### 1. Add a new variant to `Chain`

```rust
pub enum Chain {
    // ... existing variants ...
    MyNewChain,
}
```

### 2. Add pallets for the new chain

```rust
pub const MY_NEW_CHAIN_PALLETS: &[Pallet] = &[
    Pallet { name: "System", index: 0 },
    Pallet { name: "Balances", index: 10 },
    // ... add all pallets from the chain's construct_runtime! macro
];
```

### 3. Update the `pallets()` method

```rust
impl Chain {
    pub fn pallets(&self) -> &'static [Pallet] {
        match self {
            // ... existing matches ...
            Chain::MyNewChain => MY_NEW_CHAIN_PALLETS,
        }
    }
}
```

### 4. Add parsing and display implementations

Update `FromStr` and `Display` implementations similarly to endpoints.
