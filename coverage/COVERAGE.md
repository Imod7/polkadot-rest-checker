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
- **Last updated:** 2026-01-29T15:34:09.480327+00:00

| Metric | Value |
|--------|-------|
| Endpoints tested | 7/29 |
| Overall pass rate | 69.23% (657/949) |

#### Pallet Endpoints

| Endpoint | Status | Pallets Tested | Pass Rate |
|----------|--------|----------------|------------|
| consts | ❌ | - | - |
| consts-item | ❌ | - | - |
| storage | ❌ | - | - |
| dispatchables | ❌ | - | - |
| errors | ✅ | 55/55 | 63.6% |
| events | ❌ | - | - |

#### Block Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| block | ❌ | - | - |
| blocks-head | ❌ | - | - |
| block-header | ❌ | - | - |
| block-extrinsics | ✅ | 2200200-2200210 | 0.0% |
| block-extrinsics-raw | ✅ | 2600200-2600240, 10600200-10600240, 12600200-12600240 | 75.0% |
| block-extrinsics-idx | ❌ | - | - |
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

#### Standalone Endpoints

| Endpoint | Status | Result |
|----------|--------|--------|
| runtime-spec | ❌ | - |
| runtime-metadata | ❌ | - |
| tx-material | ❌ | - |
| node-version | ❌ | - |
| node-network | ❌ | - |
| blocks-head-rcblock | ❌ | - |

#### Detailed Pallet Coverage

**errors:**

| Pallet | Block Ranges | Matched | Mismatched | Errors | Pass Rate |
|--------|--------------|---------|------------|--------|------------|
| AhMigrator | 10294700-10294710 | 0 | 11 | 0 | 0.0% |
| AhOps | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| AssetConversion | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| AssetRate | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| AssetTxPayment | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| Assets | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Aura | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| AuraExt | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| Authorship | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| Balances | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Bounties | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| ChildBounties | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Claims | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| CollatorSelection | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| ConvictionVoting | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| CumulusXcm | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| DelegatedStaking | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| ForeignAssets | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Indices | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| MessageQueue | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| MultiBlockElection | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| MultiBlockElectionSigned | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| MultiBlockElectionUnsigned | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| MultiBlockElectionVerifier | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| Multisig | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Nfts | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| NominationPools | 10294700-10294710 | 0 | 11 | 0 | 0.0% |
| Origins | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| ParachainInfo | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| ParachainSystem | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Parameters | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| PolkadotXcm | 10294700-10294710 | 0 | 11 | 0 | 0.0% |
| PoolAssets | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Preimage | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Proxy | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Referenda | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Revive | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| Scheduler | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Session | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| SnowbridgeSystemFrontend | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Staking | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| StakingRcClient | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| StateTrieMigration | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| System | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Timestamp | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| ToKusamaXcmRouter | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| TransactionPayment | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| Treasury | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Uniques | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Utility | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| Vesting | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| VoterList | 10294700-10294710 | 0 | 11 | 0 | 0.0% |
| WeightReclaim | 10294700-10294710 | 0 | 0 | 11 | 0.0% |
| Whitelist | 10294700-10294710 | 11 | 0 | 0 | 100.0% |
| XcmpQueue | 10294700-10294710 | 11 | 0 | 0 | 100.0% |

### Chain: asset-hub-kusama

- **Total pallets:** 59
- **Last updated:** 2026-02-04T12:56:54.962021+00:00

| Metric | Value |
|--------|-------|
| Endpoints tested | 4/29 |
| Overall pass rate | 74.12% (922/1244) |

#### Pallet Endpoints

| Endpoint | Status | Pallets Tested | Pass Rate |
|----------|--------|----------------|------------|
| consts | ✅ | 59/59 | 69.5% |
| consts-item | ❌ | - | - |
| storage | ❌ | - | - |
| dispatchables | ❌ | - | - |
| errors | ❌ | - | - |
| events | ❌ | - | - |

