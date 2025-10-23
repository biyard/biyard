use by_axum::axum::http::request::Parts;
use tower_sessions::Session;

use crate::{
    features::{accounts::AccountType, session::SESSION_KEY_ACCOUNT_ID},
    *,
};

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, JsonSchema, OperationIo, Default,
)]
pub struct Account {
    #[schemars(description = "ID of the created account")]
    pub pk: Partition,
    #[schemars(description = "Entity type of the created account")]
    #[dynamo(index = "gsi2", sk, name = "find_by_email")]
    pub sk: EntityType,

    #[schemars(description = "Name of the created account")]
    pub name: String,

    #[dynamo(index = "gsi1", pk, prefix = "AC", name = "find_by_email_and_password")]
    #[dynamo(index = "gsi2", pk, prefix = "AC", name = "find_by_email")]
    #[schemars(description = "Email of the created account")]
    pub email: String,
    #[dynamo(index = "gsi1", sk, name = "find_by_email_and_password")]
    #[schemars(description = "Server-side Hashed password of the created account")]
    pub password: String,

    #[schemars(description = "Creation timestamp of the account")]
    pub created_at: i64,
    #[schemars(description = "Last update timestamp of the account")]
    pub updated_at: i64,

    #[schemars(description = "Type of the user account")]
    pub user_type: AccountType,
}

impl Account {
    pub fn new(name: String, email: String, password: String) -> Self {
        let now = time_utils::get_now();
        let uuid = uuid::Uuid::new_v4().to_string();

        Self {
            pk: Partition::Account(uuid),
            sk: EntityType::Account,
            name,
            email,
            password,
            created_at: now,
            updated_at: now,
            user_type: AccountType::User,
        }
    }
}

impl FromRequestParts<AppState> for Option<Account> {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> std::result::Result<Self, Self::Rejection> {
        tracing::debug!("extracting optional user from request parts");
        let session = Session::from_request_parts(parts, state).await;

        if let Err(_e) = &session {
            return Ok(None);
        }

        let session = session.unwrap();

        let account_pk: Partition = if let Ok(Some(u)) = session.get(SESSION_KEY_ACCOUNT_ID).await {
            tracing::debug!("found user id in session: {:?}", u);
            u
        } else {
            let _ = session.flush().await;
            return Ok(None);
        };

        let user = if let Ok(Some(u)) =
            Account::get(&state.cli, account_pk, Some(EntityType::Account)).await
        {
            u
        } else {
            let _ = session.flush().await;
            return Ok(None);
        };

        Ok(Some(user))
    }
}

// For authenticated routes where User must be present
impl FromRequestParts<AppState> for Account {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> std::result::Result<Self, Self::Rejection> {
        tracing::debug!("extracting user from request parts");
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                tracing::error!("no session found from request: {:?}", e);
                Error::NoSessionFound
            })?;

        let account_pk: Partition = session
            .get(SESSION_KEY_ACCOUNT_ID)
            .await
            .map_err(|e| {
                tracing::error!("no user id found from session: {:?}", e);
                Error::NoSessionFound
            })?
            .ok_or(Error::NoSessionFound)?;

        let account = Account::get(&state.cli, account_pk, Some(EntityType::Account))
            .await
            .map_err(|e| {
                tracing::error!("failed to get user from db: {:?}", e);
                Error::NoSessionFound
            });

        if account.is_err() {
            tracing::error!("no user found: {:?}", account);
            if let Err(e) = session.flush().await {
                tracing::error!("failed to flush session: {:?}", e);
            }
            return Err(Error::NoSessionFound);
        }

        let account = account.unwrap();

        if account.is_none() {
            if let Err(e) = session.flush().await {
                tracing::error!("failed to flush session: {:?}", e);
            }
            return Err(Error::AccountNotFound);
        }

        Ok(account.unwrap())
    }
}
