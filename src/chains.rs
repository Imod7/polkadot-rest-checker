//! Chain definitions and pallet configurations for Polkadot ecosystem chains.

use std::fmt;

/// Supported chains for testing
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chain {
    Polkadot,
    Kusama,
    AssetHubPolkadot,
    AssetHubKusama,
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Chain::Polkadot => write!(f, "polkadot"),
            Chain::Kusama => write!(f, "kusama"),
            Chain::AssetHubPolkadot => write!(f, "asset-hub-polkadot"),
            Chain::AssetHubKusama => write!(f, "asset-hub-kusama"),
        }
    }
}

impl std::str::FromStr for Chain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "polkadot" | "dot" => Ok(Chain::Polkadot),
            "kusama" | "ksm" => Ok(Chain::Kusama),
            "asset-hub-polkadot" | "ahp" | "statemint" => Ok(Chain::AssetHubPolkadot),
            "asset-hub-kusama" | "ahk" | "statemine" => Ok(Chain::AssetHubKusama),
            _ => Err(format!(
                "Unknown chain '{}'. Valid options: polkadot, kusama, asset-hub-polkadot, asset-hub-kusama",
                s
            )),
        }
    }
}

/// Pallet definition with name and index
#[derive(Clone, Debug)]
pub struct Pallet {
    pub name: &'static str,
    pub index: u8,
}

impl Chain {
    /// Get all pallets for this chain
    pub fn pallets(&self) -> &'static [Pallet] {
        match self {
            Chain::Polkadot => POLKADOT_PALLETS,
            Chain::Kusama => KUSAMA_PALLETS,
            Chain::AssetHubPolkadot => ASSET_HUB_POLKADOT_PALLETS,
            Chain::AssetHubKusama => ASSET_HUB_KUSAMA_PALLETS,
        }
    }

    /// List all available chains
    pub fn all() -> &'static [Chain] {
        &[
            Chain::Polkadot,
            Chain::Kusama,
            Chain::AssetHubPolkadot,
            Chain::AssetHubKusama,
        ]
    }
}

// =============================================================================
// Polkadot Relay Chain Pallets
// From: runtimes/relay/polkadot/src/lib.rs
// =============================================================================

pub const POLKADOT_PALLETS: &[Pallet] = &[
    // Basic stuff
    Pallet { name: "System", index: 0 },
    Pallet { name: "Scheduler", index: 1 },
    Pallet { name: "Babe", index: 2 },
    Pallet { name: "Timestamp", index: 3 },
    Pallet { name: "Indices", index: 4 },
    Pallet { name: "Balances", index: 5 },
    Pallet { name: "Authorship", index: 6 },
    Pallet { name: "Staking", index: 7 },
    Pallet { name: "Offences", index: 8 },
    Pallet { name: "Session", index: 9 },
    Pallet { name: "Preimage", index: 10 },
    Pallet { name: "Grandpa", index: 11 },
    Pallet { name: "AuthorityDiscovery", index: 13 },
    // Governance
    Pallet { name: "Treasury", index: 19 },
    Pallet { name: "ConvictionVoting", index: 20 },
    Pallet { name: "Referenda", index: 21 },
    Pallet { name: "Origins", index: 22 },
    Pallet { name: "Whitelist", index: 23 },
    Pallet { name: "Claims", index: 24 },
    Pallet { name: "Vesting", index: 25 },
    Pallet { name: "Utility", index: 26 },
    Pallet { name: "Proxy", index: 29 },
    Pallet { name: "Multisig", index: 30 },
    Pallet { name: "TransactionPayment", index: 32 },
    Pallet { name: "Historical", index: 33 },
    Pallet { name: "Bounties", index: 34 },
    // Staking related
    Pallet { name: "ElectionProviderMultiPhase", index: 36 },
    Pallet { name: "VoterList", index: 37 },
    Pallet { name: "ChildBounties", index: 38 },
    Pallet { name: "NominationPools", index: 39 },
    Pallet { name: "FastUnstake", index: 40 },
    Pallet { name: "DelegatedStaking", index: 41 },
    Pallet { name: "StakingAhClient", index: 42 },
    // Parachains pallets
    Pallet { name: "ParachainsOrigin", index: 50 },
    Pallet { name: "Configuration", index: 51 },
    Pallet { name: "ParasShared", index: 52 },
    Pallet { name: "ParaInclusion", index: 53 },
    Pallet { name: "ParaInherent", index: 54 },
    Pallet { name: "ParaScheduler", index: 55 },
    Pallet { name: "Paras", index: 56 },
    Pallet { name: "Initializer", index: 57 },
    Pallet { name: "Dmp", index: 58 },
    Pallet { name: "Hrmp", index: 60 },
    Pallet { name: "ParaSessionInfo", index: 61 },
    Pallet { name: "ParasDisputes", index: 62 },
    Pallet { name: "ParasSlashing", index: 63 },
    Pallet { name: "OnDemand", index: 64 },
    Pallet { name: "CoretimeAssignmentProvider", index: 65 },
    // Parachain Onboarding
    Pallet { name: "Registrar", index: 70 },
    Pallet { name: "Slots", index: 71 },
    Pallet { name: "Auctions", index: 72 },
    Pallet { name: "Crowdloan", index: 73 },
    Pallet { name: "Coretime", index: 74 },
    // Infrastructure
    Pallet { name: "StateTrieMigration", index: 98 },
    Pallet { name: "XcmPallet", index: 99 },
    Pallet { name: "MessageQueue", index: 100 },
    Pallet { name: "AssetRate", index: 101 },
    // BEEFY & MMR
    Pallet { name: "Beefy", index: 200 },
    Pallet { name: "Mmr", index: 201 },
    Pallet { name: "BeefyMmrLeaf", index: 202 },
    // Migrator
    Pallet { name: "RcMigrator", index: 255 },
];

