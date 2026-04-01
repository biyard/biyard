use crate::common::Result;
use crate::features::credentials::CredentialSummaryResponse;
use dioxus::prelude::get;

#[cfg(feature = "server")]
use crate::common::CommonConfig;
#[cfg(feature = "server")]
use crate::features::accounts::Account;
#[cfg(feature = "server")]
use crate::features::credentials::{Credential, CredentialQueryOption};

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
