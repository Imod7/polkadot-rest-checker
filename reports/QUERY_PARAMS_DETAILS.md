# Query Params Coverage Details

Detailed breakdown of query param test runs.

- **Summary**: [QUERY_PARAMS_SUMMARY.md](QUERY_PARAMS_SUMMARY.md)

## Chain: asset-hub-polkadot

### block-extrinsics-idx — eventDocs, extrinsicDocs, noFees, useEvmFormat, useRcBlock

- **Block ranges**: 10500000-10500010, 11500000-11500010, 18500000-18500010, 22500000-22500010
- **Last tested**: 2026-03-05T16:50:43.235542+00:00
- **Pass rate**: 57.9%

| Matched | Mismatched | Rust Err | Sidecar Err | Both Err (diff codes) |
|---------|------------|----------|-------------|----------------------|
| 44 | 10 | 22 | 0 | 0 |

#### Issues

- **Block 115000010000**: `MISMATCH [http://localhost:8080/v1/blocks/11500001/extrinsics/0?eventDocs=true&extrinsicDocs=true&noFees=true&useEvmFormat=true&useRcBlock=true] - 1 difference: [0].extrinsics.args.data.horizontalMessages: type mismatch (rust=array vs sidecar=object)`
- **Block 115000030000**: `MISMATCH [http://localhost:8080/v1/blocks/11500003/extrinsics/0?eventDocs=true&extrinsicDocs=true&noFees=true&useEvmFormat=true&useRcBlock=true] - 1 difference: [0].extrinsics.args.data.horizontalMessages: type mismatch (rust=array vs sidecar=object)`
- **Block 115000050000**: `MISMATCH [http://localhost:8080/v1/blocks/11500005/extrinsics/0?eventDocs=true&extrinsicDocs=true&noFees=true&useEvmFormat=true&useRcBlock=true] - 1 difference: [0].extrinsics.args.data.horizontalMessages: type mismatch (rust=array vs sidecar=object)`
- **Block 115000070000**: `MISMATCH [http://localhost:8080/v1/blocks/11500007/extrinsics/0?eventDocs=true&extrinsicDocs=true&noFees=true&useEvmFormat=true&useRcBlock=true] - 1 difference: [0].extrinsics.args.data.horizontalMessages: type mismatch (rust=array vs sidecar=object)`
- **Block 115000090000**: `MISMATCH [http://localhost:8080/v1/blocks/11500009/extrinsics/0?eventDocs=true&extrinsicDocs=true&noFees=true&useEvmFormat=true&useRcBlock=true] - 1 difference: [0].extrinsics.args.data.horizontalMessages: type mismatch (rust=array vs sidecar=object)`
- **Block 22500000**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 22500001**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 22500002**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 22500003**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 22500004**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 22500005**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 22500006**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 22500007**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 22500008**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 22500009**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 22500010**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 18500000**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 18500001**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 18500002**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 18500003**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 18500004**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 18500005**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 18500006**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 18500007**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 18500008**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 18500009**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 18500010**: `Failed to fetch extrinsics: HTTP 500 Internal Server Error`
- **Block 105000010000**: `MISMATCH [http://localhost:8080/v1/blocks/10500001/extrinsics/0?eventDocs=true&extrinsicDocs=true&noFees=true&useEvmFormat=true&useRcBlock=true] - 1 difference: [0].extrinsics.args.data.horizontalMessages: type mismatch (rust=array vs sidecar=object)`
- **Block 105000030000**: `MISMATCH [http://localhost:8080/v1/blocks/10500003/extrinsics/0?eventDocs=true&extrinsicDocs=true&noFees=true&useEvmFormat=true&useRcBlock=true] - 1 difference: [0].extrinsics.args.data.horizontalMessages: type mismatch (rust=array vs sidecar=object)`
- **Block 105000050000**: `MISMATCH [http://localhost:8080/v1/blocks/10500005/extrinsics/0?eventDocs=true&extrinsicDocs=true&noFees=true&useEvmFormat=true&useRcBlock=true] - 1 difference: [0].extrinsics.args.data.horizontalMessages: type mismatch (rust=array vs sidecar=object)`
- **Block 105000070000**: `MISMATCH [http://localhost:8080/v1/blocks/10500007/extrinsics/0?eventDocs=true&extrinsicDocs=true&noFees=true&useEvmFormat=true&useRcBlock=true] - 1 difference: [0].extrinsics.args.data.horizontalMessages: type mismatch (rust=array vs sidecar=object)`
- **Block 105000090000**: `MISMATCH [http://localhost:8080/v1/blocks/10500009/extrinsics/0?eventDocs=true&extrinsicDocs=true&noFees=true&useEvmFormat=true&useRcBlock=true] - 1 difference: [0].extrinsics.args.data.horizontalMessages: type mismatch (rust=array vs sidecar=object)`