#### Block Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| block | ❌ | - | - |
| blocks-head | ❌ | - | - |
| block-header | ❌ | - | - |
| block-extrinsics | ❌ | - | - |
| block-extrinsics-raw | ❌ | - | - |
| block-extrinsics-idx | ✅ | 12798138-12798149, 12940347-12940359, 12958046-12958049, 12958582-12958589 | 62.5% |
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
| account-balance-info | ❌ | - | - |

#### Standalone Endpoints

| Endpoint | Status | Result |
|----------|--------|--------|
| runtime-spec | ❌ | - |
| runtime-metadata | ❌ | - |
| tx-material | ❌ | - |
| node-version | ❌ | - |
| node-network | ❌ | - |
| blocks-head-rcblock | ❌ | - |

#### Detailed Pallet Coverage

**consts:**

| Pallet | Block Ranges | Matched | Mismatched | Errors | Pass Rate |
|--------|--------------|---------|------------|--------|------------|
| AhMigrator | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| AhOps | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| AssetConversion | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| AssetRate | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| AssetTxPayment | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| Assets | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Aura | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| AuraExt | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| Authorship | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| Balances | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Bounties | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| ChildBounties | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Claims | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| CollatorSelection | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| ConvictionVoting | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| CumulusXcm | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| DelegatedStaking | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| ForeignAssets | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Indices | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| MessageQueue | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| MultiBlockElection | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| MultiBlockElectionSigned | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| MultiBlockElectionUnsigned | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| MultiBlockElectionVerifier | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| MultiBlockMigrations | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Multisig | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| NftFractionalization | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Nfts | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| NominationPools | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Origins | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| ParachainInfo | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| ParachainSystem | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Parameters | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| PolkadotXcm | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| PoolAssets | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Preimage | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| Proxy | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Recovery | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Referenda | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| RemoteProxyRelayChain | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| Revive | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Scheduler | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Session | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Society | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Staking | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| StakingRcClient | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| StateTrieMigration | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| System | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Timestamp | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| ToPolkadotXcmRouter | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| TransactionPayment | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Treasury | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Uniques | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Utility | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| Vesting | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| VoterList | 11400300-11400310 | 11 | 0 | 0 | 100.0% |
| WeightReclaim | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| Whitelist | 11400300-11400310 | 0 | 0 | 11 | 0.0% |
| XcmpQueue | 11400300-11400310 | 11 | 0 | 0 | 100.0% |

### Chain: kusama

- **Total pallets:** 65
- **Last updated:** 2026-02-05T10:13:41.885902+00:00

| Metric | Value |
|--------|-------|
| Endpoints tested | 6/29 |
| Overall pass rate | 43.27% (964/2228) |

#### Pallet Endpoints

| Endpoint | Status | Pallets Tested | Pass Rate |
|----------|--------|----------------|------------|
| consts | ✅ | 65/65 | 50.8% |
| consts-item | ❌ | - | - |
| storage | ❌ | - | - |
| dispatchables | ✅ | 65/65 | 9.2% |
| errors | ✅ | 65/65 | 69.2% |
| events | ❌ | - | - |

#### Block Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| block | ❌ | - | - |
| blocks-head | ❌ | - | - |
| block-header | ❌ | - | - |
| block-extrinsics | ❌ | - | - |
| block-extrinsics-raw | ✅ | 233241-233255, 10233241-10233255 | 100.0% |
| block-extrinsics-idx | ❌ | - | - |
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

#### Standalone Endpoints

| Endpoint | Status | Result |
|----------|--------|--------|
| runtime-spec | ❌ | - |
| runtime-metadata | ❌ | - |
| tx-material | ❌ | - |
| node-version | ❌ | - |
| node-network | ❌ | - |
| blocks-head-rcblock | ❌ | - |

#### Detailed Pallet Coverage

**consts:**

