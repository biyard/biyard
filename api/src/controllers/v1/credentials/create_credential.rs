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
    tracing::info!("Creating credential for account: {:?}", account.pk);

    // Generate a random API key
    let api_key = format!(
        "biyard_{}",
        uuid::Uuid::new_v4().to_string().replace("-", "")
    );

    // Create credential with hashed key
    let credential = Credential::new(account.pk.clone(), req.name.clone(), &api_key);

    // Save to DynamoDB
    credential.create(&cli).await?;

    tracing::info!("Created credential: {:?}", credential.pk);

    // Return response with full API key (only time it's shown)
    Ok(Json(CredentialResponse {
        pk: credential.pk,
        name: credential.name,
        api_key_prefix: credential.api_key_prefix,
        status: credential.status,
        created_at: credential.created_at,
        last_used_at: credential.last_used_at,
        api_key: Some(api_key), // Full key only on creation
    }))
}
