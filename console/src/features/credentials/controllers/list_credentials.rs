use crate::common::Result;
use crate::features::credentials::CredentialSummaryResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::CommonConfig;
#[cfg(feature = "server")]
use crate::common::EnterpriseContextAuth;
#[cfg(feature = "server")]
use crate::features::credentials::{Credential, CredentialQueryOption};

#[get("/v1/credentials", auth: EnterpriseContextAuth)]
pub async fn list_credentials_handler() -> Result<Vec<CredentialSummaryResponse>> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    // Enterprise context is guaranteed by EnterpriseContextAuth, which lazily
    // backfills legacy account-scoped resources on first request.
    let (credentials, _) = Credential::find_by_organization_id(
        cli,
        auth.enterprise.pk.clone(),
        CredentialQueryOption::default(),
    )
    .await?;

    let responses: Vec<CredentialSummaryResponse> =
        credentials.into_iter().map(|c| c.into()).collect();

    Ok(responses)
}
