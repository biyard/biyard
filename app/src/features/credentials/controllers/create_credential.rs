use crate::common::{CommonConfig, Partition, Result};
use crate::features::accounts::Account;
use crate::features::credentials::{Credential, CredentialResponse};
use dioxus::prelude::post;

#[post("/v1/credentials", account: Account)]
pub async fn create_credential_handler(name: String) -> Result<CredentialResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let api_key = format!(
        "biyard_{}",
        uuid::Uuid::new_v4().to_string().replace("-", "")
    );

    let credential = Credential::new(account.pk, name, &api_key);
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
