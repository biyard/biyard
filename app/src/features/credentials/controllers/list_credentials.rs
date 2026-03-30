use crate::common::{CommonConfig, Result};
use crate::features::accounts::Account;
use crate::features::credentials::{Credential, CredentialQueryOption, CredentialSummaryResponse};
use dioxus::prelude::get;

#[get("/v1/credentials", account: Account)]
pub async fn list_credentials_handler() -> Result<Vec<CredentialSummaryResponse>> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let (credentials, _) =
        Credential::find_by_account_id(cli, account.pk, CredentialQueryOption::default()).await?;

    let responses: Vec<CredentialSummaryResponse> =
        credentials.into_iter().map(|c| c.into()).collect();

    Ok(responses)
}
