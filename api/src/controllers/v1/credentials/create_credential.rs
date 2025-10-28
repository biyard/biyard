use by_axum::axum::{Json, extract::State};

use crate::{
    features::{accounts::Account, credentials::*},
    *,
};

pub async fn create_credential_handler(
    State(AppState { cli, .. }): State<AppState>,
    NoApi(account): NoApi<Account>,
    Json(req): Json<CreateCredentialRequest>,
) -> Result<Json<CredentialResponse>> {
    tracing::debug!("Creating credential for account: {:?}", account.pk);

    // Generate a random API key
    let api_key = format!(
        "biyard_{}",
        uuid::Uuid::new_v4().to_string().replace("-", "")
    );

    // Create credential with hashed key
    let credential = Credential::new(account.pk.clone(), req.name.clone(), &api_key);

    // Save to DynamoDB
    credential.create(&cli).await?;

    tracing::debug!("Created credential: {:?}", credential.pk);

    Ok(Json(credential.into()))
}
