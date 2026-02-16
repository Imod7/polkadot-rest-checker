//! Endpoint definitions for testing different API routes.

use std::fmt;

/// Category of endpoint for CLI selection
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndpointCategory {
    /// Account endpoints (require an account address)
    Account,
    /// Block-based endpoints (only need block numbers)
    Block,
    /// Pallet-based endpoints (require iterating over pallets)
    Pallet,
    /// Standalone endpoints (metadata, spec, etc.)
    Standalone,
}

/// Specific endpoint type to test
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EndpointType {
    // Account endpoints
    AccountBalanceInfo,
    AccountForeignAssetBalances,
    AccountStakingPayouts,
    AccountStakingInfo,

    // Block endpoints
    Block,
    BlocksHead,
    BlocksHeadRcBlock,
    BlocksHeader,
    BlockExtrinsics,
    BlockExtrinsicsRaw,
    BlockExtrinsicsRawRcBlock,
    BlockExtrinsicsIdx,
    BlockExtrinsicsIdxRcBlock,
    RcBlockExtrinsicsRaw,
    RcBlockExtrinsicsIdx,
    BlockParaInclusions,

    // Coretime endpoints
    CoretimeInfo,
    CoretimeOverview,
    CoretimeLeases,
    CoretimeRegions,
    CoretimeReservations,

    // Node endpoints
    NodeVersion,
    NodeNetwork,

    // Pallet endpoints
    PalletConsts,
    PalletConstsConstantItem,
    PalletStorage,
    RcPalletStorage,
    PalletDispatchables,
    RcPalletDispatchables,
    PalletErrors,
    RcPalletErrors,
    PalletEvents,
    RcPalletEvents,
    PalletStakingValidators,
    RcPalletStakingValidators,

    // Runtime endpoints
    RuntimeSpec,
    RuntimeMetadata,

    // Transaction endpoints
    TransactionMaterial,
}

impl EndpointType {
    /// Get the category of this endpoint
    pub fn category(&self) -> EndpointCategory {
        match self {
            EndpointType::AccountBalanceInfo => EndpointCategory::Account,
            EndpointType::AccountForeignAssetBalances => EndpointCategory::Account,
            EndpointType::AccountStakingPayouts => EndpointCategory::Account,
            EndpointType::AccountStakingInfo => EndpointCategory::Account,

            EndpointType::PalletConsts
            | EndpointType::PalletStorage
            | EndpointType::RcPalletStorage
            | EndpointType::PalletDispatchables
            | EndpointType::RcPalletDispatchables
            | EndpointType::PalletErrors
            | EndpointType::RcPalletErrors
            | EndpointType::PalletEvents 
            | EndpointType::RcPalletEvents => EndpointCategory::Pallet,

            // PalletConstsConstantItem is block-based (tests a specific constant across blocks)
            EndpointType::PalletConstsConstantItem => EndpointCategory::Block,

            EndpointType::Block
            | EndpointType::BlocksHeader
            | EndpointType::BlockExtrinsics
            | EndpointType::BlockExtrinsicsRaw
            | EndpointType::BlockExtrinsicsRawRcBlock
            | EndpointType::BlockExtrinsicsIdx
            | EndpointType::BlockExtrinsicsIdxRcBlock
            | EndpointType::RcBlockExtrinsicsRaw
            | EndpointType::RcBlockExtrinsicsIdx
            | EndpointType::BlockParaInclusions
            | EndpointType::PalletStakingValidators
            | EndpointType::RcPalletStakingValidators
            | EndpointType::CoretimeInfo
            | EndpointType::CoretimeOverview
            | EndpointType::CoretimeLeases
            | EndpointType::CoretimeReservations
            | EndpointType::CoretimeRegions => EndpointCategory::Block,

            EndpointType::RuntimeSpec
            | EndpointType::RuntimeMetadata
            | EndpointType::TransactionMaterial
            | EndpointType::NodeVersion
            | EndpointType::NodeNetwork
            | EndpointType::BlocksHeadRcBlock
            | EndpointType::BlocksHead => EndpointCategory::Standalone,
        }
    }

