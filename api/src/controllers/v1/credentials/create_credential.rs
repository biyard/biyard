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

    // Manually construct response to include the full API key (only returned on creation)
    let response = CredentialResponse {
        id: match &credential.pk {
            Partition::Credential(id) => id.clone(),
            _ => panic!("Invalid partition key for Credential"),
        },
        name: credential.name,
        api_key_prefix: credential.api_key_prefix,
        status: credential.status,
        created_at: credential.created_at,
        last_used_at: credential.last_used_at,
        api_key, // Include the full API key on creation
    };

    Ok(Json(response))
}