| Pallet | Block Ranges | Matched | Mismatched | Errors | Pass Rate |
|--------|--------------|---------|------------|--------|------------|
| AssetRate | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Auctions | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| AuthorityDiscovery | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Authorship | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Babe | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Balances | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Beefy | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| BeefyMmrLeaf | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Bounties | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| ChildBounties | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Claims | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Configuration | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| ConvictionVoting | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Coretime | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| CoretimeAssignmentProvider | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Crowdloan | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| DelegatedStaking | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Dmp | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| ElectionProviderMultiPhase | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| FastUnstake | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| FellowshipCollective | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| FellowshipReferenda | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Grandpa | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Historical | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Hrmp | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Indices | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Initializer | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| MessageQueue | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Mmr | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Multisig | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| NominationPools | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Offences | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| OnDemandAssignmentProvider | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Origins | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| ParaInclusion | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| ParaInherent | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| ParaScheduler | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| ParaSessionInfo | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| ParachainsOrigin | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Parameters | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Paras | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| ParasDisputes | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| ParasShared | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| ParasSlashing | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Preimage | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Proxy | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| RcMigrator | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Recovery | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Referenda | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Registrar | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Scheduler | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Session | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| Slots | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Society | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Staking | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| StakingAhClient | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| System | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Timestamp | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| TransactionPayment | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Treasury | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Utility | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Vesting | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| VoterList | 21400300-21400310 | 11 | 0 | 0 | 100.0% |
| Whitelist | 21400300-21400310 | 0 | 0 | 11 | 0.0% |
| XcmPallet | 21400300-21400310 | 0 | 0 | 11 | 0.0% |

**dispatchables:**

| Pallet | Block Ranges | Matched | Mismatched | Errors | Pass Rate |
|--------|--------------|---------|------------|--------|------------|
| AssetRate | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Auctions | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| AuthorityDiscovery | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Authorship | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Babe | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Balances | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Beefy | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| BeefyMmrLeaf | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Bounties | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| ChildBounties | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Claims | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Configuration | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| ConvictionVoting | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Coretime | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| CoretimeAssignmentProvider | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Crowdloan | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| DelegatedStaking | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Dmp | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| ElectionProviderMultiPhase | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| FastUnstake | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| FellowshipCollective | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| FellowshipReferenda | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Grandpa | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Historical | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Hrmp | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Indices | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Initializer | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| MessageQueue | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Mmr | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Multisig | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| NominationPools | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Offences | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| OnDemandAssignmentProvider | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Origins | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| ParaInclusion | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| ParaInherent | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| ParaScheduler | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| ParaSessionInfo | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| ParachainsOrigin | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Parameters | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Paras | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| ParasDisputes | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| ParasShared | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| ParasSlashing | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Preimage | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Proxy | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| RcMigrator | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Recovery | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Referenda | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Registrar | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Scheduler | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Session | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Slots | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Society | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Staking | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| StakingAhClient | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| System | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Timestamp | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| TransactionPayment | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Treasury | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Utility | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Vesting | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| VoterList | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Whitelist | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| XcmPallet | 29205800-29205810 | 0 | 11 | 0 | 0.0% |

**errors:**