    /// Build the URL path for this endpoint
    pub fn path(&self, pallet: Option<&str>, block: Option<u32>) -> String {
        self.path_with_account(pallet, block, None)
    }

    /// Build the URL path for this endpoint with optional account address
    pub fn path_with_account(
        &self,
        pallet: Option<&str>,
        block: Option<u32>,
        account: Option<&str>,
    ) -> String {
        self.path_with_extrinsic(pallet, block, account, None)
    }

    /// Build the URL path for this endpoint with optional extrinsic index
    pub fn path_with_extrinsic(
        &self,
        pallet: Option<&str>,
        block: Option<u32>,
        account: Option<&str>,
        extrinsic_index: Option<u32>,
    ) -> String {
        match self {
            // Account endpoints
            EndpointType::AccountBalanceInfo => {
                let account = account.expect("Account required for AccountBalanceInfo endpoint");
                match block {
                    Some(b) => format!("/accounts/{}/balance-info?at={}", account, b),
                    None => format!("/accounts/{}/balance-info", account),
                }
            }
            EndpointType::AccountForeignAssetBalances => {
                let account = account.expect("Account required for AccountForeignAssetBalances endpoint");
                match block {
                    Some(b) => format!("/accounts/{}/foreign-asset-balances?at={}", account, b),
                    None => format!("/accounts/{}/foreign-asset-balances", account),
                }
            }
            EndpointType::AccountStakingPayouts => {
                let account = account.expect("Account required for AccountStakingPayouts endpoint");
                match block {
                    Some(b) => format!("/accounts/{}/staking-payouts?at={}", account, b),
                    None => format!("/accounts/{}/staking-payouts", account),
                }
            }
            EndpointType::AccountStakingInfo => {
                let account = account.expect("Account required for AccountStakingInfo endpoint");
                match block {
                    Some(b) => format!("/accounts/{}/staking-info?at={}", account, b),
                    None => format!("/accounts/{}/staking-info", account),
                }
            }

            // Block endpoints
            EndpointType::Block => {
                let block = block.expect("Block required for Block endpoint");
                format!("/blocks/{}", block)
            }
            EndpointType::BlocksHead => "/blocks/head".to_string(),
            EndpointType::BlocksHeadRcBlock => "/blocks/head?useRcBlock=true".to_string(),
            EndpointType::BlocksHeader => {
                let block = block.expect("Block required for BlocksHeader endpoint");
                format!("/blocks/{}/header", block)
            }
            EndpointType::BlockExtrinsics => {
                let block = block.expect("Block required for BlockExtrinsics endpoint");
                format!("/blocks/{}/extrinsics-info", block)
            }
            EndpointType::BlockExtrinsicsRaw => {
                let block = block.expect("Block required for BlockExtrinsicsRaw endpoint");
                format!("/blocks/{}/extrinsics-raw", block)
            }
            EndpointType::BlockExtrinsicsRawRcBlock => {
                let block = block.expect("Block required for BlockExtrinsicsRawRcBlock endpoint");
                format!("/blocks/{}/extrinsics-raw?useRcBlock=true", block)
            }
            EndpointType::BlockExtrinsicsIdx => {
                let block =
                    block.expect("Block required for BlockExtrinsicsIdx endpoint");
                let idx = extrinsic_index
                    .expect("Extrinsic index required for BlockExtrinsicsIdx endpoint");
                format!("/blocks/{}/extrinsics/{}", block, idx)
            }
            EndpointType::BlockExtrinsicsIdxRcBlock => {
                let block =
                    block.expect("Block required for BlockExtrinsicsIdxRcBlock endpoint");
                let idx = extrinsic_index
                    .expect("Extrinsic index required for BlockExtrinsicsIdxRcBlock endpoint");
                format!("/blocks/{}/extrinsics/{}?useRcBlock=true", block, idx)
            }
            EndpointType::RcBlockExtrinsicsRaw => {
                let block =
                    block.expect("Relay Chain Block required for RcBlockExtrinsicsRaw endpoint");
                format!("/rc/blocks/{}/extrinsics-raw", block)
            }
            EndpointType::RcBlockExtrinsicsIdx => {
                let block =
                    block.expect("Relay Chain Block required for RcBlockExtrinsicsIdx endpoint");
                let idx = extrinsic_index
                    .expect("Extrinsic index required for RcBlockExtrinsicsIdx endpoint");
                format!("/rc/blocks/{}/extrinsics/{}", block, idx)
            }
            EndpointType::BlockParaInclusions => {
                let block = block.expect("Block required for BlockParaInclusions endpoint");
                format!("/blocks/{}/para-inclusions", block)
            }

            // Coretime endpoints
            EndpointType::CoretimeInfo => {
                let block = block.expect("Block required for CoretimeInfo endpoint");
                format!("/coretime/info?at={}", block)
            }
            EndpointType::CoretimeOverview => {
                let block = block.expect("Block required for CoretimeOverview endpoint");
                format!("/coretime/overview?at={}", block)
            }
            EndpointType::CoretimeLeases => {
                let block = block.expect("Block required for CoretimeLeases endpoint");
                format!("/coretime/leases?at={}", block)
            }
            EndpointType::CoretimeReservations => {
                let block = block.expect("Block required for CoretimeReservations endpoint");
                format!("/coretime/reservations?at={}", block)
            }
            EndpointType::CoretimeRegions => {
                let block = block.expect("Block required for CoretimeRegions endpoint");
                format!("/coretime/regions?at={}", block)
            }

            // Node endpoints
            EndpointType::NodeVersion => "/node/version".to_string(),
            EndpointType::NodeNetwork => "/node/network".to_string(),

            // Pallet endpoints
            EndpointType::PalletConsts => {
                let pallet = pallet.expect("Pallet required for PalletConsts");
                match block {
                    Some(b) => format!("/pallets/{}/consts?at={}", pallet, b),
                    None => format!("/pallets/{}/consts", pallet),
                }
            }
            EndpointType::PalletConstsConstantItem => {
                // Expects pallet in format "PalletName/ConstantName" (e.g., "System/BlockHashCount")
                let pallet_const = pallet.expect("Pallet/Constant required for PalletConstsConstantItem (format: PalletName/ConstantName)");
                let parts: Vec<&str> = pallet_const.splitn(2, '/').collect();
                if parts.len() != 2 {
                    panic!("PalletConstsConstantItem requires format 'PalletName/ConstantName', got: {}", pallet_const);
                }
                let pallet_name = parts[0];
                let constant_name = parts[1];
                match block {
                    Some(b) => {
                        format!("/pallets/{}/consts/{}?at={}", pallet_name, constant_name, b)
                    }
                    None => format!("/pallets/{}/consts/{}", pallet_name, constant_name),
                }
            }
            EndpointType::PalletStorage => {
                let pallet = pallet.expect("Pallet required for PalletStorage");
                match block {
                    Some(b) => format!("/pallets/{}/storage?at={}", pallet, b),
                    None => format!("/pallets/{}/storage", pallet),
                }
            }
            EndpointType::RcPalletStorage => {
                let pallet = pallet.expect("Pallet required for RcPalletStorage");
                match block {
                    Some(b) => format!("/rc/pallets/{}/storage?at={}", pallet, b),
                    None => format!("/rc/pallets/{}/storage", pallet),
                }
            }
            EndpointType::PalletDispatchables => {
                let pallet = pallet.expect("Pallet required for PalletDispatchables");
                match block {
                    Some(b) => format!("/pallets/{}/dispatchables?at={}", pallet, b),
                    None => format!("/pallets/{}/dispatchables", pallet),
                }
            }
            EndpointType::RcPalletDispatchables => {
                let pallet = pallet.expect("Pallet required for RcPalletDispatchables");
                match block {
                    Some(b) => format!("/rc/pallets/{}/dispatchables?at={}", pallet, b),
                    None => format!("/rc/pallets/{}/dispatchables", pallet),
                }
            }
            EndpointType::PalletErrors => {
                let pallet = pallet.expect("Pallet required for PalletErrors");
                match block {
                    Some(b) => format!("/pallets/{}/errors?at={}", pallet, b),
                    None => format!("/pallets/{}/errors", pallet),
                }
            }
            EndpointType::RcPalletErrors => {
                let pallet = pallet.expect("Pallet required for RcPalletErrors");
                match block {
                    Some(b) => format!("/rc/pallets/{}/errors?at={}", pallet, b),
                    None => format!("/rc/pallets/{}/errors", pallet),
                }
            }
            EndpointType::PalletEvents => {
                let pallet = pallet.expect("Pallet required for PalletEvents");
                match block {
                    Some(b) => format!("/pallets/{}/events?at={}", pallet, b),
                    None => format!("/pallets/{}/events", pallet),
                }
            }
            EndpointType::RcPalletEvents => {
                let pallet = pallet.expect("Pallet required for RcPalletEvents");
                match block {
                    Some(b) => format!("/rc/pallets/{}/events?at={}", pallet, b),
                    None => format!("/rc/pallets/{}/events", pallet),
                }
            }
            EndpointType::PalletStakingValidators => match block {
                Some(b) => format!("/pallets/staking/validators?at={}", b),
                None => "/pallets/staking/validators".to_string(),
            },
            EndpointType::RcPalletStakingValidators => match block {
                Some(b) => format!("/rc/pallets/staking/validators?at={}", b),
                None => "/rc/pallets/staking/validators".to_string(),
            },

            // Runtime endpoints
            EndpointType::RuntimeSpec => match block {
                Some(b) => format!("/runtime/spec?at={}", b),
                None => "/runtime/spec".to_string(),
            },
            EndpointType::RuntimeMetadata => match block {
                Some(b) => format!("/runtime/metadata?at={}", b),
                None => "/runtime/metadata".to_string(),
            },

            // Transaction endpoints
            EndpointType::TransactionMaterial => match block {
                Some(b) => format!("/transaction/material?at={}", b),
                None => "/transaction/material".to_string(),
            },
        }
    }

