use crate::common::Result;
use crate::features::enterprises::InvitationPreviewResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, Partition};
#[cfg(feature = "server")]
use crate::features::enterprises::{
    Enterprise, EnterpriseError, Invitation, InvitationQueryOption, InvitationStatus,
};

/// Public, unauthenticated lookup used by the accept page so a visitor
/// can see what they're being invited to before signing up.
///
/// Intentionally returns minimal information \u2014 no inviter identity,
/// no member roster.
#[get("/v1/invitations/:token")]
pub async fn get_invitation_preview_handler(
    token: String,
) -> Result<InvitationPreviewResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let (invitations, _) = Invitation::find_by_token(
        cli,
        &token,
        InvitationQueryOption::builder().limit(1),
    )
    .await?;

    let invitation = invitations
        .into_iter()
        .next()
        .ok_or(EnterpriseError::InvitationNotFound)?;

    let now = crate::common::utils::time_utils::get_now();
    match invitation.status {
        InvitationStatus::Revoked => return Err(EnterpriseError::InvitationRevoked.into()),
        InvitationStatus::Accepted => {
            return Err(EnterpriseError::InvitationAlreadyAccepted.into())
        }
        InvitationStatus::Pending => {}
    }

    if invitation.expires_at <= now {
        return Err(EnterpriseError::InvitationExpired.into());
    }

    let enterprise = Enterprise::get(cli, &invitation.pk, Some(EntityType::Enterprise))
        .await?
        .ok_or(EnterpriseError::EnterpriseNotFound)?;

    let enterprise_id = match &invitation.pk {
        Partition::Enterprise(id) => id.clone(),
        _ => String::new(),
    };

    Ok(InvitationPreviewResponse {
        token: invitation.token,
        enterprise_id,
        enterprise_name: enterprise.name,
        invited_email: invitation.invited_email,
        role: invitation.role,
        expires_at: invitation.expires_at,
    })
}
