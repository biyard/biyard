use serde::{Deserialize, Serialize};

/// Live on-chain snapshot of a brand project's treasury.
///
/// All values come from an RPC read against the treasury and brand
/// token contracts. When `deployed = false` every other field is
/// zero / empty and the frontend should treat the whole payload as
/// unavailable (the brand still needs to deploy its contracts).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TreasuryStatusResponse {
    /// Whether the treasury + brand token contracts have been deployed
    /// for this project.
    pub deployed: bool,

    /// Chain id the contracts live on (e.g. 1001 for Kaia Kairos).
    pub chain_id: Option<u64>,

    /// Treasury contract address (hex string, checksummed/lowercased
    /// from the provider).
    pub treasury_contract_address: Option<String>,
    /// Brand token contract address.
    pub brand_token_address: Option<String>,

    /// Raw treasury balance in stable token units (as string to avoid
    /// JSON 53-bit precision loss on large u128 values).
    pub treasury_balance_raw: String,
    /// Stable token decimals (e.g. 6 for USDT).
    pub stable_decimals: u8,
    /// Stable token symbol (e.g. "USDT").
    pub stable_symbol: String,
    /// Whether the stable token has a public faucet mint (e.g. BUSDT).
    /// When true, the console can mint + deposit for demo purposes.
    pub stable_mintable: bool,

    /// Brand token `totalSupply` (raw units).
    pub total_supply_raw: String,
    /// Brand token `circulatingSupply` from the treasury contract.
    pub circulating_supply_raw: String,
    /// Brand tokens held by the Treasury (bought back, out of circulation).
    #[serde(default)]
    pub treasury_held_tokens_raw: String,
    /// Brand token decimals.
    pub token_decimals: u8,
    /// Brand token symbol (e.g. "TKN").
    #[serde(default)]
    pub token_symbol: String,

    /// Raw floor price scaled by 1e18, matching `getFloorPrice()` on
    /// the Solidity contract. `0` when circulating supply is zero.
    pub floor_price_raw_1e18: String,

    /// On-chain `currentMonth()` index from the BrandToken contract.
    /// Includes `monthOffset` from `advanceMonth()` calls.
    #[serde(default)]
    pub current_month: u64,
}
