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

    // Block endpoints
    Block,
    BlockHeader,
    BlockExtrinsics,
    BlockExtrinsicsRaw,
    RcBlockExtrinsicsRaw,
    RcBlockExtrinsicsIdx,
    BlockParaInclusions,

    // Coretime endpoints
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
    PalletDispatchables,
    PalletErrors,
    PalletEvents,
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
            EndpointType::AccountBalanceInfo
             => EndpointCategory::Account,

            EndpointType::PalletConsts
            | EndpointType::PalletStorage
            | EndpointType::PalletDispatchables
            | EndpointType::PalletErrors
            | EndpointType::PalletEvents => EndpointCategory::Pallet,

            // PalletConstsConstantItem is block-based (tests a specific constant across blocks)
            EndpointType::PalletConstsConstantItem => EndpointCategory::Block,

            EndpointType::Block
            | EndpointType::BlockHeader
            | EndpointType::BlockExtrinsics
            | EndpointType::BlockExtrinsicsRaw
            | EndpointType::RcBlockExtrinsicsRaw
            | EndpointType::RcBlockExtrinsicsIdx
            | EndpointType::BlockParaInclusions
            | EndpointType::PalletStakingValidators
            | EndpointType::RcPalletStakingValidators
            | EndpointType::CoretimeLeases
            | EndpointType::CoretimeReservations
            | EndpointType::CoretimeRegions => EndpointCategory::Block,

            EndpointType::RuntimeSpec
            | EndpointType::RuntimeMetadata
            | EndpointType::TransactionMaterial
            | EndpointType::NodeVersion
            | EndpointType::NodeNetwork => EndpointCategory::Standalone,
        }
    }

    /// Build the URL path for this endpoint
    pub fn path(&self, pallet: Option<&str>, block: Option<u32>) -> String {
        self.path_with_account(pallet, block, None)
    }

    /// Build the URL path for this endpoint with optional account address
    pub fn path_with_account(&self, pallet: Option<&str>, block: Option<u32>, account: Option<&str>) -> String {
        self.path_with_extrinsic(pallet, block, account, None)
    }

    /// Build the URL path for this endpoint with optional extrinsic index
    pub fn path_with_extrinsic(&self, pallet: Option<&str>, block: Option<u32>, account: Option<&str>, extrinsic_index: Option<u32>) -> String {
        match self {
            // Account endpoints
            EndpointType::AccountBalanceInfo => {
                let account = account.expect("Account required for AccountBalanceInfo endpoint");
                match block {
                    Some(b) => format!("/accounts/{}/balance-info?at={}", account, b),
                    None => format!("/accounts/{}/balance-info", account),
                }
            }

            // Block endpoints
            EndpointType::Block => {
                let block = block.expect("Block required for Block endpoint");
                format!("/blocks/{}", block)
            }
            EndpointType::BlockHeader => {
                let block = block.expect("Block required for BlockHeader endpoint");
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
            EndpointType::RcBlockExtrinsicsRaw => {
                let block = block.expect("Relay Chain Block required for RcBlockExtrinsicsRaw endpoint");
                format!("/rc/blocks/{}/extrinsics-raw", block)
            }
            EndpointType::RcBlockExtrinsicsIdx => {
                let block = block.expect("Relay Chain Block required for RcBlockExtrinsicsIdx endpoint");
                let idx = extrinsic_index.expect("Extrinsic index required for RcBlockExtrinsicsIdx endpoint");
                format!("/rc/blocks/{}/extrinsics/{}", block, idx)
            }
            EndpointType::BlockParaInclusions => {
                let block = block.expect("Block required for BlockParaInclusions endpoint");
                format!("/blocks/{}/para-inclusions", block)
            }

            // Coretime endpoints
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
                    Some(b) => format!("/pallets/{}/consts/{}?at={}", pallet_name, constant_name, b),
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
            EndpointType::PalletDispatchables => {
                let pallet = pallet.expect("Pallet required for PalletDispatchables");
                match block {
                    Some(b) => format!("/pallets/{}/dispatchables?at={}", pallet, b),
                    None => format!("/pallets/{}/dispatchables", pallet),
                }
            }
            EndpointType::PalletErrors => {
                let pallet = pallet.expect("Pallet required for PalletErrors");
                match block {
                    Some(b) => format!("/pallets/{}/errors?at={}", pallet, b),
                    None => format!("/pallets/{}/errors", pallet),
                }
            }
            EndpointType::PalletEvents => {
                let pallet = pallet.expect("Pallet required for PalletEvents");
                match block {
                    Some(b) => format!("/pallets/{}/events?at={}", pallet, b),
                    None => format!("/pallets/{}/events", pallet),
                }
            }
            EndpointType::PalletStakingValidators => {
                match block {
                    Some(b) => format!("/pallets/staking/validators?at={}", b),
                    None => "/pallets/staking/validators".to_string(),
                }
            }
            EndpointType::RcPalletStakingValidators => {
                match block {
                    Some(b) => format!("/rc/pallets/staking/validators?at={}", b),
                    None => "/rc/pallets/staking/validators".to_string(),
                }
            }

            // Runtime endpoints
            EndpointType::RuntimeSpec => {
                match block {
                    Some(b) => format!("/runtime/spec?at={}", b),
                    None => "/runtime/spec".to_string(),
                }
            }
            EndpointType::RuntimeMetadata => {
                match block {
                    Some(b) => format!("/runtime/metadata?at={}", b),
                    None => "/runtime/metadata".to_string(),
                }
            }

            // Transaction endpoints
            EndpointType::TransactionMaterial => {
                match block {
                    Some(b) => format!("/transaction/material?at={}", b),
                    None => "/transaction/material".to_string(),
                }
            }
        }
    }

    /// Get a short name for this endpoint (used in filenames)
    pub fn short_name(&self) -> &'static str {
        match self {
            EndpointType::AccountBalanceInfo => "account-balance-info",
            EndpointType::Block => "block",
            EndpointType::BlockHeader => "block-header",
            EndpointType::BlockExtrinsics => "block-extrinsics",
            EndpointType::BlockExtrinsicsRaw => "block-extrinsics-raw",
            EndpointType::RcBlockExtrinsicsRaw => "rc-block-extrinsics-raw",
            EndpointType::RcBlockExtrinsicsIdx => "rc-block-extrinsics-idx",
            EndpointType::BlockParaInclusions => "block-para-inclusions",
            EndpointType::CoretimeLeases => "coretime-leases",
            EndpointType::CoretimeReservations => "coretime-reservations",
            EndpointType::CoretimeRegions => "coretime-regions",
            EndpointType::NodeVersion => "node-version",
            EndpointType::NodeNetwork => "node-network",
            EndpointType::PalletConsts => "consts",
            EndpointType::PalletConstsConstantItem => "consts-item",
            EndpointType::PalletStorage => "storage",
            EndpointType::PalletDispatchables => "dispatchables",
            EndpointType::PalletErrors => "errors",
            EndpointType::PalletEvents => "events",
            EndpointType::PalletStakingValidators => "staking-validators",
            EndpointType::RcPalletStakingValidators => "rc-staking-validators",
            EndpointType::RuntimeSpec => "runtime-spec",
            EndpointType::RuntimeMetadata => "runtime-metadata",
            EndpointType::TransactionMaterial => "tx-material",
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
}

impl fmt::Display for EndpointType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EndpointType::AccountBalanceInfo => write!(f, "account-balance-info"),
            EndpointType::Block => write!(f, "block"),
            EndpointType::BlockHeader => write!(f, "block-header"),
            EndpointType::BlockExtrinsics => write!(f, "block-extrinsics"),
            EndpointType::BlockExtrinsicsRaw => write!(f, "block-extrinsics-raw"),
            EndpointType::RcBlockExtrinsicsRaw => write!(f, "rc-block-extrinsics-raw"),
            EndpointType::RcBlockExtrinsicsIdx => write!(f, "rc-block-extrinsics-idx"),
            EndpointType::BlockParaInclusions => write!(f, "block-para-inclusions"),
            EndpointType::CoretimeLeases => write!(f, "coretime-leases"),
            EndpointType::CoretimeReservations => write!(f, "coretime-reservations"),
            EndpointType::CoretimeRegions => write!(f, "coretime-regions"),
            EndpointType::NodeVersion => write!(f, "node-version"),
            EndpointType::NodeNetwork => write!(f, "node-network"),
            EndpointType::PalletConsts => write!(f, "pallet-consts"),
            EndpointType::PalletConstsConstantItem => write!(f, "pallet-consts-item"),
            EndpointType::PalletStorage => write!(f, "pallet-storage"),
            EndpointType::PalletDispatchables => write!(f, "pallet-dispatchables"),
            EndpointType::PalletErrors => write!(f, "pallet-errors"),
            EndpointType::PalletEvents => write!(f, "pallet-events"),
            EndpointType::PalletStakingValidators => write!(f, "staking-validators"),
            EndpointType::RcPalletStakingValidators => write!(f, "rc-staking-validators"),
            EndpointType::RuntimeSpec => write!(f, "runtime-spec"),
            EndpointType::RuntimeMetadata => write!(f, "runtime-metadata"),
            EndpointType::TransactionMaterial => write!(f, "tx-material"),
        }
    }
}

