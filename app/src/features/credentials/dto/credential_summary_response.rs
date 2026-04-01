use crate::features::credentials::CredentialStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CredentialSummaryResponse {
    pub id: String,
    pub name: String,
    pub api_key_prefix: String,
    pub status: CredentialStatus,
    pub created_at: i64,
    pub last_used_at: Option<i64>,
}
