use crate::common::Result;
use crate::features::accounts::AccountResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::SESSION_KEY_ACCOUNT_ID;
#[cfg(feature = "server")]
use crate::common::{CommonConfig, Extension};
#[cfg(feature = "server")]
use crate::features::accounts::{Account, AccountError, AccountQueryOption, PasswordScheme};
#[cfg(feature = "server")]
use crate::features::enterprises::{
    EnterpriseError, Invitation, InvitationQueryOption, InvitationStatus,
};

/// Sign up flow that consumes a pending invitation token. The invitation
/// determines which Enterprise the new Account joins and at what role.
/// On success the invitation is marked Accepted (one-shot) and a session
/// is created so the visitor lands authenticated.
#[post("/v1/accounts/signup-with-invite", session: Extension<tower_sessions::Session>)]
pub async fn signup_with_invite_handler(
    name: String,
    email: String,
    password: String,
    invitation_token: String,
) -> Result<AccountResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let Extension(session) = session;

    let (invitations, _) = Invitation::find_by_token(
        cli,
        &invitation_token,
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
        InvitationStatus::Accepted => return Err(EnterpriseError::InvitationAlreadyAccepted.into()),
        InvitationStatus::Pending => {}
    }
    if invitation.expires_at <= now {
        return Err(EnterpriseError::InvitationExpired.into());
    }

    // Reject if email already exists. The accept page UX should redirect
    // such users to the regular sign-in flow instead.
    let (existing, _) =
        Account::find_by_email(cli, &email, AccountQueryOption::builder().limit(1)).await?;
    if !existing.is_empty() {
        return Err(AccountError::EmailAlreadyExists.into());
    }

    crate::common::utils::user_password_utils::enforce_password_policy(
        &password,
        Some(&email),
        Some(&name),
    )?;

    let password_hash = crate::common::utils::user_password_utils::hash_password(&password)?;
    let mut account = Account::new(name, email, password_hash, PasswordScheme::BcryptV1);
    account.enterprise_id = invitation.pk.clone();
    account.organization_role = invitation.role;

    account.create(cli).await?;

    // Mark the invitation as accepted so it cannot be reused.
    Invitation::updater(invitation.pk.clone(), invitation.sk.clone())
        .with_status(InvitationStatus::Accepted)
        .with_updated_at(now)
        .execute(cli)
        .await?;

    session
        .insert(SESSION_KEY_ACCOUNT_ID, account.pk.to_string())
        .await?;

    Ok(account.into())
}
