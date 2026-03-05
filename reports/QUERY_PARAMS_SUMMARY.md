# Query Params Coverage Summary

Auto-generated from `--query-params` test runs. Accumulates across runs.

- **Details**: [QUERY_PARAMS_DETAILS.md](QUERY_PARAMS_DETAILS.md)

## Supported Query Params by Endpoint

| Endpoint | Supported Query Params |
|----------|------------------------|
| `block` | eventDocs, extrinsicDocs, noFees, decodedXcmMsgs, paraId, useEvmFormat, finalizedKey, useRcBlock |
| `blocks-head` | eventDocs, extrinsicDocs, noFees, decodedXcmMsgs, paraId, useEvmFormat, finalized, useRcBlock |
| `blocks-head-rcblock` | eventDocs, extrinsicDocs, noFees, decodedXcmMsgs, paraId, useEvmFormat, finalized |
| `blocks-header` | useRcBlock |
| `rc-blocks-blockid` | eventDocs, extrinsicDocs, noFees, decodedXcmMsgs, paraId, useEvmFormat, finalizedKey |
| `rc-blocks-range` | eventDocs, extrinsicDocs, noFees, useEvmFormat |
| `block-extrinsics` | eventDocs, extrinsicDocs, noFees, useEvmFormat |
| `block-extrinsics-raw` | useRcBlock |
| `block-extrinsics-idx` | eventDocs, extrinsicDocs, noFees, useEvmFormat, useRcBlock |
| `block-extrinsics-idx-rcblock` | eventDocs, extrinsicDocs, noFees, useEvmFormat |
| `rc-block-extrinsics-idx` | eventDocs, extrinsicDocs, noFees, useEvmFormat |
| `blocks-para-inclusions` | paraId |
| `account-balance-info` | denominated |
| `rc-account-balance-info` | denominated |
| `account-staking-info` | includeClaimedRewards |
| `pallet-consts` | onlyIds |
| `pallet-consts-item` | metadata |
| `pallet-storage` | onlyIds |
| `rc-pallet-storage` | onlyIds |
| `pallet-dispatchables` | onlyIds |
| `rc-pallet-dispatchables` | onlyIds |
| `pallet-errors` | onlyIds |
| `rc-pallet-errors` | onlyIds |
| `pallet-events` | onlyIds |
| `rc-pallet-events` | onlyIds |
| `tx-material` | noMeta |

## Chain: asset-hub-polkadot

- **Last updated**: 2026-03-05T17:05:01.966577+00:00

| Endpoints with query params tested | 4/26 |
|---|---|

| Endpoint | Query Params | Block Ranges | Pass Rate | Matched | Mismatched | Rust Err | Sidecar Err | Both Err |
|----------|-------------|--------------|-----------|---------|------------|----------|-------------|----------|
| [block-extrinsics-idx](QUERY_PARAMS_DETAILS.md#block-extrinsics-idx---eventDocs-extrinsicDocs-noFees-useEvmFormat-useRcBlock) | eventDocs, extrinsicDocs, noFees, useEvmFormat, useRcBlock | 10500000-10500010, 11500000-11500010, 18500000-18500010, 22500000-22500010 | 57.9% | 44 | 10 | 22 | 0 | 0 |
| [blocks-head](QUERY_PARAMS_DETAILS.md#blocks-head---decodedXcmMsgs-eventDocs-extrinsicDocs-finalized-noFees-paraId=1000-useEvmFormat-useRcBlock) | decodedXcmMsgs, eventDocs, extrinsicDocs, finalized, noFees, paraId=1000, useEvmFormat, useRcBlock | - | 0.0% | 0 | 0 | 3 | 0 | 0 |
| blocks-header | useRcBlock | 11000000-11000010, 25000000-25000010 | 100.0% | 33 | 0 | 0 | 0 | 0 |
| [blocks-para-inclusions](QUERY_PARAMS_DETAILS.md#blocks-para-inclusions---paraId=2000) | paraId=2000 | 10700000-10700010 | 50.0% | 11 | 0 | 0 | 0 | 11 |

