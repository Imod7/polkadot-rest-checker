# Coverage Summary

This file is auto-generated from test results. Run tests to update coverage data.

- **Details**: [COVERAGE_DETAILS.md](COVERAGE_DETAILS.md)

## How it Works

- Every test run automatically saves results to a coverage file (default: `coverage/coverage.json`)
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
cargo run -- --coverage-file coverage/my-coverage.json --coverage-report
```

## Current Coverage

### Chain: polkadot

- **Total pallets:** 61
- **Last updated:** 2026-02-11T08:34:20.220824+00:00

| Metric | Value |
|--------|-------|
| Endpoints tested | 14/34 |
| Overall pass rate | 73.04% (4478/6131) |

#### Pallet Endpoints

| Endpoint | Status | Pallets Tested | Pass Rate |
|----------|--------|----------------|------------|
| pallet-consts | ❌ | - | - |
| pallet-consts-item | ❌ | - | - |
| pallet-storage | ❌ | - | - |
| pallet-dispatchables | ❌ | - | - |
| pallet-errors | ❌ | - | - |
| rc-pallet-errors | ❌ | - | - |
| pallet-events | ❌ | - | - |
| rc-pallet-events | ❌ | - | - |

#### Block Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| block | ❌ | - | - |
| blocks-head | ❌ | - | - |
| block-header | ❌ | - | - |
| block-extrinsics | ❌ | - | - |
| block-extrinsics-raw | ✅ | 3233222-3233232, 9233222-9233232, 13233222-13233250, 22233222-22233250, 23233222-23233232 | 100.0% |
| block-extrinsics-raw-rcblock | ✅ | 1000000-1000100, 29290300-29290370, 29290600-29290670, 29300500-29300550 | 72.3% |
| block-extrinsics-idx | ❌ | - | - |
| block-extrinsics-idx-rcblock | ✅ | 30103052-30103059, 30123052-30123059, 30423052-30423059, 30723052-30723059 | 100.0% |
| rc-block-extrinsics-raw | ❌ | - | - |
| rc-block-extrinsics-idx | ❌ | - | - |
| block-para-inclusions | ✅ | 1500200-1500250, 5840200-5840240, 8500200-8500250, 10500200-10500250 | 61.6% |
| staking-validators | ✅ | 10494650-10494660, 20494650-20494660, 28494651-28494660 | 68.8% |
| rc-staking-validators | ❌ | - | - |
| coretime-info | ✅ | 3200700-3200710, 8200700-8200710 | 0.0% |
| coretime-overview | ✅ | 3120700-3120710, 3200700-3200710, 29000000-29000010 | 0.0% |
| coretime-leases | ❌ | - | - |
| coretime-reservations | ❌ | - | - |
| coretime-regions | ❌ | - | - |

#### Account Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| account-balance-info | ✅ | 15233222-15233250, 22233222-22233250, 24233222-24233250 | 91.2% |
| account-foreign-asset-balance | ✅ | 20000000-20000100 | 0.0% |

#### Standalone Endpoints

| Endpoint | Status | Result |
|----------|--------|--------|
| runtime-spec | ❌ | - |
| runtime-metadata | ❌ | - |
| tx-material | ❌ | - |
| node-version | ❌ | - |
| node-network | ❌ | - |
| blocks-head-rcblock | ✅ | FAIL |

### Chain: kusama

- **Total pallets:** 65
- **Last updated:** 2026-02-05T10:13:41.885902+00:00

| Metric | Value |
|--------|-------|
| Endpoints tested | 6/34 |
| Overall pass rate | 43.27% (964/2228) |

#### Pallet Endpoints

| Endpoint | Status | Pallets Tested | Pass Rate |
|----------|--------|----------------|------------|
| pallet-consts | ❌ | - | - |
| pallet-consts-item | ❌ | - | - |
| pallet-storage | ❌ | - | - |
| pallet-dispatchables | ❌ | - | - |
| pallet-errors | ❌ | - | - |
| rc-pallet-errors | ❌ | - | - |
| pallet-events | ❌ | - | - |
| rc-pallet-events | ❌ | - | - |

#### Block Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| block | ❌ | - | - |
| blocks-head | ❌ | - | - |
| block-header | ❌ | - | - |
| block-extrinsics | ❌ | - | - |
| block-extrinsics-raw | ✅ | 233241-233255, 10233241-10233255 | 100.0% |
| block-extrinsics-raw-rcblock | ❌ | - | - |
| block-extrinsics-idx | ❌ | - | - |
| block-extrinsics-idx-rcblock | ❌ | - | - |
| rc-block-extrinsics-raw | ❌ | - | - |
| rc-block-extrinsics-idx | ❌ | - | - |
| block-para-inclusions | ❌ | - | - |
| staking-validators | ✅ | 18494651-18494660, 38494651-38494660 | 50.0% |
| rc-staking-validators | ❌ | - | - |
| coretime-info | ✅ | 29200700-29200710, 30500400-30500410 | 0.0% |
| coretime-overview | ❌ | - | - |
| coretime-leases | ❌ | - | - |
| coretime-reservations | ❌ | - | - |
| coretime-regions | ❌ | - | - |

#### Account Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| account-balance-info | ❌ | - | - |
| account-foreign-asset-balance | ❌ | - | - |

#### Standalone Endpoints

| Endpoint | Status | Result |
|----------|--------|--------|
| runtime-spec | ❌ | - |
| runtime-metadata | ❌ | - |
| tx-material | ❌ | - |
| node-version | ❌ | - |
| node-network | ❌ | - |
| blocks-head-rcblock | ❌ | - |

### Chain: asset-hub-polkadot

- **Total pallets:** 55
- **Last updated:** 2026-02-12T19:08:23.295039+00:00

| Metric | Value |
|--------|-------|
| Endpoints tested | 12/34 |
| Overall pass rate | 41.88% (1834/4379) |

#### Pallet Endpoints

| Endpoint | Status | Pallets Tested | Pass Rate |
|----------|--------|----------------|------------|
| [pallet-consts](COVERAGE_DETAILS.md#pallet-consts) | ✅ | 55/55 | 50.0% |
| pallet-consts-item | ❌ | - | - |
| pallet-storage | ❌ | - | - |
| pallet-dispatchables | ❌ | - | - |
| pallet-errors | ❌ | - | - |
| [rc-pallet-errors](COVERAGE_DETAILS.md#rc-pallet-errors) | ✅ | 55/55 | 0.0% |
| [pallet-events](COVERAGE_DETAILS.md#pallet-events) | ✅ | 55/55 | 44.4% |
| [rc-pallet-events](COVERAGE_DETAILS.md#rc-pallet-events) | ✅ | 55/55 | 42.1% |

#### Block Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| block | ❌ | - | - |
| blocks-head | ❌ | - | - |
| block-header | ❌ | - | - |
| block-extrinsics | ✅ | 2200200-2200210 | 0.0% |
| block-extrinsics-raw | ✅ | 2600200-2600240, 10600200-10600240, 12600200-12600240 | 75.0% |
| block-extrinsics-raw-rcblock | ❌ | - | - |
| block-extrinsics-idx | ❌ | - | - |
| block-extrinsics-idx-rcblock | ❌ | - | - |
| rc-block-extrinsics-raw | ✅ | 1900100-1900110, 2300200-2300210, 10678777-10678787, 15678777-15678787, 22678777-22678787, 25678777-25678787 | 100.0% |
| rc-block-extrinsics-idx | ✅ | 1000000-1000010, 28494700-28494710 | 100.0% |
| block-para-inclusions | ❌ | - | - |
| staking-validators | ✅ | 10678777-10678787, 11494651-11494660, 18494651-18494660 | 35.5% |
| rc-staking-validators | ✅ | 10678777-10678787 | 100.0% |
| coretime-info | ❌ | - | - |
| coretime-overview | ❌ | - | - |
| coretime-leases | ❌ | - | - |
| coretime-reservations | ❌ | - | - |
| coretime-regions | ❌ | - | - |

#### Account Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| account-balance-info | ❌ | - | - |
| account-foreign-asset-balance | ❌ | - | - |

#### Standalone Endpoints

| Endpoint | Status | Result |
|----------|--------|--------|
| runtime-spec | ❌ | - |
| runtime-metadata | ❌ | - |
| tx-material | ❌ | - |
| node-version | ❌ | - |
| node-network | ❌ | - |
| blocks-head-rcblock | ❌ | - |

### Chain: asset-hub-kusama

- **Total pallets:** 59
- **Last updated:** 2026-02-11T11:08:10.604774+00:00

| Metric | Value |
|--------|-------|
| Endpoints tested | 6/34 |
| Overall pass rate | 18.33% (922/5029) |

#### Pallet Endpoints

| Endpoint | Status | Pallets Tested | Pass Rate |
|----------|--------|----------------|------------|
| pallet-consts | ❌ | - | - |
| pallet-consts-item | ❌ | - | - |
| pallet-storage | ❌ | - | - |
| pallet-dispatchables | ❌ | - | - |
| pallet-errors | ❌ | - | - |
| rc-pallet-errors | ❌ | - | - |
| pallet-events | ❌ | - | - |
| rc-pallet-events | ❌ | - | - |

#### Block Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| block | ❌ | - | - |
| blocks-head | ❌ | - | - |
| block-header | ❌ | - | - |
| block-extrinsics | ❌ | - | - |
| block-extrinsics-raw | ❌ | - | - |
| block-extrinsics-raw-rcblock | ❌ | - | - |
| block-extrinsics-idx | ✅ | 12798138-12798149, 12940347-12940359, 12958046-12958049, 12958582-12958589 | 62.5% |
| block-extrinsics-idx-rcblock | ❌ | - | - |
| rc-block-extrinsics-raw | ✅ | 1500200-1500210, 10678777-10678787, 11300200-11300210, 12678777-12678787, 20678777-20678787, 22678777-22678787, 25678777-25678787, 29678777-29678787, 30300200-30300210, 31678777-31678787 | 66.7% |
| rc-block-extrinsics-idx | ✅ | 3494700-3494710, 12958046-12958049, 13494700-13494710, 21494700-21494710, 28494700-28494710, 30423052-30423059, 30423265-30423269 | 97.0% |
| block-para-inclusions | ❌ | - | - |
| staking-validators | ❌ | - | - |
| rc-staking-validators | ❌ | - | - |
| coretime-info | ❌ | - | - |
| coretime-overview | ❌ | - | - |
| coretime-leases | ❌ | - | - |
| coretime-reservations | ❌ | - | - |
| coretime-regions | ❌ | - | - |

#### Account Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| account-balance-info | ✅ | 10000-10010 | 0.0% |
| account-foreign-asset-balance | ✅ | 1000000-1000100, 10300400-10300410, 12300400-12300410 | 0.0% |

#### Standalone Endpoints

| Endpoint | Status | Result |
|----------|--------|--------|
| runtime-spec | ❌ | - |
| runtime-metadata | ❌ | - |
| tx-material | ❌ | - |
| node-version | ❌ | - |
| node-network | ❌ | - |
| blocks-head-rcblock | ❌ | - |

### Chain: coretime-polkadot

- **Total pallets:** 0
- **Last updated:** 2026-02-01T21:17:58.754673+00:00

| Metric | Value |
|--------|-------|
| Endpoints tested | 3/34 |
| Overall pass rate | 25.00% (6/24) |

#### Pallet Endpoints

| Endpoint | Status | Pallets Tested | Pass Rate |
|----------|--------|----------------|------------|
| pallet-consts | ❌ | - | - |
| pallet-consts-item | ❌ | - | - |
| pallet-storage | ❌ | - | - |
| pallet-dispatchables | ❌ | - | - |
| pallet-errors | ❌ | - | - |
| rc-pallet-errors | ❌ | - | - |
| pallet-events | ❌ | - | - |
| rc-pallet-events | ❌ | - | - |

#### Block Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| block | ❌ | - | - |
| blocks-head | ❌ | - | - |
| block-header | ❌ | - | - |
| block-extrinsics | ❌ | - | - |
| block-extrinsics-raw | ❌ | - | - |
| block-extrinsics-raw-rcblock | ❌ | - | - |
| block-extrinsics-idx | ❌ | - | - |
| block-extrinsics-idx-rcblock | ❌ | - | - |
| rc-block-extrinsics-raw | ❌ | - | - |
| rc-block-extrinsics-idx | ❌ | - | - |
| block-para-inclusions | ❌ | - | - |
| staking-validators | ❌ | - | - |
| rc-staking-validators | ❌ | - | - |
| coretime-info | ❌ | - | - |
| coretime-overview | ❌ | - | - |
| coretime-leases | ✅ | 1970105-1970110, 2770105-2770110 | 0.0% |
| coretime-reservations | ✅ | 1970105-1970110 | 100.0% |
| coretime-regions | ✅ | 1970105-1970110 | 0.0% |

#### Account Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| account-balance-info | ❌ | - | - |
| account-foreign-asset-balance | ❌ | - | - |

#### Standalone Endpoints

| Endpoint | Status | Result |
|----------|--------|--------|
| runtime-spec | ❌ | - |
| runtime-metadata | ❌ | - |
| tx-material | ❌ | - |
| node-version | ❌ | - |
| node-network | ❌ | - |
| blocks-head-rcblock | ❌ | - |

## Coverage File Format

Coverage data is stored in JSON format (`coverage/coverage.json`) and can be analyzed programmatically.
