# Coverage Tracking

This file is auto-generated from test results. Run tests to update coverage data.

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

### Chain: asset-hub-polkadot

- **Total pallets:** 55
- **Last updated:** 2026-01-26T15:38:40.121821+00:00

| Metric | Value |
|--------|-------|
| Endpoints tested | 3/17 |
| Overall pass rate | 75.00% (156/208) |

#### Pallet Endpoints

| Endpoint | Status | Pallets Tested | Pass Rate |
|----------|--------|----------------|------------|
| consts | ❌ | - | - |
| storage | ❌ | - | - |
| dispatchables | ❌ | - | - |
| errors | ❌ | - | - |
| events | ❌ | - | - |

#### Block Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| block | ❌ | - | - |
| block-header | ❌ | - | - |
| block-extrinsics | ✅ | 2200200-2200210 | 0.0% |
| block-extrinsics-raw | ✅ | 2600200-2600240, 10600200-10600240, 12600200-12600240 | 75.0% |
| rc-block-extrinsics-raw | ✅ | 1900100-1900110, 2300200-2300210 | 100.0% |
| block-para-inclusions | ❌ | - | - |

#### Account Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| account-balance-info | ❌ | - | - |

#### Runtime Endpoints

| Endpoint | Status | Result |
|----------|--------|--------|
| runtime-spec | ❌ | - |
| runtime-metadata | ❌ | - |
| tx-material | ❌ | - |
| node-version | ❌ | - |
| node-network | ❌ | - |

### Chain: polkadot

- **Total pallets:** 61
- **Last updated:** 2026-01-23T15:48:34.865734+00:00

| Metric | Value |
|--------|-------|
| Endpoints tested | 5/17 |
| Overall pass rate | 53.47% (3369/6301) |

#### Pallet Endpoints

| Endpoint | Status | Pallets Tested | Pass Rate |
|----------|--------|----------------|------------|
| consts | ✅ | 61/61 | 0.0% |
| storage | ✅ | 61/61 | 90.2% |
| dispatchables | ❌ | - | - |
| errors | ❌ | - | - |
| events | ❌ | - | - |

#### Block Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| block | ❌ | - | - |
| block-header | ❌ | - | - |
| block-extrinsics | ❌ | - | - |
| block-extrinsics-raw | ✅ | 3233222-3233232, 9233222-9233232, 13233222-13233250, 22233222-22233250, 23233222-23233232 | 100.0% |
| rc-block-extrinsics-raw | ❌ | - | - |
| block-para-inclusions | ✅ | 1500200-1500250, 5840200-5840240, 8500200-8500250, 10500200-10500250 | 61.6% |

#### Account Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| account-balance-info | ✅ | 15233222-15233250, 22233222-22233250, 24233222-24233250 | 91.2% |

#### Runtime Endpoints

| Endpoint | Status | Result |
|----------|--------|--------|
| runtime-spec | ❌ | - |
| runtime-metadata | ❌ | - |
| tx-material | ❌ | - |
| node-version | ❌ | - |
| node-network | ❌ | - |

#### Detailed Pallet Coverage

**consts:**

| Pallet | Block Ranges | Matched | Mismatched | Errors | Pass Rate |
|--------|--------------|---------|------------|--------|------------|
| AssetRate | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Auctions | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| AuthorityDiscovery | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Authorship | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Babe | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Balances | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Beefy | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| BeefyMmrLeaf | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Bounties | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| ChildBounties | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Claims | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Configuration | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| ConvictionVoting | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Coretime | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| CoretimeAssignmentProvider | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Crowdloan | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| DelegatedStaking | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Dmp | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| ElectionProviderMultiPhase | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| FastUnstake | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Grandpa | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Historical | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Hrmp | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Indices | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Initializer | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| MessageQueue | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Mmr | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Multisig | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| NominationPools | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Offences | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| OnDemand | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Origins | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| ParaInclusion | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| ParaInherent | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| ParaScheduler | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| ParaSessionInfo | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| ParachainsOrigin | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Paras | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| ParasDisputes | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| ParasShared | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| ParasSlashing | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Preimage | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Proxy | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| RcMigrator | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Referenda | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Registrar | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Scheduler | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Session | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Slots | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Staking | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| StakingAhClient | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| StateTrieMigration | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| System | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Timestamp | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| TransactionPayment | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Treasury | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Utility | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Vesting | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| VoterList | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| Whitelist | 8840200-8840240 | 0 | 0 | 41 | 0.0% |
| XcmPallet | 8840200-8840240 | 0 | 0 | 41 | 0.0% |

**storage:**

