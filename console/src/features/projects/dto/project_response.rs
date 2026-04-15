use crate::common::types::Partition;
use crate::features::projects::ProjectStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct ProjectResponse {
    /// Unique project identifier.
    pub id: String,
    /// Owner account identifier.
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub account_id: Partition,
    /// Enterprise (organization) identifier.
    #[serde(default)]
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub organization_id: Partition,
    /// Project display name.
    pub name: String,
    /// Project description.
    pub description: Option<String>,
    /// Brand logo image URL.
    pub brand_logo_url: Option<String>,
    /// Monthly token supply for this project.
    pub monthly_token_supply: i64,
    /// Treasury reserve rate (e.g. 0.2 = 20%).
    pub treasury_reserve_rate: f64,
    /// Project status: Active or Inactive.
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub status: ProjectStatus,
    /// Creation timestamp (Unix epoch seconds).
    pub created_at: i64,
    /// Last update timestamp (Unix epoch seconds).
    pub updated_at: i64,
}
