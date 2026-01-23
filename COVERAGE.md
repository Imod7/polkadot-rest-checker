# Coverage Tracking

The checker automatically tracks which endpoints, pallets, and block ranges have been tested across multiple runs. This helps you understand your overall API testing coverage.

## How it Works

- Every test run automatically saves results to a coverage file (default: `coverage.json`)
- Coverage data accumulates across runs, tracking:
  - Which endpoints have been tested
  - Which pallets have been tested for each endpoint
  - Which block ranges have been covered
  - Pass/fail rates for each endpoint and pallet

## Viewing Coverage Report

```bash
# Show coverage report
cargo run -- --coverage-report

# Use a custom coverage file
cargo run -- --coverage-file my-coverage.json --coverage-report
```

## Coverage Report Output

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
  [ ] block-para-inclusions not tested

RUNTIME ENDPOINTS:
  [✓] runtime-spec         tested (PASS)
  [✓] node-version         tested (PASS)
  [ ] runtime-metadata     not tested
  [ ] tx-material          not tested
  [ ] node-network         not tested

SUMMARY:
  Endpoints tested: 4/14
  Overall pass rate: 99.21% (12345/12443)
  Last updated: 2024-01-15T10:30:00Z

================================================================================
```

## Coverage File Format

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

## Multiple Chain Coverage

You can track coverage across multiple chains in the same file:

```bash
# Test Polkadot
cargo run -- --chain polkadot --endpoint consts --start 0 --end 1000

# Test Kusama (coverage accumulates)
cargo run -- --chain kusama --endpoint consts --start 0 --end 1000

# View combined coverage
cargo run -- --coverage-report
```