| Pallet | Block Ranges | Matched | Mismatched | Errors | Pass Rate |
|--------|--------------|---------|------------|--------|------------|
| AssetRate | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Auctions | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| AuthorityDiscovery | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Authorship | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Babe | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Balances | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Beefy | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| BeefyMmrLeaf | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Bounties | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| ChildBounties | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Claims | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Configuration | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| ConvictionVoting | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Coretime | 25500200-25500250 | 0 | 0 | 51 | 0.0% |
| CoretimeAssignmentProvider | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Crowdloan | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| DelegatedStaking | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Dmp | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| ElectionProviderMultiPhase | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| FastUnstake | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Grandpa | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Historical | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Hrmp | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Indices | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Initializer | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| MessageQueue | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Mmr | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Multisig | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| NominationPools | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Offences | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| OnDemand | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Origins | 25500200-25500250 | 0 | 0 | 51 | 0.0% |
| ParaInclusion | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| ParaInherent | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| ParaScheduler | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| ParaSessionInfo | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| ParachainsOrigin | 25500200-25500250 | 0 | 0 | 51 | 0.0% |
| Paras | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| ParasDisputes | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| ParasShared | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| ParasSlashing | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Preimage | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Proxy | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| RcMigrator | 25500200-25500250 | 0 | 0 | 51 | 0.0% |
| Referenda | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Registrar | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Scheduler | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Session | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Slots | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Staking | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| StakingAhClient | 25500200-25500250 | 0 | 0 | 51 | 0.0% |
| StateTrieMigration | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| System | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Timestamp | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| TransactionPayment | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Treasury | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Utility | 25500200-25500250 | 0 | 0 | 51 | 0.0% |
| Vesting | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| VoterList | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| Whitelist | 25500200-25500250 | 51 | 0 | 0 | 100.0% |
| XcmPallet | 25500200-25500250 | 51 | 0 | 0 | 100.0% |

### Chain: kusama

- **Total pallets:** 65
- **Last updated:** 2026-01-23T15:55:06.130495+00:00

| Metric | Value |
|--------|-------|
| Endpoints tested | 1/17 |
| Overall pass rate | 100.00% (30/30) |

#### Pallet Endpoints

| Endpoint | Status | Pallets Tested | Pass Rate |
|----------|--------|----------------|------------|
| consts | ❌ | - | - |
| storage | ❌ | - | - |
| dispatchables | ❌ | - | - |
| errors | ❌ | - | - |
| events | ❌ | - | - |

#### Block Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| block | ❌ | - | - |
| block-header | ❌ | - | - |
| block-extrinsics | ❌ | - | - |
| block-extrinsics-raw | ✅ | 233241-233255, 10233241-10233255 | 100.0% |
| rc-block-extrinsics-raw | ❌ | - | - |
| block-para-inclusions | ❌ | - | - |

#### Account Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| account-balance-info | ❌ | - | - |

#### Runtime Endpoints

| Endpoint | Status | Result |
|----------|--------|--------|
| runtime-spec | ❌ | - |
| runtime-metadata | ❌ | - |
| tx-material | ❌ | - |
| node-version | ❌ | - |
| node-network | ❌ | - |

### Chain: asset-hub-kusama

- **Total pallets:** 59
- **Last updated:** 2026-01-26T16:01:38.027693+00:00

| Metric | Value |
|--------|-------|
| Endpoints tested | 1/17 |
| Overall pass rate | 100.00% (33/33) |

#### Pallet Endpoints

| Endpoint | Status | Pallets Tested | Pass Rate |
|----------|--------|----------------|------------|
| consts | ❌ | - | - |
| storage | ❌ | - | - |
| dispatchables | ❌ | - | - |
| errors | ❌ | - | - |
| events | ❌ | - | - |

#### Block Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| block | ❌ | - | - |
| block-header | ❌ | - | - |
| block-extrinsics | ❌ | - | - |
| block-extrinsics-raw | ❌ | - | - |
| rc-block-extrinsics-raw | ✅ | 1500200-1500210, 11300200-11300210, 30300200-30300210 | 100.0% |
| block-para-inclusions | ❌ | - | - |

#### Account Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| account-balance-info | ❌ | - | - |

#### Runtime Endpoints

| Endpoint | Status | Result |
|----------|--------|--------|
| runtime-spec | ❌ | - |
| runtime-metadata | ❌ | - |
| tx-material | ❌ | - |
| node-version | ❌ | - |
| node-network | ❌ | - |

## Coverage File Format

Coverage data is stored in JSON format (`coverage/coverage.json`) and can be analyzed programmatically.
