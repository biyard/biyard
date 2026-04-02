use crate::common::types::Partition;
use crate::features::projects::ProjectStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ProjectResponse {
    pub id: String,
    pub account_id: Partition,
    pub name: String,
    pub description: Option<String>,
    pub brand_logo_url: Option<String>,
    pub monthly_token_supply: i64,
    pub treasury_reserve_rate: f64,
    pub simulated_sales_total: i64,
    pub treasury_balance: i64,
    pub status: ProjectStatus,
    pub created_at: i64,
    pub updated_at: i64,
}
