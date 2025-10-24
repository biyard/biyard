use by_axum::axum::http::{header::AUTHORIZATION, request::Parts};
use tower_sessions::Session;

use crate::{
    features::{
        accounts::AccountType,
        credentials::{Credential, CredentialQueryOption, CredentialStatus},
        session::SESSION_KEY_ACCOUNT_ID,
    },
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

    pub async fn from_session(session: Session, state: &AppState) -> Result<Self> {
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

    pub async fn from_credential(auth_str: &str, state: &AppState) -> Result<Self> {
        tracing::debug!("attempting API key authentication");

        // Hash the API key to look it up
        let api_key_hash = password_utils::hash_password(auth_str);

        // Look up credential by API key hash using GSI
        let (credentials, _) = Credential::find_by_api_key_hash(
            &state.cli,
            &api_key_hash,
            CredentialQueryOption::builder().limit(1),
        )
        .await
        .map_err(|e| {
            tracing::error!("failed to query credential by api key: {:?}", e);
            Error::InvalidCredentials
        })?;

        if credentials.is_empty() {
            tracing::warn!("API key not found");
            return Err(Error::InvalidCredentials);
        }

        let credential = &credentials[0];

        // Check if credential is active
        if credential.status != CredentialStatus::Active {
            tracing::warn!("credential is not active: {:?}", credential.status);
            return Err(Error::InvalidCredentials);
        }

        // Update last_used_at timestamp (async, don't wait for it)
        let credential_pk = credential.pk.clone();
        let credential_sk = credential.sk.clone();
        let cli = state.cli.clone();
        tokio::spawn(async move {
            let _ = Credential::updater(credential_pk, credential_sk)
                .with_last_used_at(time_utils::get_now())
                .execute(&cli)
                .await;
        });

        // Get the account
        let account = Account::get(
            &state.cli,
            credential.account_id.clone(),
            Some(EntityType::Account),
        )
        .await
        .map_err(|e| {
            tracing::error!("failed to get account from db: {:?}", e);
            Error::AccountNotFound
        })?
        .ok_or_else(|| {
            tracing::error!("account not found for credential");
            Error::AccountNotFound
        })?;

        tracing::debug!(
            "successfully authenticated via API key for account: {:?}",
            account.pk
        );
        return Ok(account);
    }
}

impl FromRequestParts<AppState> for Option<Account> {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> std::result::Result<Self, Self::Rejection> {
        tracing::debug!("extracting optional user from request parts");

        Ok(Account::from_request_parts(parts, state).await.ok())
    }
}

// For authenticated routes where User must be present
// Supports both session-based auth (cookie) and API key auth (Bearer token)
impl FromRequestParts<AppState> for Account {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> std::result::Result<Self, Self::Rejection> {
        tracing::debug!("extracting user from request parts");

        // First, try API key authentication via Authorization header
        if let Some(auth_header) = parts.headers.get(AUTHORIZATION) {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    return Self::from_credential(auth_str.trim_start_matches("Bearer ").trim(), state).await;
                }
            }
        }

        // Fall back to session-based authentication
        tracing::debug!("attempting session-based authentication");
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                tracing::error!("no session found from request: {:?}", e);
                Error::NoSessionFound
            })?;

        Self::from_session(session, state).await
    }
}
