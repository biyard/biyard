use crate::features::projects::ProjectStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub monthly_token_supply: Option<i64>,
    pub status: Option<ProjectStatus>,
}
