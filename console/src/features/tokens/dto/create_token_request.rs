use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTokenRequest {
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    #[serde(default)]
    pub monthly_emission: i64,
    #[serde(default = "default_decay_rate_bps")]
    pub decay_rate_bps: u16,
    #[serde(default)]
    pub distribution_slots: Vec<crate::features::tokens::DistributionSlotEntry>,
    pub stable_token_address: Option<String>,
}

fn default_decay_rate_bps() -> u16 {
    500
}