    /// Get the URL path pattern with placeholders (e.g. `/blocks/{blockId}/extrinsics/{index}`)
    pub fn path_pattern(&self) -> &'static str {
        match self {
            EndpointType::AccountBalanceInfo => "/accounts/{accountId}/balance-info",
            EndpointType::AccountForeignAssetBalances => "/accounts/{accountId}/foreign-asset-balances",
            EndpointType::AccountStakingPayouts => "/accounts/{accountId}/staking-payouts",
            EndpointType::AccountStakingInfo => "/accounts/{accountId}/staking-info",
            EndpointType::Block => "/blocks/{blockId}",
            EndpointType::BlocksHead => "/blocks/head",
            EndpointType::BlocksHeadRcBlock => "/blocks/head?useRcBlock=true",
            EndpointType::BlocksHeader => "/blocks/{blockId}/header",
            EndpointType::BlockExtrinsics => "/blocks/{blockId}/extrinsics",
            EndpointType::BlockExtrinsicsRaw => "/blocks/{blockId}/extrinsics-raw",
            EndpointType::BlockExtrinsicsRawRcBlock => "/blocks/{blockId}/extrinsics-raw?useRcBlock=true",
            EndpointType::BlockExtrinsicsIdx => "/blocks/{blockId}/extrinsics/{index}",
            EndpointType::BlockExtrinsicsIdxRcBlock => "/blocks/{blockId}/extrinsics/{index}?useRcBlock=true",
            EndpointType::RcBlockExtrinsicsRaw => "/rc/blocks/{blockId}/extrinsics-raw",
            EndpointType::RcBlockExtrinsicsIdx => "/rc/blocks/{blockId}/extrinsics/{index}",
            EndpointType::BlockParaInclusions => "/blocks/{blockId}/para-inclusions",
            EndpointType::CoretimeInfo => "/coretime/info",
            EndpointType::CoretimeOverview => "/coretime/overview",
            EndpointType::CoretimeLeases => "/coretime/leases",
            EndpointType::CoretimeReservations => "/coretime/reservations",
            EndpointType::CoretimeRegions => "/coretime/regions",
            EndpointType::NodeVersion => "/node/version",
            EndpointType::NodeNetwork => "/node/network",
            EndpointType::PalletConsts => "/pallets/{palletId}/consts",
            EndpointType::PalletConstsConstantItem => "/pallets/{palletId}/consts/{constantId}",
            EndpointType::PalletStorage => "/pallets/{palletId}/storage",
            EndpointType::RcPalletStorage => "/rc/pallets/{palletId}/storage",
            EndpointType::PalletDispatchables => "/pallets/{palletId}/dispatchables",
            EndpointType::RcPalletDispatchables => "/rc/pallets/{palletId}/dispatchables",
            EndpointType::PalletErrors => "/pallets/{palletId}/errors",
            EndpointType::RcPalletErrors => "/rc/pallets/{palletId}/errors",
            EndpointType::PalletEvents => "/pallets/{palletId}/events",
            EndpointType::RcPalletEvents => "/rc/pallets/{palletId}/events",
            EndpointType::PalletStakingValidators => "/pallets/staking/validators",
            EndpointType::RcPalletStakingValidators => "/rc/pallets/staking/validators",
            EndpointType::RuntimeSpec => "/runtime/spec",
            EndpointType::RuntimeMetadata => "/runtime/metadata",
            EndpointType::TransactionMaterial => "/transaction/material",
        }
    }

    /// Check if this endpoint requires iterating over pallets
    pub fn requires_pallet(&self) -> bool {
        self.category() == EndpointCategory::Pallet
    }

    /// Check if this endpoint requires iterating over blocks
    pub fn requires_block(&self) -> bool {
        matches!(
            self.category(),
            EndpointCategory::Pallet | EndpointCategory::Block | EndpointCategory::Account
        )
    }

    /// Check if this endpoint requires iterating over accounts
    pub fn requires_account(&self) -> bool {
        self.category() == EndpointCategory::Account
    }

    /// Check if this is a staking-related account endpoint (needs stash accounts)
    pub fn is_staking(&self) -> bool {
        matches!(
            self,
            EndpointType::AccountStakingPayouts | EndpointType::AccountStakingInfo
        )
    }
}

