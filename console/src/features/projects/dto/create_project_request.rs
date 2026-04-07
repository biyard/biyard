use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub brand_logo_url: Option<String>,
    pub monthly_token_supply: i64,
    #[serde(default = "default_treasury_reserve_rate")]
    pub treasury_reserve_rate: f64,
}

fn default_treasury_reserve_rate() -> f64 {
    0.1
}
