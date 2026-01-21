# Polkadot REST Checker

A CLI tool for testing and comparing responses between the new Rust-based `polkadot-rest-api` and the TypeScript-based `substrate-api-sidecar`. It scans pallet endpoints across block ranges and reports any differences or errors.

## Features

- Test pallet constants endpoints across block ranges
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
| `--start` | `-s` | Start block number | `0` |
| `--end` | `-e` | End block number | latest block |
| `--batch-size` | `-b` | Concurrent requests per batch | `100` |
| `--url` | `-u` | Rust API base URL | `http://localhost:8080/v1` |
| `--sidecar-url` | | Sidecar API base URL | `http://localhost:8045` |
| `--delay` | `-d` | Delay between batches (ms) | `100` |
| `--pallet` | `-p` | Filter to specific pallet | all pallets |

### Supported Chains

| Chain | Aliases |
|-------|---------|
| `polkadot` | `dot` |
| `kusama` | `ksm` |
| `asset-hub-polkadot` | `ahp`, `statemint` |
| `asset-hub-kusama` | `ahk`, `statemine` |

## Examples

### Basic usage - test all Polkadot pallets

```bash
cargo run -- --start 0 --end 1000
```

### Test specific pallet on Kusama

```bash
cargo run -- --chain kusama --pallet Balances --start 0 --end 500
```

### Test Asset Hub Polkadot with custom ports

```bash
cargo run -- \
  --chain asset-hub-polkadot \
  --url http://localhost:9080/v1 \
  --sidecar-url http://localhost:9045 \
  --start 1000000 \
  --end 1001000
```

### Test with smaller batch size (for slower connections)

```bash
cargo run -- --batch-size 10 --delay 500 --start 0 --end 100
```

### Test only System pallet across a large range

```bash
cargo run -- --pallet System --start 0 --end 1000000
```

## Output

### Console Output

The tool prints progress and a summary table:

```
Starting Polkadot REST API checker...
Chain: polkadot
Rust API URL: http://localhost:8080/v1
Sidecar API URL: http://localhost:8045
Block range: 0 - 1000
Batch size: 100

==============================================================
Scanning pallet: System (index: 0)
==============================================================
  Processing blocks 0 to 99...
  Block 0: http://localhost:8080/v1/pallets/System/consts?at=0 vs http://localhost:8045/pallets/System/consts?at=0
  ...

==========================================================================================
                                    FINAL SUMMARY
==========================================================================================
Chain: polkadot
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

When issues are found, they're saved to log files:

```
errors_polkadot_0-1000_System.log
errors_polkadot_0-1000_Balances.log
```

Log file contents for mismatches include both responses:

```
# Error/Mismatch log for chain: polkadot, pallet: System (index: 0)
# Block range: 0 - 1000
# Rust API: http://localhost:8080/v1/pallets/System/consts
# Sidecar API: http://localhost:8045/pallets/System/consts
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

## Tips

1. **Start with a small block range** to verify both APIs are working before running large scans.

2. **Use `--pallet` filter** to focus on specific functionality when debugging.

3. **Check log files** for detailed mismatch information - the console only shows the first 10 issues per pallet.

4. **Adjust `--batch-size` and `--delay`** if you're hitting rate limits or timeouts.

5. **Ensure both APIs connect to the same RPC endpoint** for accurate comparisons.

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
