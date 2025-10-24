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
    Path(credential_id): Path<String>,
) -> Result<Json<CredentialResponse>> {
    tracing::info!(
        "Revoking credential {} for account: {:?}",
        credential_id,
        account.pk
    );

    let credential_pk = Partition::Credential(credential_id);

    // Get the credential
    let mut credential = Credential::get(&cli, credential_pk.clone(), Some(EntityType::Credential))
        .await?
        .ok_or(Error::CredentialNotFound)?;

    // Verify ownership
    if credential.account_id != account.pk {
        return Err(Error::CredentialNotFound);
    }

    // Revoke it
    credential.revoke();

    // Save updated credential (recreate with new status)
    credential.create(&cli).await?;

    tracing::info!("Revoked credential: {:?}", credential.pk);

    Ok(Json(CredentialResponse {
        pk: credential.pk,
        name: credential.name,
        api_key_prefix: credential.api_key_prefix,
        status: credential.status,
        created_at: credential.created_at,
        last_used_at: credential.last_used_at,
        api_key: None,
    }))
}
