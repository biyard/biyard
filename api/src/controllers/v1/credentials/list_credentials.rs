use by_axum::axum::{extract::State, Json};

use crate::{features::{accounts::Account, credentials::*}, *};

pub async fn list_credentials_handler(
    State(AppState { cli, .. }): State<AppState>,
    account: Account,
) -> Result<Json<Vec<CredentialResponse>>> {
    tracing::info!("Listing credentials for account: {:?}", account.pk);

    // Find all credentials for this account using GSI1
    let (credentials, _bookmark) = Credential::find_by_account_id(
        &cli,
        account.pk,
        CredentialQueryOption::default()
    ).await?;

    // Convert to response DTOs (without api_key)
    let responses: Vec<CredentialResponse> = credentials
        .into_iter()
        .map(|c| CredentialResponse {
            pk: c.pk,
            name: c.name,
            api_key_prefix: c.api_key_prefix,
            status: c.status,
            created_at: c.created_at,
            last_used_at: c.last_used_at,
            api_key: None, // Never return full key on list
        })
        .collect();

    Ok(Json(responses))
}