### blocks-head — decodedXcmMsgs, eventDocs, extrinsicDocs, finalized, noFees, paraId=1000, useEvmFormat, useRcBlock

- **Block ranges**: -
- **Last tested**: 2026-03-05T17:00:13.382528+00:00
- **Pass rate**: 0.0%

| Matched | Mismatched | Rust Err | Sidecar Err | Both Err (diff codes) |
|---------|------------|----------|-------------|----------------------|
| 0 | 0 | 3 | 0 | 0 |

#### Issues

- **Block 0**: `SIDECAR ERROR: HTTP 500 Internal Server Error`
- **Block 0**: `SIDECAR ERROR: HTTP 500 Internal Server Error`

### blocks-header — useRcBlock

- **Block ranges**: 11000000-11000010, 25000000-25000010
- **Last tested**: 2026-03-05T17:01:30.845902+00:00
- **Pass rate**: 100.0%

| Matched | Mismatched | Rust Err | Sidecar Err | Both Err (diff codes) |
|---------|------------|----------|-------------|----------------------|
| 33 | 0 | 0 | 0 | 0 |

### blocks-para-inclusions — paraId=2000

- **Block ranges**: 10700000-10700010
- **Last tested**: 2026-03-05T17:05:01.966599+00:00
- **Pass rate**: 50.0%

| Matched | Mismatched | Rust Err | Sidecar Err | Both Err (diff codes) |
|---------|------------|----------|-------------|----------------------|
| 11 | 0 | 0 | 0 | 11 |

#### Issues

- **Block 10700000**: `BOTH ERRORS [http://localhost:8080/v1/blocks/10700000/para-inclusions?paraId=2000] (different codes) - Rust: HTTP 400 Bad Request, Sidecar: HTTP 404 Not Found`
- **Block 10700001**: `BOTH ERRORS [http://localhost:8080/v1/blocks/10700001/para-inclusions?paraId=2000] (different codes) - Rust: HTTP 400 Bad Request, Sidecar: HTTP 404 Not Found`
- **Block 10700002**: `BOTH ERRORS [http://localhost:8080/v1/blocks/10700002/para-inclusions?paraId=2000] (different codes) - Rust: HTTP 400 Bad Request, Sidecar: HTTP 404 Not Found`
- **Block 10700003**: `BOTH ERRORS [http://localhost:8080/v1/blocks/10700003/para-inclusions?paraId=2000] (different codes) - Rust: HTTP 400 Bad Request, Sidecar: HTTP 404 Not Found`
- **Block 10700004**: `BOTH ERRORS [http://localhost:8080/v1/blocks/10700004/para-inclusions?paraId=2000] (different codes) - Rust: HTTP 400 Bad Request, Sidecar: HTTP 404 Not Found`
- **Block 10700005**: `BOTH ERRORS [http://localhost:8080/v1/blocks/10700005/para-inclusions?paraId=2000] (different codes) - Rust: HTTP 400 Bad Request, Sidecar: HTTP 404 Not Found`
- **Block 10700006**: `BOTH ERRORS [http://localhost:8080/v1/blocks/10700006/para-inclusions?paraId=2000] (different codes) - Rust: HTTP 400 Bad Request, Sidecar: HTTP 404 Not Found`
- **Block 10700007**: `BOTH ERRORS [http://localhost:8080/v1/blocks/10700007/para-inclusions?paraId=2000] (different codes) - Rust: HTTP 400 Bad Request, Sidecar: HTTP 404 Not Found`
- **Block 10700008**: `BOTH ERRORS [http://localhost:8080/v1/blocks/10700008/para-inclusions?paraId=2000] (different codes) - Rust: HTTP 400 Bad Request, Sidecar: HTTP 404 Not Found`
- **Block 10700009**: `BOTH ERRORS [http://localhost:8080/v1/blocks/10700009/para-inclusions?paraId=2000] (different codes) - Rust: HTTP 400 Bad Request, Sidecar: HTTP 404 Not Found`
- **Block 10700010**: `BOTH ERRORS [http://localhost:8080/v1/blocks/10700010/para-inclusions?paraId=2000] (different codes) - Rust: HTTP 400 Bad Request, Sidecar: HTTP 404 Not Found`