// =============================================================================
// Kusama Relay Chain Pallets
// From: runtimes/relay/kusama/src/lib.rs
// =============================================================================

pub const KUSAMA_PALLETS: &[Pallet] = &[
    // Basic stuff
    Pallet { name: "System", index: 0 },
    Pallet { name: "Babe", index: 1 },
    Pallet { name: "Timestamp", index: 2 },
    Pallet { name: "Indices", index: 3 },
    Pallet { name: "Balances", index: 4 },
    Pallet { name: "Authorship", index: 5 },
    Pallet { name: "Staking", index: 6 },
    Pallet { name: "Offences", index: 7 },
    Pallet { name: "Session", index: 8 },
    Pallet { name: "Grandpa", index: 10 },
    Pallet { name: "AuthorityDiscovery", index: 12 },
    // Governance
    Pallet { name: "Treasury", index: 18 },
    Pallet { name: "Claims", index: 19 },
    Pallet { name: "ConvictionVoting", index: 20 },
    Pallet { name: "Referenda", index: 21 },
    Pallet { name: "FellowshipCollective", index: 22 },
    Pallet { name: "FellowshipReferenda", index: 23 },
    Pallet { name: "Utility", index: 24 },
    Pallet { name: "Society", index: 26 },
    Pallet { name: "Recovery", index: 27 },
    Pallet { name: "Vesting", index: 28 },
    Pallet { name: "Scheduler", index: 29 },
    Pallet { name: "Proxy", index: 30 },
    Pallet { name: "Multisig", index: 31 },
    Pallet { name: "Preimage", index: 32 },
    Pallet { name: "TransactionPayment", index: 33 },
    Pallet { name: "Historical", index: 34 },
    Pallet { name: "Bounties", index: 35 },
    Pallet { name: "ElectionProviderMultiPhase", index: 37 },
    Pallet { name: "VoterList", index: 39 },
    Pallet { name: "ChildBounties", index: 40 },
    Pallet { name: "NominationPools", index: 41 },
    Pallet { name: "FastUnstake", index: 42 },
    Pallet { name: "Origins", index: 43 },
    Pallet { name: "Whitelist", index: 44 },
    Pallet { name: "Parameters", index: 46 },
    Pallet { name: "DelegatedStaking", index: 47 },
    Pallet { name: "StakingAhClient", index: 48 },
    // Parachains pallets
    Pallet { name: "ParachainsOrigin", index: 50 },
    Pallet { name: "Configuration", index: 51 },
    Pallet { name: "ParasShared", index: 52 },
    Pallet { name: "ParaInclusion", index: 53 },
    Pallet { name: "ParaInherent", index: 54 },
    Pallet { name: "ParaScheduler", index: 55 },
    Pallet { name: "Paras", index: 56 },
    Pallet { name: "Initializer", index: 57 },
    Pallet { name: "Dmp", index: 58 },
    Pallet { name: "Hrmp", index: 60 },
    Pallet { name: "ParaSessionInfo", index: 61 },
    Pallet { name: "ParasDisputes", index: 62 },
    Pallet { name: "ParasSlashing", index: 63 },
    Pallet { name: "OnDemandAssignmentProvider", index: 64 },
    Pallet { name: "CoretimeAssignmentProvider", index: 65 },
    // Parachain Onboarding
    Pallet { name: "Registrar", index: 70 },
    Pallet { name: "Slots", index: 71 },
    Pallet { name: "Auctions", index: 72 },
    Pallet { name: "Crowdloan", index: 73 },
    Pallet { name: "Coretime", index: 74 },
    // Infrastructure
    Pallet { name: "XcmPallet", index: 99 },
    Pallet { name: "MessageQueue", index: 100 },
    Pallet { name: "AssetRate", index: 101 },
    // BEEFY & MMR
    Pallet { name: "Beefy", index: 200 },
    Pallet { name: "Mmr", index: 201 },
    Pallet { name: "BeefyMmrLeaf", index: 202 },
    // Migrator
    Pallet { name: "RcMigrator", index: 255 },
];

