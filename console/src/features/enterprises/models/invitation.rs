use crate::common::*;
use crate::features::enterprises::types::InvitationStatus;

/// Pending or historical invitation to join an Enterprise.
///
/// Storage layout (single-table):
///   pk = ENT#<enterprise_id>
///   sk = INVITE#<token>
///
/// `gsi1` allows looking up an invitation by token alone (used by the
/// public `/v1/invitations/:token` preview endpoint where the caller
/// does not yet know the target enterprise).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct Invitation {
    pub pk: Partition,
    pub sk: EntityType,

    /// URL-safe random token (also embedded in `sk` as `INVITE#<token>`).
    /// Indexed via gsi1 so the public preview endpoint can look up the
    /// invitation without knowing the enterprise.
    #[dynamo(index = "gsi1", pk, prefix = "INVITE", name = "find_by_token")]
    pub token: String,

    #[dynamo(index = "gsi1", sk, name = "find_by_token")]
    pub gsi1_sk: EntityType,

    /// Email the invite was created for. Stored for display only — the
    /// accept flow does not enforce that the signing-up user matches.
    pub invited_email: String,

    pub role: OrganizationRole,
    pub invited_by_account_id: Partition,
    pub expires_at: i64,
    pub status: InvitationStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Invitation {
    pub fn new(
        enterprise_id: Partition,
        token: String,
        invited_email: String,
        role: OrganizationRole,
        invited_by_account_id: Partition,
        ttl_seconds: i64,
    ) -> Self {
        let now = crate::common::utils::time_utils::get_now();
        let expires_at = now + ttl_seconds;

        Self {
            pk: enterprise_id,
            sk: EntityType::Invitation(token.clone()),
            token,
            gsi1_sk: EntityType::Invitation(String::new()),
            invited_email,
            role,
            invited_by_account_id,
            expires_at,
            status: InvitationStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }

    /// Build the (pk, sk) pair for direct `Invitation::get` lookups when
    /// both enterprise and token are known.
    pub fn keys(enterprise_id: Partition, token: &str) -> (Partition, EntityType) {
        (enterprise_id, EntityType::Invitation(token.to_string()))
    }

    pub fn is_active(&self, now: i64) -> bool {
        matches!(self.status, InvitationStatus::Pending) && self.expires_at > now
    }
}
