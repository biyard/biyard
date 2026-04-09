use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTokenRequest {
    pub name: String,
    pub symbol: String,
    #[serde(default)]
    pub decimals: u8,
    pub description: Option<String>,
}
