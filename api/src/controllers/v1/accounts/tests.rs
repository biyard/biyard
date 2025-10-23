use crate::{
    features::accounts::AccountResponse, post, tests::test_context::TestContext,
    utils::password_utils::hash_password,
};

#[tokio::test]
async fn test_signup_account() {
    let TestContext { app, ddb: _, now, .. } = TestContext::setup().await;
    let email = format!("signup-{}@biyard.co", now);
    let password = hash_password(&now.to_string());
    let name = format!("displayName{}", now);

    let (status, headers, body) = post! {
        app: &app,
        path: format!("/v1/accounts/signup"),
        body: {
            "email": email,
            "hashed_password": password,
            "name": name,
        },
        response_type: AccountResponse,
    };

    assert_eq!(status, 200);
    assert!(headers.get("set-cookie").is_some());
    assert!(body.pk.to_string().len() > 0);
}
