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
| `--coverage-file` | | Path to coverage data file | `coverage/coverage.json` |
| `--coverage-report` | | Show coverage report and exit | - |
| `--logs` | | Create detailed log files for errors and summaries | disabled |

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
| `pallet-consts-item` | `consts-item` | `/pallets/{pallet}/consts/{constant}?at={block}` |
| `pallet-storage` | `storage` | `/pallets/{pallet}/storage?at={block}` |
| `pallet-dispatchables` | `dispatchables` | `/pallets/{pallet}/dispatchables?at={block}` |
| `pallet-errors` | `errors` | `/pallets/{pallet}/errors?at={block}` |
| `pallet-events` | `events` | `/pallets/{pallet}/events?at={block}` |

> **Note:** The `consts-item` endpoint requires a special `--pallet` format: `PalletName/ConstantName` (e.g., `System/BlockHashCount`). See [Pallet Constant Item Examples](#pallet-constant-item-examples) for usage.

#### Block Endpoints (iterate over blocks only)

| Endpoint | Aliases | API Path |
|----------|---------|----------|
| `block` | `blocks` | `/blocks/{block}` |
| `blocks-head` | - | `/blocks/head` |
| `block-header` | `header` | `/blocks/{block}/header` |
| `block-extrinsics` | `extrinsics` | `/blocks/{block}/extrinsics-info` |
| `block-extrinsics-raw` | - | `/blocks/{block}/extrinsics-raw` |
| `block-extrinsics-raw-rcblock` | `block-extrinsics-raw-rc` | `/blocks/{block}/extrinsics-raw?useRcBlock=true` |
| `block-extrinsics-idx-rcblock` | `block-extrinsics-idx-rc` | `/blocks/{block}/extrinsics/{idx}?useRcBlock=true` |
| `block-para-inclusions` | `para-inclusions` | `/blocks/{block}/para-inclusions` |

#### Relay Chain Block Endpoints (iterate over blocks only)

| Endpoint | Aliases | API Path |
|----------|---------|----------|
| `rc-block-extrinsics-raw` | - | `/rc/blocks/{block}/extrinsics-raw` |
| `rc-block-extrinsics-idx` | - | `/rc/blocks/{block}/extrinsics/{index}` |

> **Note:** The `para-inclusions` endpoint only works on **relay chains** (Polkadot, Kusama) as it queries parachain inclusion events which don't exist on parachains like Asset Hub.

> **Note:** The `rc-block-extrinsics-idx` endpoint uses **dynamic extrinsic iteration**. For each block, it first fetches `/rc/blocks/{block}/extrinsics-raw` to discover how many extrinsics exist, then tests each index (0, 1, 2, ...) individually. This ensures all extrinsics in every block are tested, not just index 0.

#### Account Endpoints (iterate over test accounts and blocks)

| Endpoint | Aliases | API Path |
|----------|---------|----------|
| `account-balance-info` | `accounts-balance-info` | `/accounts/{address}/balance-info?at={block}` |

Account endpoints test against a predefined set of test accounts for each chain. See [Test Accounts](#test-accounts) for the list of accounts used.

#### Runtime Endpoints (single request, no iteration)

| Endpoint | Aliases | API Path |
|----------|---------|----------|
| `runtime-spec` | `spec` | `/runtime/spec` |
| `runtime-metadata` | `metadata` | `/runtime/metadata` |
| `tx-material` | `transaction-material` | `/transaction/material` |
| `node-version` | `version` | `/node/version` |
| `node-network` | `network` | `/node/network` |
| `blocks-head-rcblock` | `blocks-head-rc` | `/blocks/head?useRcBlock=true` |

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

### Pallet Constant Item Examples

The `consts-item` endpoint tests a specific constant within a pallet. Use the `--pallet` flag with format `PalletName/ConstantName`:

```bash
# Test a specific constant: System pallet's BlockHashCount
cargo run -- --endpoint consts-item --pallet "System/BlockHashCount" --start 1000 --end 1010

# Test Balances pallet's ExistentialDeposit constant
cargo run -- --endpoint consts-item --pallet "Balances/ExistentialDeposit" --start 1000 --end 1010

# Test Staking pallet's MaxNominations constant
cargo run -- --endpoint consts-item --pallet "Staking/MaxNominations" --start 1000 --end 1010

# Test on a specific chain
cargo run -- --chain kusama --endpoint consts-item --pallet "System/BlockHashCount" --start 1000 --end 1010
```

Common constants to test:
- `System/BlockHashCount` - Number of block hashes kept in storage
- `System/BlockLength` - Maximum block length limits
- `Balances/ExistentialDeposit` - Minimum balance to keep an account alive
- `Staking/MaxNominations` - Maximum nominations per nominator
- `TransactionPayment/OperationalFeeMultiplier` - Fee multiplier for operational transactions

### Block Endpoint Examples

```bash
# Test full block data
cargo run -- --endpoint block --start 1000000 --end 1001000

# Test block headers only (faster)
cargo run -- --endpoint block-header --start 0 --end 10000

# Test block extrinsics info
cargo run -- --endpoint extrinsics --start 1000000 --end 1000100

# Test block extrinsics raw
cargo run -- --endpoint block-extrinsics-raw --start 1000000 --end 1000100

# Test block extrinsics raw with relay chain block (useRcBlock=true)
cargo run -- --endpoint block-extrinsics-raw-rcblock --start 1000000 --end 1000100

# Test block extrinsics by index with relay chain block (useRcBlock=true)
cargo run -- --endpoint block-extrinsics-idx-rcblock --start 1000000 --end 1000010

# Test parachain inclusions (relay chain only)
cargo run -- --endpoint para-inclusions --start 10293194 --end 10293200
```

### Relay Chain Extrinsic Endpoint Examples

```bash
# Test relay chain extrinsics raw
cargo run -- --endpoint rc-block-extrinsics-raw --start 1000000 --end 1000100

# Test individual extrinsics by index (dynamic iteration)
# This will discover how many extrinsics exist in each block and test each one
cargo run -- --endpoint rc-block-extrinsics-idx --start 1000000 --end 1000010

# Example output:
#   Processing blocks 1000000 to 1000010...
#     Block 1000000: Found 5 extrinsics
#     Block 1000001: Found 3 extrinsics
#     Block 1000002: Found 7 extrinsics
#     ...

# With logging enabled to see detailed results
cargo run -- --endpoint rc-block-extrinsics-idx --start 1000000 --end 1000010 --logs

# Test on Kusama relay chain
cargo run -- --chain kusama --endpoint rc-block-extrinsics-idx --start 1000000 --end 1000010
```

> **How `rc-block-extrinsics-idx` works:**
> 1. For each block, fetches `/rc/blocks/{block}/extrinsics-raw` to count extrinsics
> 2. For each extrinsic index (0 to count-1), tests `/rc/blocks/{block}/extrinsics/{index}`
> 3. Compares responses between Rust API and Sidecar
>
> For a block with 5 extrinsics, this creates 5 comparison tests (one per extrinsic).

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

# Test blocks/head with relay chain block (useRcBlock=true)
cargo run -- --endpoint blocks-head-rcblock
```

### Account Endpoint Examples

```bash
# Test account balance-info across blocks
cargo run -- --endpoint account-balance-info --start 20000000 --end 20000100

# Test on Kusama
cargo run -- --chain kusama --endpoint account-balance-info --start 1000000 --end 1000100

# Test with logs enabled
cargo run -- --endpoint account-balance-info --start 20000000 --end 20000100 --logs
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

Log files are only created when using the `--logs` flag:

```bash
# Run with log files
cargo run -- --endpoint consts --start 0 --end 100 --logs

# Run without log files (default)
cargo run -- --endpoint consts --start 0 --end 100
```

**Summary logs** contain the final results:

```
# Pallet endpoints
summary_polkadot_0-1000_consts.log

# Block endpoints
summary_polkadot_0-1000_block.log

# Account endpoints
summary_polkadot_0-1000_account-balance-info_accounts.log

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

# Account endpoints (per account)
errors_polkadot_0-1000_account-balance-info_account_Account_1.log

# Runtime endpoints
errors_polkadot_runtime-spec.log
```

Log file contents for mismatches include the specific field differences and both full responses:

```
# Error/Mismatch log for chain: polkadot, endpoint: pallet-consts, pallet: System (index: 0)
# Block range: 0 - 1000
# Rust API: http://localhost:8080/v1
# Sidecar API: http://localhost:8045
#
Block 42: MISMATCH
  Differences (2):
    - at.height: rust="42" vs sidecar="43"
    - consts[0].value: rust=1000000 vs sidecar="1000000"

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

## Diff Detection

When responses differ, the checker automatically identifies and reports the specific fields that don't match. This makes debugging much faster than manually comparing large JSON responses.

### Console Output

Mismatches show the first few differences inline:

```
    Block 1000000: MISMATCH (3 diffs)
      - at.height: rust="1000000" vs sidecar="1000001"
      - extrinsics[0].method.pallet: rust="Balances" vs sidecar="balances"
      - timestamp: missing in sidecar (rust="2024-01-01T00:00:00Z")
      ... and 1 more
```

### Diff Types

| Type | Description | Example |
|------|-------------|---------|
| **ValueMismatch** | Same field, different values | `at.height: rust="100" vs sidecar="101"` |
| **MissingInSidecar** | Field exists in Rust only | `newField: missing in sidecar (rust="value")` |
| **MissingInRust** | Field exists in Sidecar only | `oldField: missing in rust (sidecar="value")` |
| **ArrayLengthMismatch** | Arrays have different lengths | `extrinsics: array length mismatch (rust=5 vs sidecar=3)` |
| **TypeMismatch** | Same field, different JSON types | `value: type mismatch (rust=number vs sidecar=string)` |

### Path Format

Field paths use dot notation for objects and bracket notation for arrays:
- `at.height` - nested object field
- `extrinsics[0].method` - first element of array, then nested field
- `pallets[2].storage[0].name` - deeply nested path

## Result Categories

| Category | Description |
|----------|-------------|
| **Matched** | Both APIs returned identical responses |
| **Mismatch** | Both APIs succeeded but responses differ (specific field differences are reported) |
| **RustErr** | Rust API returned an error, Sidecar succeeded |
| **SidecarErr** | Sidecar returned an error, Rust API succeeded |
| **BothErr** | Both APIs returned errors |

## Coverage Tracking

The checker automatically tracks which endpoints, pallets, and block ranges have been tested across multiple runs.

**Note:** [coverage/COVERAGE.md](coverage/COVERAGE.md) is auto-generated after each test run with current coverage data.

Quick start:
```bash
# Show coverage report in terminal
cargo run -- --coverage-report

# Run a test (this will update coverage/COVERAGE.md automatically)
cargo run -- --endpoint block --start 1000 --end 1010
```

## Tips

1. **Start with a small block range** to verify both APIs are working before running large scans.

2. **Use `--pallet` filter** to focus on specific functionality when debugging pallet endpoints.

3. **Check log files** for detailed mismatch information - the console only shows the first few differences per block.

4. **Diff detection helps identify issues quickly** - look at the field paths (e.g., `extrinsics[0].args.value`) to pinpoint exactly where responses differ.

5. **Adjust `--batch-size` and `--delay`** if you're hitting rate limits or timeouts.

6. **Ensure both APIs connect to the same RPC endpoint** for accurate comparisons.

7. **Use runtime endpoints** for quick sanity checks before running longer scans.

8. **Common diff patterns to watch for:**
   - Type mismatches (number vs string) often indicate serialization differences
   - Missing fields may indicate version differences between implementations
   - Case differences in strings are ignored (comparison is case-insensitive)

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
#   Block: block, blocks-head, block-header, block-extrinsics, para-inclusions
#   Runtime: runtime-spec, runtime-metadata, tx-material, blocks-head-rcblock
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

## Test Accounts

Account endpoints test against predefined accounts for each chain. These accounts are used when running `--endpoint account-balance-info`.

### Polkadot

| Label | Address |
|-------|---------|
| Account 1 | `15Mba2pkKLEaSsfH5nkVWnHqB1cbkmVVghTJbqthKSw7RmMs` |
| Account 2 | `1HwQbUpcr99UA6W7WBK86RtMHJTBWRazpxuYfRHyhSCbE1j` |
| Account 3 | `13nEo1kDJduJSpNkXCYFWXVLBUc2waRwcFGeFbwXWM9iALA6` |
| Account 4 | `1zunQTaRifL1XULrRLPgSbf6YbkZnjeJiQfwZuxVoJR5mhA` |

### Kusama

| Label | Address |
|-------|---------|
| Account 1 | `HCRUhtREEbmuWufk154isxs2Nt2s2mBfjfQqYtdzRSyyii8` |
| Account 2 | `G88i4RL8Py5mHWeeB62qaiGS2CXJpmr7ToJbMZ4FMFJxRjG` |
| Account 3 | `Gq2No2gcF6s4DLfzzuB53G5opWCoCtK9tZeVGRGcmkSDGoK` |
| Account 4 | `GFLdqBZKfPfbpbVB8rAc8tqqWSKpKHskkGHPGAgQ4atRkJ7` |

### Asset Hub Polkadot

| Label | Address |
|-------|---------|
| Account 1 | `19KT274PAdSchBjDmnxh6vEMdy4QFU9Bo6jgMZhen3esYGG` |
| Account 2 | `16GMHo9HZv8CcJy4WLoMaU9qusgzx2wxKDLbXStEBvt5274B` |
| Account 3 | `15sNh1RdsPQtrZ2w8THjGRYBjx2eAj2uWjfHiWiVvUJ6mzf2` |
| Account 4 | `13oSJ635GZos8UYrrcwXtp3XWC1G3XHrPvN6skMLdxzUr4sr` |

### Asset Hub Kusama

| Label | Address |
|-------|---------|
| Account 1 | `JLENz97TFT2kYaQmyCSEnBsK8VhaDZNmYATfsLCHyLF6Gzu` |
| Account 2 | `Gyyh8tbze83BxbZnwDoRs2RxRXhXEpnx8zau4jcaoutmcyY` |
| Account 3 | `EeDhCnEPX8eitysY6ApQXxMeiZSYMcst3YixR4nzMuARyVy` |
| Account 4 | `DSk9EUkPLu1n4ssFWwyRKYRJuJ15W2e6AiNw8eaywkv3ap6` |

### Adding New Test Accounts

To add or modify test accounts, edit `src/chains.rs`:

```rust
pub const POLKADOT_TEST_ACCOUNTS: &[TestAccount] = &[
    TestAccount { address: "15Mba2pkKLEaSsfH5nkVWnHqB1cbkmVVghTJbqthKSw7RmMs", label: "Account 1" },
    TestAccount { address: "1HwQbUpcr99UA6W7WBK86RtMHJTBWRazpxuYfRHyhSCbE1j", label: "Account 2" },
    // Add more accounts as needed
];
```
