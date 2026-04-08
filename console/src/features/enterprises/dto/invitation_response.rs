use crate::common::OrganizationRole;
use crate::features::enterprises::InvitationStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvitationResponse {
    pub token: String,
    pub enterprise_id: String,
    pub invited_email: String,
    pub role: OrganizationRole,
    pub status: InvitationStatus,
    pub expires_at: i64,
    pub created_at: i64,
}

/// Public preview shown on the accept page before the visitor signs up.
/// Intentionally does not leak the inviter's account id.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvitationPreviewResponse {
    pub token: String,
    pub enterprise_id: String,
    pub enterprise_name: String,
    pub invited_email: String,
    pub role: OrganizationRole,
    pub expires_at: i64,
}
