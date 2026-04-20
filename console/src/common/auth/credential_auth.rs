use aws_sdk_dynamodb::Client;

use crate::common::{CommonConfig, EntityType, Error};
use crate::features::accounts::{Account, AccountError, AccountType};
use crate::features::credentials::{
    Credential, CredentialError, CredentialQueryOption, CredentialStatus,
};

pub(crate) async fn authenticate_by_credential(
    api_key: &str,
    cli: &Client,
) -> crate::common::Result<Account> {
    let api_key_hash = crate::common::utils::password_utils::hash_secret_for_lookup(api_key);

    let (credentials, _) = Credential::find_by_api_key_hash(
        cli,
        &api_key_hash,
        CredentialQueryOption::builder().limit(1),
    )
    .await
    .map_err(|e| {
        crate::common::error!("failed to query credential by api key: {:?}", e);
        Error::from(CredentialError::InvalidApiKey)
    })?;

    if credentials.is_empty() {
        return Err(CredentialError::InvalidApiKey.into());
    }

    let credential = &credentials[0];

    if credential.status != CredentialStatus::Active {
        return Err(CredentialError::InvalidApiKey.into());
    }

    // Update last_used_at (fire-and-forget)
    let _ = Credential::updater(credential.pk.clone(), credential.sk.clone())
        .with_last_used_at(crate::common::utils::time_utils::get_now())
        .execute(cli)
        .await;

    // Get the account
    let account = Account::get(cli, &credential.account_id, Some(EntityType::Account))
        .await?
        .ok_or(AccountError::AccountNotFound)?;

    Ok(account)
}
