use by_axum::axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    features::{accounts::Account, credentials::*},
    *,
};

pub async fn revoke_credential_handler(
    State(AppState { cli, .. }): State<AppState>,
    NoApi(account): NoApi<Account>,
    Path(CredentialPathParam { credential_id }): CredentialPath,
) -> Result<Json<CredentialResponse>> {
    tracing::debug!(
        "Revoking credential {} for account: {:?}",
        credential_id,
        account.pk
    );

    let credential_pk = Partition::Credential(credential_id);

    // Get the credential
    let credential = Credential::get(&cli, credential_pk.clone(), Some(EntityType::Credential))
        .await?
        .ok_or(Error::CredentialNotFound)?;

    // Verify ownership
    if credential.account_id != account.pk {
        return Err(Error::CredentialNotFound);
    }

    let credential = Credential::updater(credential.pk, credential.sk)
        .with_status(CredentialStatus::Revoked)
        .with_updated_at(time_utils::get_now())
        .execute(&cli)
        .await?;

    tracing::debug!("Revoked credential: {:?}", credential.pk);

    Ok(Json(credential.into()))
}
