use crate::common::OrganizationRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberResponse {
    pub account_id: String,
    pub name: String,
    pub email: String,
    pub role: OrganizationRole,
    pub joined_at: i64,
}
