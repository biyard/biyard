use crate::{
    features::accounts::AccountResponse, post, tests::test_context::TestContext,
    utils::password_utils::hash_password,
};

// ==================== Signup Tests ====================

#[tokio::test]
async fn test_signup_account_success() {
    let TestContext { app, ddb: _, now, .. } = TestContext::setup().await;
    let email = format!("signup-{}@biyard.co", now);
    let password = hash_password(&now.to_string());
    let name = format!("displayName{}", now);

    let (status, headers, body) = post! {
        app: &app,
        path: format!("/v1/accounts/signup"),
        body: {
            "email": email.clone(),
            "hashed_password": password,
            "name": name.clone(),
        },
        response_type: AccountResponse,
    };

    assert_eq!(status, 200);
    assert!(headers.get("set-cookie").is_some(), "Should set session cookie");
    assert!(body.pk.to_string().len() > 0, "Should return account ID");
    assert_eq!(body.email, email, "Email should match");
    assert_eq!(body.name, name, "Name should match");
    assert!(body.created_at > 0, "Created timestamp should be set");
}

#[tokio::test]
async fn test_signup_account_duplicate_email() {
    let TestContext { app, ddb: _, now, .. } = TestContext::setup().await;
    let email = format!("duplicate-{}@biyard.co", now);
    let password = hash_password(&now.to_string());
    let name = format!("displayName{}", now);

    // First signup - should succeed
    let (status, _, _) = post! {
        app: &app,
        path: format!("/v1/accounts/signup"),
        body: {
            "email": email.clone(),
            "hashed_password": password.clone(),
            "name": name.clone(),
        },
        response_type: AccountResponse,
    };
    assert_eq!(status, 200);

    // Second signup with same email - should fail
    let (status, _, _) = post! {
        app: &app,
        path: format!("/v1/accounts/signup"),
        body: {
            "email": email,
            "hashed_password": password,
            "name": format!("differentName{}", now),
        },
        response_type: serde_json::Value,
    };
    assert_eq!(status, 400, "Should return 400 for duplicate email");
}

// ==================== Signin Tests ====================

#[tokio::test]
async fn test_signin_account_success() {
    let TestContext { app, ddb: _, now, .. } = TestContext::setup().await;
    let email = format!("signin-{}@biyard.co", now);
    let plain_password = now.to_string();
    let name = format!("displayName{}", now);

    // Create account first (note: hashed_password field is misleadingly named - it expects plain password)
    let (signup_status, _, _) = post! {
        app: &app,
        path: format!("/v1/accounts/signup"),
        body: {
            "email": email.clone(),
            "hashed_password": plain_password.clone(),
            "name": name,
        },
        response_type: AccountResponse,
    };
    assert_eq!(signup_status, 200);

    // Now signin with the same plain password
    let (status, headers, body) = post! {
        app: &app,
        path: format!("/v1/accounts/signin"),
        body: {
            "email": email.clone(),
            "password": plain_password,
        },
        response_type: AccountResponse,
    };

    assert_eq!(status, 200);
    assert!(headers.get("set-cookie").is_some(), "Should set session cookie");
    assert!(body.pk.to_string().len() > 0, "Should return account ID");
    assert_eq!(body.email, email, "Email should match");
}

#[tokio::test]
async fn test_signin_account_invalid_email() {
    let TestContext { app, ddb: _, now, .. } = TestContext::setup().await;
    let email = format!("nonexistent-{}@biyard.co", now);
    let password = "somepassword";

    let (status, _, _) = post! {
        app: &app,
        path: format!("/v1/accounts/signin"),
        body: {
            "email": email,
            "password": password,
        },
        response_type: serde_json::Value,
    };

    assert_eq!(status, 401, "Should return 401 for non-existent email");
}

#[tokio::test]
async fn test_signin_account_invalid_password() {
    let TestContext { app, ddb: _, now, .. } = TestContext::setup().await;
    let email = format!("wrongpw-{}@biyard.co", now);
    let correct_password = now.to_string();
    let name = format!("displayName{}", now);

    // Create account
    let (signup_status, _, _) = post! {
        app: &app,
        path: format!("/v1/accounts/signup"),
        body: {
            "email": email.clone(),
            "hashed_password": correct_password,
            "name": name,
        },
        response_type: AccountResponse,
    };
    assert_eq!(signup_status, 200);

    // Try to signin with wrong password
    let (status, _, _) = post! {
        app: &app,
        path: format!("/v1/accounts/signin"),
        body: {
            "email": email,
            "password": "wrongpassword",
        },
        response_type: serde_json::Value,
    };

    assert_eq!(status, 401, "Should return 401 for wrong password");
}

// ==================== Withdrawal Tests ====================

#[tokio::test]
async fn test_withdrawal_account_success() {
    let TestContext { app, ddb, now, .. } = TestContext::setup().await;
    let email = format!("withdrawal-{}@biyard.co", now);
    let password = hash_password(&now.to_string());
    let name = format!("displayName{}", now);

    // Create account
    let (status, headers, signup_body) = post! {
        app: &app,
        path: format!("/v1/accounts/signup"),
        body: {
            "email": email.clone(),
            "hashed_password": password,
            "name": name.clone(),
        },
        response_type: AccountResponse,
    };
    assert_eq!(status, 200);

    // Get session cookie
    let session_cookie = headers
        .get("set-cookie")
        .expect("Should have session cookie")
        .to_str()
        .expect("Cookie should be valid string")
        .to_string();

    let mut headers_with_cookie = by_axum::axum::http::HeaderMap::new();
    headers_with_cookie.insert("cookie", session_cookie.parse().unwrap());

    // Withdraw account
    let (status, _, body) = post! {
        app: &app,
        path: format!("/v1/accounts/withdrawal"),
        headers: headers_with_cookie,
        response_type: AccountResponse,
    };

    assert_eq!(status, 200);
    assert_eq!(body.pk, signup_body.pk, "Should return same account ID");
    assert_eq!(body.email, email, "Email should match");
    assert_eq!(body.name, name, "Name should match");

    // Verify account is deleted from DynamoDB
    use crate::features::accounts::Account;
    use crate::EntityType;
    let deleted_account = Account::get(&ddb, &signup_body.pk, Some(EntityType::Account))
        .await
        .expect("DynamoDB query should succeed");
    assert!(deleted_account.is_none(), "Account should be deleted from database");
}

#[tokio::test]
async fn test_withdrawal_account_unauthorized() {
    let TestContext { app, ddb: _, now: _, .. } = TestContext::setup().await;

    // Try to withdraw without authentication
    let (status, _, _) = post! {
        app: &app,
        path: format!("/v1/accounts/withdrawal"),
        response_type: serde_json::Value,
    };

    // NoApi extractor returns 400 when session is missing or invalid
    assert_eq!(status, 400, "Should return 400 when not authenticated");
}