impl std::str::FromStr for EndpointType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            // Account endpoints
            "account-balance-info" | "accounts-balance-info" => Ok(EndpointType::AccountBalanceInfo),

            // Block endpoints
            "block" | "blocks" => Ok(EndpointType::Block),
            "block-header" | "header" => Ok(EndpointType::BlockHeader),
            "block-extrinsics" | "extrinsics" => Ok(EndpointType::BlockExtrinsics),
            "block-extrinsics-raw" => Ok(EndpointType::BlockExtrinsicsRaw),
            "rc-block-extrinsics-raw" => Ok(EndpointType::RcBlockExtrinsicsRaw),
            "rc-block-extrinsics-idx" => Ok(EndpointType::RcBlockExtrinsicsIdx),
            "block-para-inclusions" | "para-inclusions" => Ok(EndpointType::BlockParaInclusions),

            // Coretime endpoints
            "coretime-leases" => Ok(EndpointType::CoretimeLeases),
            "coretime-reservations" => Ok(EndpointType::CoretimeReservations),
            "coretime-regions" | "core-reg" => Ok(EndpointType::CoretimeRegions),

            // Node endpoints
            "node-version" | "version" => Ok(EndpointType::NodeVersion),
            "node-network" | "network" => Ok(EndpointType::NodeNetwork),

            // Pallet endpoints
            "consts" | "pallet-consts" => Ok(EndpointType::PalletConsts),
            "consts-item" | "pallet-consts-item" => Ok(EndpointType::PalletConstsConstantItem),
            "storage" | "pallet-storage" => Ok(EndpointType::PalletStorage),
            "dispatchables" | "pallet-dispatchables" => Ok(EndpointType::PalletDispatchables),
            "errors" | "pallet-errors" => Ok(EndpointType::PalletErrors),
            "events" | "pallet-events" => Ok(EndpointType::PalletEvents),
            "staking-validators" => Ok(EndpointType::PalletStakingValidators),
            "rc-staking-validators" => Ok(EndpointType::RcPalletStakingValidators),

            // Runtime endpoints
            "runtime-spec" | "spec" => Ok(EndpointType::RuntimeSpec),
            "runtime-metadata" | "metadata" => Ok(EndpointType::RuntimeMetadata),
            "tx-material" | "transaction-material" => Ok(EndpointType::TransactionMaterial),

            _ => Err(format!(
                "Unknown endpoint '{}'. Valid options:\n  Account: balance-info\n Block: block, block-header, block-extrinsics, para-inclusions\n Pallet: consts, storage, dispatchables, errors, events\n  Runtime: runtime-spec, runtime-metadata, tx-material\n  Node: node-version, node-network",
                s
            )),
        }
    }
}
