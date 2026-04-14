use dioxus::fullstack::axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use tower_sessions::Session;

use crate::common::{CommonConfig, EntityType, Error, Partition};
use crate::features::accounts::{Account, AccountError};
use crate::features::accounts::controllers::SESSION_KEY_ACCOUNT_ID;


/// Extract Account from request using session only.
impl<S> FromRequestParts<S> for Account
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(account) = parts.extensions.get::<Account>() {
            return Ok(account.clone());
        }

        let config = CommonConfig::default();
        let cli = config.dynamodb();

        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                crate::common::error!("no session found from request: {:?}", e);
                Error::NoSessionFound
            })?;

        let account = authenticate_by_session(&session, cli).await?;
        parts.extensions.insert(account.clone());
        Ok(account)
    }
}

pub(crate) async fn authenticate_by_session_from_parts<S>(
    parts: &mut Parts,
    state: &S,
    cli: &aws_sdk_dynamodb::Client,
) -> crate::common::Result<Account>
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    let session = Session::from_request_parts(parts, state)
        .await
        .map_err(|e| {
            crate::common::error!("no session found from request: {:?}", e);
            Error::NoSessionFound
        })?;

    authenticate_by_session(&session, cli).await
}

pub(crate) async fn authenticate_by_session(
    session: &Session,
    cli: &aws_sdk_dynamodb::Client,
) -> crate::common::Result<Account> {
    let account_pk: String = session
        .get(SESSION_KEY_ACCOUNT_ID)
        .await
        .map_err(|e| {
            crate::common::error!("no account id found from session: {:?}", e);
            Error::NoSessionFound
        })?
        .ok_or(Error::NoSessionFound)?;

    let partition: Partition = account_pk.parse().map_err(|_| Error::NoSessionFound)?;

    let account = Account::get(cli, &partition, Some(EntityType::Account))
        .await
        .map_err(|e| {
            crate::common::error!("failed to get account from db: {:?}", e);
            Error::NoSessionFound
        })?;

    match account {
        Some(acc) => Ok(acc),
        None => {
            let _ = session.flush().await;
            Err(AccountError::AccountNotFound.into())
        }
    }
}
