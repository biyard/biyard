use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum EnterpriseError {
    #[error("Enterprise not found")]
    EnterpriseNotFound,
    #[error("Enterprise access denied")]
    EnterpriseAccessDenied,
    #[error("Invitation not found")]
    InvitationNotFound,
    #[error("Invitation expired")]
    InvitationExpired,
    #[error("Invitation already accepted")]
    InvitationAlreadyAccepted,
    #[error("Invitation revoked")]
    InvitationRevoked,
    #[error("Cannot remove the last owner of this enterprise")]
    LastOwnerCannotLeave,
    #[error("Cannot demote the last owner of this enterprise")]
    LastOwnerCannotDemote,
    #[error("Member not found")]
    MemberNotFound,
    #[error("Account already belongs to an enterprise")]
    AccountAlreadyInEnterprise,
    #[error("Enterprise name cannot be empty")]
    InvalidEnterpriseName,
}
