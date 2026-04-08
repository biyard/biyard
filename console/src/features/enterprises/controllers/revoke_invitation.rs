use crate::common::Result;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EnterpriseContextAuth, OrganizationRole};
#[cfg(feature = "server")]
use crate::features::enterprises::{EnterpriseError, Invitation, InvitationStatus};

#[delete("/v1/enterprises/invitations/:token", auth: EnterpriseContextAuth)]
pub async fn revoke_invitation_handler(token: String) -> Result<()> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    if !auth.role.allows(OrganizationRole::Admin) {
        return Err(crate::common::Error::Forbidden);
    }

    let (pk, sk) = Invitation::keys(auth.enterprise.pk.clone(), &token);
    let invitation = Invitation::get(cli, &pk, Some(sk))
        .await?
        .ok_or(EnterpriseError::InvitationNotFound)?;

    if !matches!(invitation.status, InvitationStatus::Pending) {
        // Already accepted/revoked is treated as a no-op success so the
        // UI can refresh without surfacing a confusing error.
        return Ok(());
    }

    let now = crate::common::utils::time_utils::get_now();
    Invitation::updater(invitation.pk.clone(), invitation.sk.clone())
        .with_status(InvitationStatus::Revoked)
        .with_updated_at(now)
        .execute(cli)
        .await?;

    Ok(())
}
