use crate::common::Result;
use crate::common::OrganizationRole;
use crate::features::enterprises::InvitationResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EnterpriseContextAuth, Partition};
#[cfg(feature = "server")]
use crate::features::enterprises::Invitation;

/// Default invitation lifetime: 7 days. Owners/Admins can revoke earlier
/// via DELETE if needed.
#[cfg(feature = "server")]
const INVITATION_TTL_SECONDS: i64 = 60 * 60 * 24 * 7;

#[post("/v1/enterprises/invitations", auth: EnterpriseContextAuth)]
pub async fn create_invitation_handler(
    invited_email: String,
    role: OrganizationRole,
) -> Result<InvitationResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    // Only Admin or higher can issue invitations.
    if !auth.role.allows(OrganizationRole::Admin) {
        return Err(crate::common::Error::Forbidden);
    }

    // Owner role cannot be granted via invitation \u2014 owners are created
    // implicitly by signup. Demoting to viewer/admin only.
    let role = match role {
        OrganizationRole::Owner => OrganizationRole::Admin,
        other => other,
    };

    let token = generate_invitation_token();
    let invitation = Invitation::new(
        auth.enterprise.pk.clone(),
        token,
        invited_email,
        role,
        auth.account.pk.clone(),
        INVITATION_TTL_SECONDS,
    );

    invitation.create(cli).await?;

    Ok(invitation_to_response(invitation))
}

#[cfg(feature = "server")]
fn generate_invitation_token() -> String {
    use base64::Engine;
    use rand::RngExt;
    // 24 bytes = 192 bits of entropy. Plenty for an invite link.
    let mut bytes = [0u8; 24];
    for byte in &mut bytes {
        *byte = rand::rng().random::<u8>();
    }
    // URL-safe base64 without padding (a-zA-Z0-9_-).
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

#[cfg(feature = "server")]
fn invitation_to_response(invitation: Invitation) -> InvitationResponse {
    let enterprise_id = match &invitation.pk {
        Partition::Enterprise(id) => id.clone(),
        _ => String::new(),
    };
    InvitationResponse {
        token: invitation.token,
        enterprise_id,
        invited_email: invitation.invited_email,
        role: invitation.role,
        status: invitation.status,
        expires_at: invitation.expires_at,
        created_at: invitation.created_at,
    }
}
