use crate::common::Result;
use crate::features::credentials::CredentialResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EnterpriseContextAuth, EntityType, OrganizationRole};
#[cfg(feature = "server")]
use crate::features::credentials::{Credential, CredentialError, CredentialStatus};

#[delete("/v1/credentials/:credential_id", auth: EnterpriseContextAuth)]
pub async fn revoke_credential_handler(credential_id: String) -> Result<CredentialResponse> {
    // Revocation is the kill switch for a compromised API key and
    // must be available to the same privilege tier that can create
    // them. Admin or higher only.
    if !auth.role.allows(OrganizationRole::Admin) {
        return Err(crate::common::Error::Forbidden);
    }

    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let credential_pk = crate::common::Partition::Credential(credential_id);
    let credential = Credential::get(cli, &credential_pk, Some(EntityType::Credential))
        .await?
        .ok_or(CredentialError::CredentialNotFound)?;

    if credential.organization_id != auth.enterprise.pk && credential.account_id != auth.account.pk
    {
        return Err(CredentialError::CredentialNotFound.into());
    }

    let credential = Credential::updater(credential.pk, credential.sk)
        .with_status(CredentialStatus::Revoked)
        .with_updated_at(crate::common::utils::time_utils::get_now())
        .execute(cli)
        .await?;

    Ok(credential.into())
}
