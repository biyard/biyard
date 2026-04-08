use crate::common::Result;
use crate::features::enterprises::InvitationResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EnterpriseContextAuth, OrganizationRole, Partition};
#[cfg(feature = "server")]
use crate::features::enterprises::{Invitation, InvitationQueryOption, InvitationStatus};

#[get("/v1/enterprises/invitations", auth: EnterpriseContextAuth)]
pub async fn list_invitations_handler() -> Result<Vec<InvitationResponse>> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    if !auth.role.allows(OrganizationRole::Admin) {
        return Err(crate::common::Error::Forbidden);
    }

    // Single-table query: filter by sk prefix so we only fetch
    // Invitation rows. Without this, the query returns every row
    // under pk = ENT#<id> (including the Enterprise meta row), and
    // serde_dynamo fails to deserialize non-Invitation items into
    // `Invitation` (e.g. "missing field `token`").
    let (invitations, _) = Invitation::query(
        cli,
        &auth.enterprise.pk,
        InvitationQueryOption::builder()
            .sk("INVITATION#".to_string())
            .limit(200),
    )
    .await?;

    let now = crate::common::utils::time_utils::get_now();
    let responses: Vec<InvitationResponse> = invitations
        .into_iter()
        .filter(|inv| {
            // Surface only Pending invitations that have not yet expired.
            // Expired/revoked/accepted entries stay in the database for
            // audit but are not shown in the listing.
            matches!(inv.status, InvitationStatus::Pending) && inv.expires_at > now
        })
        .map(invitation_to_response)
        .collect();

    Ok(responses)
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