/// Each entry: (variant, canonical name, aliases)
const ENDPOINT_NAMES: &[(fn() -> EndpointType, &str, &[&str])] = &[
    // Account
    (|| EndpointType::AccountBalanceInfo, "account-balance-info", &["accounts-balance-info"]),
    (|| EndpointType::AccountForeignAssetBalances, "account-foreign-asset-balance", &["account-fa-bl"]),
    (|| EndpointType::AccountStakingPayouts, "account-staking-payouts", &["account-sp"]),
    (|| EndpointType::AccountStakingInfo, "account-staking-info", &["account-info"]),
    // Block
    (|| EndpointType::Block, "block", &["blocks"]),
    (|| EndpointType::BlocksHead, "blocks-head", &[]),
    (|| EndpointType::BlocksHeadRcBlock, "blocks-head-rcblock", &["blocks-head-rc"]),
    (|| EndpointType::BlocksHeader, "blocks-header", &["header"]),
    (|| EndpointType::BlockExtrinsics, "block-extrinsics", &["extrinsics"]),
    (|| EndpointType::BlockExtrinsicsRaw, "block-extrinsics-raw", &[]),
    (|| EndpointType::BlockExtrinsicsRawRcBlock, "block-extrinsics-raw-rcblock", &["block-extrinsics-raw-rc"]),
    (|| EndpointType::BlockExtrinsicsIdx, "block-extrinsics-idx", &[]),
    (|| EndpointType::BlockExtrinsicsIdxRcBlock, "block-extrinsics-idx-rcblock", &["block-extrinsics-idx-rc"]),
    (|| EndpointType::RcBlockExtrinsicsRaw, "rc-block-extrinsics-raw", &[]),
    (|| EndpointType::RcBlockExtrinsicsIdx, "rc-block-extrinsics-idx", &[]),
    (|| EndpointType::BlockParaInclusions, "block-para-inclusions", &["para-inclusions"]),
    // Coretime
    (|| EndpointType::CoretimeInfo, "coretime-info", &[]),
    (|| EndpointType::CoretimeOverview, "coretime-overview", &[]),
    (|| EndpointType::CoretimeLeases, "coretime-leases", &[]),
    (|| EndpointType::CoretimeReservations, "coretime-reservations", &[]),
    (|| EndpointType::CoretimeRegions, "coretime-regions", &["core-reg"]),
    // Node
    (|| EndpointType::NodeVersion, "node-version", &["version"]),
    (|| EndpointType::NodeNetwork, "node-network", &["network"]),
    // Pallet
    (|| EndpointType::PalletConsts, "pallet-consts", &["consts"]),
    (|| EndpointType::PalletConstsConstantItem, "pallet-consts-item", &["consts-item"]),
    (|| EndpointType::PalletStorage, "pallet-storage", &["storage"]),
    (|| EndpointType::RcPalletStorage, "rc-pallet-storage", &[]),
    (|| EndpointType::PalletDispatchables, "pallet-dispatchables", &["dispatchables"]),
    (|| EndpointType::RcPalletDispatchables, "rc-pallet-dispatchables", &[]),
    (|| EndpointType::PalletErrors, "pallet-errors", &["errors"]),
    (|| EndpointType::RcPalletErrors, "rc-pallet-errors", &[]),
    (|| EndpointType::PalletEvents, "pallet-events", &[]),
    (|| EndpointType::RcPalletEvents, "rc-pallet-events", &[]),
    (|| EndpointType::PalletStakingValidators, "staking-validators", &[]),
    (|| EndpointType::RcPalletStakingValidators, "rc-staking-validators", &[]),
    // Runtime
    (|| EndpointType::RuntimeSpec, "runtime-spec", &["spec"]),
    (|| EndpointType::RuntimeMetadata, "runtime-metadata", &["metadata"]),
    // Transaction
    (|| EndpointType::TransactionMaterial, "tx-material", &["transaction-material"]),
];

impl EndpointType {
    /// Get the canonical string name for this endpoint.
    pub fn name(&self) -> &'static str {
        for (constructor, canonical, _) in ENDPOINT_NAMES {
            if constructor() == *self {
                return canonical;
            }
        }
        unreachable!("All variants must be in ENDPOINT_NAMES")
    }
}

impl fmt::Display for EndpointType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl std::str::FromStr for EndpointType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        for (constructor, canonical, aliases) in ENDPOINT_NAMES {
            if *canonical == lower || aliases.contains(&lower.as_str()) {
                return Ok(constructor());
            }
        }
        Err(format!(
            "Unknown endpoint '{}'. Valid options:\n  Account: account-balance-info, account-foreign-asset-balance\n  Block: block, blocks-header, block-extrinsics, para-inclusions\n  Pallet: pallet-consts, pallet-storage, pallet-dispatchables, pallet-errors, pallet-events\n  Runtime: runtime-spec, runtime-metadata, tx-material\n  Node: node-version, node-network",
            s
        ))
    }
}
