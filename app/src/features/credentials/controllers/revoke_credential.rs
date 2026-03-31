use crate::common::Result;
use crate::features::credentials::{CredentialError, CredentialResponse, CredentialStatus};
use dioxus::prelude::delete;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType};
#[cfg(feature = "server")]
use crate::features::accounts::Account;
#[cfg(feature = "server")]
use crate::features::credentials::Credential;

#[delete("/v1/credentials/:credential_id", account: Account)]
pub async fn revoke_credential_handler(credential_id: String) -> Result<CredentialResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let credential_pk = crate::common::Partition::Credential(credential_id);
    let credential = Credential::get(cli, &credential_pk, Some(EntityType::Credential))
        .await?
        .ok_or(CredentialError::CredentialNotFound)?;

    if credential.account_id != account.pk {
        return Err(CredentialError::CredentialNotFound.into());
    }

    let credential = Credential::updater(credential.pk, credential.sk)
        .with_status(CredentialStatus::Revoked)
        .with_updated_at(crate::common::utils::time_utils::get_now())
        .execute(cli)
        .await?;

    Ok(credential.into())
}
