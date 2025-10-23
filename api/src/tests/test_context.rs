use crate::features::accounts::{AccountResponse, AccountType};
use crate::utils::password_utils::hash_password;
use crate::*;
use crate::{api_main, features::accounts::Account};
use by_axum::aide::axum::ApiRouter;
use by_axum::axum::{self, Extension, Router};
use std::sync::Arc;
use std::time::SystemTime;

#[derive(Clone)]
pub struct TestContext {
    pub app: axum::AxumRouter,
    pub app_state: AppState,
    pub now: u64,
    pub ddb: aws_sdk_dynamodb::Client,
    pub account1: (Account, axum::http::HeaderMap),
    pub account2: (Account, axum::http::HeaderMap),
    pub admin: (Account, axum::http::HeaderMap),
}

impl TestContext {
    pub async fn setup() -> Self {
        setup().await
    }

    pub async fn create_new_user(&self) -> (Account, axum::http::HeaderMap) {
        create_account_session(self.app.clone(), &self.ddb).await
    }
}

pub async fn setup() -> TestContext {
    let app = api_main::api_main().await.unwrap();
    let app = by_axum::finishing(app);

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64
        - 1750000000u64;

    let conf = config::get();
    let app_state = AppState::new(&conf);
    let ddb = app_state.cli.clone();
    let (user, headers) = create_account_session(app.clone(), &ddb).await;
    let (user2, headers2) = create_account_session(app.clone(), &ddb).await;

    // Create admin user
    let (mut admin, admin_headers) = create_account_session(app.clone(), &ddb).await;
    admin.user_type = AccountType::SystemAdmin;
    Account::updater(admin.pk.clone(), admin.sk.clone())
        .with_user_type(AccountType::SystemAdmin)
        .execute(&ddb)
        .await
        .unwrap();

    TestContext {
        app,
        app_state,
        now,
        ddb,
        account1: (user, headers),
        account2: (user2, headers2),
        admin: (admin, admin_headers),
    }
}

pub async fn create_account_session(
    app: axum::AxumRouter,
    cli: &aws_sdk_dynamodb::Client,
) -> (Account, axum::http::HeaderMap) {
    let uid = uuid::Uuid::new_v4().to_string();

    let email = format!("setup-{}@biyard.co", uid);
    let password = hash_password(&uid);

    // For mocking user.
    let (_, header, account) = post! {
        app: &app,
        path: "/v1/accounts/signup",
        body: {
            "email": email,
            "hashed_password": password,
            "name": format!("displayName{}", uid),
        },
        response_type: AccountResponse,
    };

    let account = Account::get(cli, &account.pk, Some(EntityType::Account))
        .await
        .expect("Failed to get account from DynamoDB")
        .expect("Account not found in DynamoDB");

    let session_cookie = header
        .get("set-cookie")
        .expect("No set-cookie header found")
        .to_str()
        .expect("Failed to convert set-cookie header to str")
        .to_string();

    let mut headers = axum::http::HeaderMap::new();
    headers.insert("cookie", session_cookie.parse().unwrap());
    (account, headers)
}