// =============================================================================
// Asset Hub Polkadot Pallets
// From: runtimes/system-parachains/asset-hubs/asset-hub-polkadot/src/lib.rs
// =============================================================================

pub const ASSET_HUB_POLKADOT_PALLETS: &[Pallet] = &[
    // System support
    Pallet { name: "System", index: 0 },
    Pallet { name: "ParachainSystem", index: 1 },
    Pallet { name: "Timestamp", index: 3 },
    Pallet { name: "ParachainInfo", index: 4 },
    Pallet { name: "Preimage", index: 5 },
    Pallet { name: "Scheduler", index: 6 },
    Pallet { name: "Parameters", index: 7 },
    Pallet { name: "WeightReclaim", index: 8 },
    // Monetary
    Pallet { name: "Balances", index: 10 },
    Pallet { name: "TransactionPayment", index: 11 },
    Pallet { name: "AssetTxPayment", index: 13 },
    Pallet { name: "Vesting", index: 14 },
    Pallet { name: "Claims", index: 15 },
    // Collator support
    Pallet { name: "Authorship", index: 20 },
    Pallet { name: "CollatorSelection", index: 21 },
    Pallet { name: "Session", index: 22 },
    Pallet { name: "Aura", index: 23 },
    Pallet { name: "AuraExt", index: 24 },
    // XCM helpers
    Pallet { name: "XcmpQueue", index: 30 },
    Pallet { name: "PolkadotXcm", index: 31 },
    Pallet { name: "CumulusXcm", index: 32 },
    Pallet { name: "ToKusamaXcmRouter", index: 34 },
    Pallet { name: "MessageQueue", index: 35 },
    Pallet { name: "SnowbridgeSystemFrontend", index: 36 },
    // Utilities
    Pallet { name: "Utility", index: 40 },
    Pallet { name: "Multisig", index: 41 },
    Pallet { name: "Proxy", index: 42 },
    Pallet { name: "Indices", index: 43 },
    // Assets
    Pallet { name: "Assets", index: 50 },
    Pallet { name: "Uniques", index: 51 },
    Pallet { name: "Nfts", index: 52 },
    Pallet { name: "ForeignAssets", index: 53 },
    Pallet { name: "PoolAssets", index: 54 },
    Pallet { name: "AssetConversion", index: 55 },
    // OpenGov
    Pallet { name: "Treasury", index: 60 },
    Pallet { name: "ConvictionVoting", index: 61 },
    Pallet { name: "Referenda", index: 62 },
    Pallet { name: "Origins", index: 63 },
    Pallet { name: "Whitelist", index: 64 },
    Pallet { name: "Bounties", index: 65 },
    Pallet { name: "ChildBounties", index: 66 },
    Pallet { name: "AssetRate", index: 67 },
    // State trie migration
    Pallet { name: "StateTrieMigration", index: 70 },
    // Staking
    Pallet { name: "NominationPools", index: 80 },
    Pallet { name: "VoterList", index: 82 },
    Pallet { name: "DelegatedStaking", index: 83 },
    Pallet { name: "StakingRcClient", index: 84 },
    Pallet { name: "MultiBlockElection", index: 85 },
    Pallet { name: "MultiBlockElectionVerifier", index: 86 },
    Pallet { name: "MultiBlockElectionUnsigned", index: 87 },
    Pallet { name: "MultiBlockElectionSigned", index: 88 },
    Pallet { name: "Staking", index: 89 },
    // Contracts
    Pallet { name: "Revive", index: 90 },
    // Migration
    Pallet { name: "AhOps", index: 254 },
    Pallet { name: "AhMigrator", index: 255 },
];