| Pallet | Block Ranges | Matched | Mismatched | Errors | Pass Rate |
|--------|--------------|---------|------------|--------|------------|
| AssetRate | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Auctions | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| AuthorityDiscovery | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Authorship | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Babe | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Balances | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Beefy | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| BeefyMmrLeaf | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Bounties | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| ChildBounties | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Claims | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Configuration | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| ConvictionVoting | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Coretime | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| CoretimeAssignmentProvider | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Crowdloan | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| DelegatedStaking | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Dmp | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| ElectionProviderMultiPhase | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| FastUnstake | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| FellowshipCollective | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| FellowshipReferenda | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Grandpa | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Historical | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Hrmp | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Indices | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Initializer | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| MessageQueue | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Mmr | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Multisig | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| NominationPools | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Offences | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| OnDemandAssignmentProvider | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Origins | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| ParaInclusion | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| ParaInherent | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| ParaScheduler | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| ParaSessionInfo | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| ParachainsOrigin | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Parameters | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Paras | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| ParasDisputes | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| ParasShared | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| ParasSlashing | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Preimage | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Proxy | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| RcMigrator | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Recovery | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Referenda | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Registrar | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Scheduler | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Session | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Slots | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Society | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Staking | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| StakingAhClient | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| System | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Timestamp | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| TransactionPayment | 29205800-29205810 | 0 | 0 | 11 | 0.0% |
| Treasury | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Utility | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| Vesting | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| VoterList | 29205800-29205810 | 0 | 11 | 0 | 0.0% |
| Whitelist | 29205800-29205810 | 11 | 0 | 0 | 100.0% |
| XcmPallet | 29205800-29205810 | 11 | 0 | 0 | 100.0% |

### Chain: polkadot

- **Total pallets:** 61
- **Last updated:** 2026-02-05T17:57:23.455794+00:00

| Metric | Value |
|--------|-------|
| Endpoints tested | 11/29 |
| Overall pass rate | 79.70% (4162/5222) |

#### Pallet Endpoints

| Endpoint | Status | Pallets Tested | Pass Rate |
|----------|--------|----------------|------------|
| consts | ✅ | 61/61 | 55.7% |
| consts-item | ✅ | 0/61 | 100.0% |
| storage | ✅ | 61/61 | 90.2% |
| dispatchables | ❌ | - | - |
| errors | ✅ | 61/61 | 57.4% |
| events | ❌ | - | - |

#### Block Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| block | ❌ | - | - |
| blocks-head | ❌ | - | - |
| block-header | ❌ | - | - |
| block-extrinsics | ❌ | - | - |
| block-extrinsics-raw | ✅ | 3233222-3233232, 9233222-9233232, 13233222-13233250, 22233222-22233250, 23233222-23233232 | 100.0% |
| block-extrinsics-idx | ❌ | - | - |
| rc-block-extrinsics-raw | ❌ | - | - |
| rc-block-extrinsics-idx | ❌ | - | - |
| block-para-inclusions | ✅ | 1500200-1500250, 5840200-5840240, 8500200-8500250, 10500200-10500250 | 61.6% |
| staking-validators | ✅ | 10494650-10494660, 20494650-20494660, 28494651-28494660 | 68.8% |
| rc-staking-validators | ❌ | - | - |
| coretime-info | ✅ | 3200700-3200710, 8200700-8200710 | 0.0% |
| coretime-overview | ✅ | 3200700-3200710 | 0.0% |
| coretime-leases | ❌ | - | - |
| coretime-reservations | ❌ | - | - |
| coretime-regions | ❌ | - | - |

#### Account Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| account-balance-info | ✅ | 15233222-15233250, 22233222-22233250, 24233222-24233250 | 91.2% |

#### Standalone Endpoints

| Endpoint | Status | Result |
|----------|--------|--------|
| runtime-spec | ❌ | - |
| runtime-metadata | ❌ | - |
| tx-material | ❌ | - |
| node-version | ❌ | - |
| node-network | ❌ | - |
| blocks-head-rcblock | ✅ | FAIL |

#### Detailed Pallet Coverage

**consts:**

