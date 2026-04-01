use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub monthly_token_supply: i64,
    pub symbol: String,
    #[serde(default = "default_decimals")]
    pub decimals: u8,
}

fn default_decimals() -> u8 {
    0
}
