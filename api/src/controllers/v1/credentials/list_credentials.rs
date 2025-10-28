use by_axum::axum::{Json, extract::State};

use crate::{
    features::{accounts::Account, credentials::*},
    *,
};

pub async fn list_credentials_handler(
    State(AppState { cli, .. }): State<AppState>,
    NoApi(account): NoApi<Account>,
) -> Result<Json<Vec<CredentialSummaryResponse>>> {
    tracing::debug!("Listing credentials for account: {:?}", account.pk);

    // Find all credentials for this account using GSI1
    let (credentials, _bookmark) =
        Credential::find_by_account_id(&cli, account.pk, CredentialQueryOption::default()).await?;

    // Convert to response DTOs (without api_key)
    let responses: Vec<CredentialSummaryResponse> =
        credentials.into_iter().map(|c| c.into()).collect();

    Ok(Json(responses))
}