| Pallet | Block Ranges | Matched | Mismatched | Errors | Pass Rate |
|--------|--------------|---------|------------|--------|------------|
| AssetRate | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| Auctions | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| AuthorityDiscovery | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| Authorship | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| Babe | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Balances | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Beefy | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| BeefyMmrLeaf | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| Bounties | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| ChildBounties | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Claims | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Configuration | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| ConvictionVoting | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Coretime | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| CoretimeAssignmentProvider | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| Crowdloan | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| DelegatedStaking | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Dmp | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| ElectionProviderMultiPhase | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| FastUnstake | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Grandpa | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Historical | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| Hrmp | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| Indices | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Initializer | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| MessageQueue | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Mmr | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| Multisig | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| NominationPools | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Offences | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| OnDemand | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Origins | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| ParaInclusion | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| ParaInherent | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| ParaScheduler | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| ParaSessionInfo | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| ParachainsOrigin | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| Paras | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| ParasDisputes | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| ParasShared | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| ParasSlashing | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| Preimage | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| Proxy | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| RcMigrator | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| Referenda | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Registrar | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Scheduler | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Session | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| Slots | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Staking | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| StakingAhClient | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| StateTrieMigration | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| System | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Timestamp | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| TransactionPayment | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Treasury | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Utility | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Vesting | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| VoterList | 25678777-25678787 | 11 | 0 | 0 | 100.0% |
| Whitelist | 25678777-25678787 | 0 | 0 | 11 | 0.0% |
| XcmPallet | 25678777-25678787 | 0 | 0 | 11 | 0.0% |

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

**errors:**

| Pallet | Block Ranges | Matched | Mismatched | Errors | Pass Rate |
|--------|--------------|---------|------------|--------|------------|
| AssetRate | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| Auctions | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| AuthorityDiscovery | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| Authorship | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| Babe | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Balances | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Beefy | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| BeefyMmrLeaf | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| Bounties | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| ChildBounties | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Claims | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Configuration | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| ConvictionVoting | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Coretime | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| CoretimeAssignmentProvider | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| Crowdloan | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| DelegatedStaking | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| Dmp | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| ElectionProviderMultiPhase | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| FastUnstake | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Grandpa | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Historical | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| Hrmp | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Indices | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Initializer | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| MessageQueue | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Mmr | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| Multisig | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| NominationPools | 19294700-19294710 | 0 | 11 | 0 | 0.0% |
| Offences | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| OnDemand | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| Origins | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| ParaInclusion | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| ParaInherent | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| ParaScheduler | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| ParaSessionInfo | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| ParachainsOrigin | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| Paras | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| ParasDisputes | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| ParasShared | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| ParasSlashing | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Preimage | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Proxy | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| RcMigrator | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| Referenda | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Registrar | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Scheduler | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Session | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Slots | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Staking | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| StakingAhClient | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| StateTrieMigration | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| System | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Timestamp | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| TransactionPayment | 19294700-19294710 | 0 | 0 | 11 | 0.0% |
| Treasury | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Utility | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| Vesting | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| VoterList | 19294700-19294710 | 0 | 11 | 0 | 0.0% |
| Whitelist | 19294700-19294710 | 11 | 0 | 0 | 100.0% |
| XcmPallet | 19294700-19294710 | 11 | 0 | 0 | 100.0% |

### Chain: coretime-polkadot

- **Total pallets:** 0
- **Last updated:** 2026-02-01T21:17:58.754673+00:00

| Metric | Value |
|--------|-------|
| Endpoints tested | 3/29 |
| Overall pass rate | 25.00% (6/24) |

#### Pallet Endpoints

| Endpoint | Status | Pallets Tested | Pass Rate |
|----------|--------|----------------|------------|
| consts | ❌ | - | - |
| consts-item | ❌ | - | - |
| storage | ❌ | - | - |
| dispatchables | ❌ | - | - |
| errors | ❌ | - | - |
| events | ❌ | - | - |

#### Block Endpoints

| Endpoint | Status | Block Ranges | Pass Rate |
|----------|--------|--------------|------------|
| block | ❌ | - | - |
| blocks-head | ❌ | - | - |
| block-header | ❌ | - | - |
| block-extrinsics | ❌ | - | - |
| block-extrinsics-raw | ❌ | - | - |
| block-extrinsics-idx | ❌ | - | - |
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
