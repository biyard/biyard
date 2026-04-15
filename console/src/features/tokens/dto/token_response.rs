use crate::common::types::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct TokenResponse {
    /// Token identifier.
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub pk: Partition,
    /// Token name (e.g. "My Brand Token").
    pub name: String,
    /// Token symbol (e.g. "MBT"). Immutable after deployment.
    pub symbol: String,
    /// Current circulating supply in smallest token units.
    pub circulating_supply: i64,
    /// Token description.
    pub description: Option<String>,
    /// Deployed ERC-20 contract address.
    pub contract_address: Option<String>,
    /// Treasury contract address.
    pub treasury_contract_address: Option<String>,
    /// Multisig wallet address.
    pub multisig_address: Option<String>,
    /// Stable token address used for treasury (e.g. USDT).
    pub stable_token_address: Option<String>,
    /// Blockchain chain ID (e.g. 1001 for Kaia Kairos).
    pub chain_id: Option<u64>,
    /// Transaction hash of the token deployment.
    pub deployment_tx_hash: Option<String>,
    /// Transaction hash of the treasury deployment.
    pub treasury_deployment_tx_hash: Option<String>,
    /// Transaction hash of the multisig deployment.
    pub multisig_deployment_tx_hash: Option<String>,
    /// Treasury reserve rate in basis points (e.g. 2000 = 20%).
    pub treasury_reserve_bps: u64,
    /// Monthly token emission amount.
    pub monthly_emission: i64,
    /// Monthly emission decay rate in basis points (e.g. 500 = 5%).
    pub decay_rate_bps: u16,
    /// Token distribution slots with wallet addresses and basis point shares.
    pub distribution_slots: Vec<crate::features::tokens::DistributionSlotEntry>,
    /// Last month tokens were minted (YYYY-MM).
    #[serde(default)]
    pub last_minted_month: Option<String>,
    /// Whether a deployment is currently in progress.
    #[serde(default)]
    pub deploying: bool,
    /// First month of token emission (YYYY-MM).
    #[serde(default)]
    pub start_month: Option<String>,
    /// Creation timestamp (Unix epoch seconds).
    pub created_at: i64,
    /// Last update timestamp (Unix epoch seconds).
    pub updated_at: i64,
}
