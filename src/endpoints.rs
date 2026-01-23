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
    /// Runtime endpoints (metadata, spec, etc.)
    Runtime,
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
    BlockParaInclusions,

    // Node endpoints
    NodeVersion,
    NodeNetwork,

    // Pallet endpoints
    PalletConsts,
    PalletStorage,
    PalletDispatchables,
    PalletErrors,
    PalletEvents,

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

            EndpointType::Block
            | EndpointType::BlockHeader
            | EndpointType::BlockExtrinsics
            | EndpointType::BlockParaInclusions => EndpointCategory::Block,

            EndpointType::RuntimeSpec
            | EndpointType::RuntimeMetadata
            | EndpointType::TransactionMaterial
            | EndpointType::NodeVersion
            | EndpointType::NodeNetwork => EndpointCategory::Runtime,
        }
    }

    /// Build the URL path for this endpoint
    pub fn path(&self, pallet: Option<&str>, block: Option<u32>) -> String {
        self.path_with_account(pallet, block, None)
    }

    /// Build the URL path for this endpoint with optional account address
    pub fn path_with_account(&self, pallet: Option<&str>, block: Option<u32>, account: Option<&str>) -> String {
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
            EndpointType::BlockParaInclusions => {
                let block = block.expect("Block required for BlockParaInclusions endpoint");
                format!("/blocks/{}/para-inclusions", block)
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
            EndpointType::BlockParaInclusions => "block-para-inclusions",
            EndpointType::NodeVersion => "node-version",
            EndpointType::NodeNetwork => "node-network",
            EndpointType::PalletConsts => "consts",
            EndpointType::PalletStorage => "storage",
            EndpointType::PalletDispatchables => "dispatchables",
            EndpointType::PalletErrors => "errors",
            EndpointType::PalletEvents => "events",
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

    /// List all available endpoint types
    pub fn all() -> &'static [EndpointType] {
        &[
            EndpointType::AccountBalanceInfo,
            EndpointType::Block,
            EndpointType::BlockHeader,
            EndpointType::BlockExtrinsics,
            EndpointType::BlockParaInclusions,
            EndpointType::NodeVersion,
            EndpointType::NodeNetwork,
            EndpointType::PalletConsts,
            EndpointType::PalletStorage,
            EndpointType::PalletDispatchables,
            EndpointType::PalletErrors,
            EndpointType::PalletEvents,
            EndpointType::RuntimeSpec,
            EndpointType::RuntimeMetadata,
            EndpointType::TransactionMaterial,
        ]
    }

    /// List pallet endpoint types only
    pub fn pallet_endpoints() -> &'static [EndpointType] {
        &[
            EndpointType::PalletConsts,
            EndpointType::PalletStorage,
            EndpointType::PalletDispatchables,
            EndpointType::PalletErrors,
            EndpointType::PalletEvents,
        ]
    }

    /// List block endpoint types only
    pub fn block_endpoints() -> &'static [EndpointType] {
        &[
            EndpointType::Block,
            EndpointType::BlockHeader,
            EndpointType::BlockExtrinsics,
            EndpointType::BlockParaInclusions,
        ]
    }

    /// List runtime endpoint types only
    pub fn runtime_endpoints() -> &'static [EndpointType] {
        &[
            EndpointType::RuntimeSpec,
            EndpointType::RuntimeMetadata,
            EndpointType::TransactionMaterial,
            EndpointType::NodeVersion,
            EndpointType::NodeNetwork,
        ]
    }
}

impl fmt::Display for EndpointType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EndpointType::AccountBalanceInfo => write!(f, "account-balance-info"),
            EndpointType::Block => write!(f, "block"),
            EndpointType::BlockHeader => write!(f, "block-header"),
            EndpointType::BlockExtrinsics => write!(f, "block-extrinsics"),
            EndpointType::BlockParaInclusions => write!(f, "block-para-inclusions"),
            EndpointType::NodeVersion => write!(f, "node-version"),
            EndpointType::NodeNetwork => write!(f, "node-network"),
            EndpointType::PalletConsts => write!(f, "pallet-consts"),
            EndpointType::PalletStorage => write!(f, "pallet-storage"),
            EndpointType::PalletDispatchables => write!(f, "pallet-dispatchables"),
            EndpointType::PalletErrors => write!(f, "pallet-errors"),
            EndpointType::PalletEvents => write!(f, "pallet-events"),
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
            "block-para-inclusions" | "para-inclusions" => Ok(EndpointType::BlockParaInclusions),

            // Node endpoints
            "node-version" | "version" => Ok(EndpointType::NodeVersion),
            "node-network" | "network" => Ok(EndpointType::NodeNetwork),

            // Pallet endpoints
            "consts" | "pallet-consts" => Ok(EndpointType::PalletConsts),
            "storage" | "pallet-storage" => Ok(EndpointType::PalletStorage),
            "dispatchables" | "pallet-dispatchables" => Ok(EndpointType::PalletDispatchables),
            "errors" | "pallet-errors" => Ok(EndpointType::PalletErrors),
            "events" | "pallet-events" => Ok(EndpointType::PalletEvents),

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
