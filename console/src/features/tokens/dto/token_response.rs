use crate::common::types::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenResponse {
    pub pk: Partition,
    pub name: String,
    pub symbol: String,
    pub circulating_supply: i64,
    pub description: Option<String>,
    pub contract_address: Option<String>,
    pub treasury_contract_address: Option<String>,
    pub multisig_address: Option<String>,
    pub stable_token_address: Option<String>,
    pub chain_id: Option<u64>,
    pub deployment_tx_hash: Option<String>,
    pub treasury_deployment_tx_hash: Option<String>,
    pub multisig_deployment_tx_hash: Option<String>,
    pub treasury_reserve_bps: u64,
    pub monthly_emission: i64,
    pub decay_rate_bps: u16,
    pub distribution_slots: Vec<crate::features::tokens::DistributionSlotEntry>,
    #[serde(default)]
    pub last_minted_month: Option<String>,
    #[serde(default)]
    pub deploying: bool,
    #[serde(default)]
    pub start_month: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
