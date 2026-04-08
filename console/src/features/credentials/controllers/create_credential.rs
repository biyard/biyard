use crate::common::Result;
use crate::features::credentials::CredentialResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::EnterpriseContextAuth;
#[cfg(feature = "server")]
use crate::common::OrganizationRole;
#[cfg(feature = "server")]
use crate::common::{CommonConfig, Partition};
#[cfg(feature = "server")]
use crate::features::credentials::Credential;

#[post("/v1/credentials", auth: EnterpriseContextAuth)]
pub async fn create_credential_handler(name: String) -> Result<CredentialResponse> {
    // API key issuance is a privileged action: a leaked key grants
    // full API-level access to the enterprise's data. Only Admin or
    // higher may create credentials.
    if !auth.role.allows(OrganizationRole::Admin) {
        return Err(crate::common::Error::Forbidden);
    }

    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let api_key = format!(
        "biyard_{}",
        uuid::Uuid::now_v7().to_string().replace("-", "")
    );

    let credential = Credential::new(auth.account.pk, auth.enterprise.pk, name, &api_key);
    credential.create(cli).await?;

    let response = CredentialResponse {
        id: match &credential.pk {
            Partition::Credential(id) => id.clone(),
            _ => String::new(),
        },
        name: credential.name,
        api_key_prefix: credential.api_key_prefix,
        status: credential.status,
        created_at: credential.created_at,
        last_used_at: credential.last_used_at,
        api_key,
    };

    Ok(response)
}