// =============================================================================
// Asset Hub Kusama Pallets
// From: runtimes/system-parachains/asset-hubs/asset-hub-kusama/src/lib.rs
// =============================================================================

pub const ASSET_HUB_KUSAMA_PALLETS: &[Pallet] = &[
    // System support
    Pallet { name: "System", index: 0 },
    Pallet { name: "ParachainSystem", index: 1 },
    Pallet { name: "Timestamp", index: 3 },
    Pallet { name: "ParachainInfo", index: 4 },
    Pallet { name: "MultiBlockMigrations", index: 5 },
    Pallet { name: "Preimage", index: 6 },
    Pallet { name: "Scheduler", index: 7 },
    Pallet { name: "Parameters", index: 8 },
    Pallet { name: "WeightReclaim", index: 9 },
    // Monetary
    Pallet { name: "Balances", index: 10 },
    Pallet { name: "TransactionPayment", index: 11 },
    Pallet { name: "AssetTxPayment", index: 13 },
    Pallet { name: "Vesting", index: 14 },
    Pallet { name: "Claims", index: 15 },
    // Collator support
    Pallet { name: "Authorship", index: 20 },
    Pallet { name: "CollatorSelection", index: 21 },
    Pallet { name: "Session", index: 22 },
    Pallet { name: "Aura", index: 23 },
    Pallet { name: "AuraExt", index: 24 },
    // XCM helpers
    Pallet { name: "XcmpQueue", index: 30 },
    Pallet { name: "PolkadotXcm", index: 31 },
    Pallet { name: "CumulusXcm", index: 32 },
    Pallet { name: "ToPolkadotXcmRouter", index: 34 },
    Pallet { name: "MessageQueue", index: 35 },
    // Utilities
    Pallet { name: "Utility", index: 40 },
    Pallet { name: "Multisig", index: 41 },
    Pallet { name: "Proxy", index: 42 },
    Pallet { name: "RemoteProxyRelayChain", index: 43 },
    Pallet { name: "Indices", index: 44 },
    // Assets
    Pallet { name: "Assets", index: 50 },
    Pallet { name: "Uniques", index: 51 },
    Pallet { name: "Nfts", index: 52 },
    Pallet { name: "ForeignAssets", index: 53 },
    Pallet { name: "NftFractionalization", index: 54 },
    Pallet { name: "PoolAssets", index: 55 },
    Pallet { name: "AssetConversion", index: 56 },
    Pallet { name: "Recovery", index: 57 },
    Pallet { name: "Society", index: 58 },
    // Contracts
    Pallet { name: "Revive", index: 60 },
    // State trie migration
    Pallet { name: "StateTrieMigration", index: 70 },
    // Staking
    Pallet { name: "NominationPools", index: 80 },
    Pallet { name: "VoterList", index: 82 },
    Pallet { name: "DelegatedStaking", index: 83 },
    Pallet { name: "StakingRcClient", index: 84 },
    Pallet { name: "MultiBlockElection", index: 85 },
    Pallet { name: "MultiBlockElectionVerifier", index: 86 },
    Pallet { name: "MultiBlockElectionUnsigned", index: 87 },
    Pallet { name: "MultiBlockElectionSigned", index: 88 },
    Pallet { name: "Staking", index: 89 },
    // OpenGov
    Pallet { name: "Treasury", index: 90 },
    Pallet { name: "ConvictionVoting", index: 91 },
    Pallet { name: "Referenda", index: 92 },
    Pallet { name: "Origins", index: 93 },
    Pallet { name: "Whitelist", index: 94 },
    Pallet { name: "Bounties", index: 95 },
    Pallet { name: "ChildBounties", index: 96 },
    Pallet { name: "AssetRate", index: 97 },
    // Migration
    Pallet { name: "AhOps", index: 254 },
    Pallet { name: "AhMigrator", index: 255 },
];
