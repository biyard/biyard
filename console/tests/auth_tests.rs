//! Phase 1 — auth / tenancy boundary tests.

mod common;

use common::{factories, test_server};
use reqwest::StatusCode;

// ----- AccountAuth ---------------------------------------------------------

#[tokio::test]
async fn get_me_without_session_is_unauthorized() {
    let server = test_server().await;
    let client = server.client();

    let res = client.get("/v1/accounts/me").await;
    assert_eq!(
        res.status,
        StatusCode::UNAUTHORIZED,
        "body: {}",
        res.body_text
    );
}

#[tokio::test]
async fn get_me_with_valid_session_returns_account() {
    let server = test_server().await;
    let client = server.client();
    let seeded = factories::new_account(
        console::common::OrganizationRole::Owner,
        console::features::accounts::AccountType::User,
    )
    .await;

    client
        .signin(&seeded.account.email, &seeded.password_plain)
        .await;

    let res = client.get("/v1/accounts/me").await;
    assert_eq!(res.status, StatusCode::OK, "body: {}", res.body_text);
}

// ----- EnterpriseContextAuth ----------------------------------------------

#[tokio::test]
async fn list_projects_without_session_is_unauthorized() {
    let server = test_server().await;
    let client = server.client();

    let res = client.get("/v1/projects?limit=10").await;
    assert_eq!(res.status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn list_projects_with_enterprise_context_returns_scoped_list() {
    let server = test_server().await;
    let client = server.client();
    let seeded = factories::new_enterprise_with_project().await;

    client
        .signin(
            &seeded.enterprise.owner.account.email,
            &seeded.enterprise.owner.password_plain,
        )
        .await;

    let res = client.get("/v1/projects?limit=50").await;
    assert_eq!(res.status, StatusCode::OK, "body: {}", res.body_text);
    assert!(
        res.body_text.contains(&factories::project_id_segment(&seeded.project)),
        "expected own project in listing, got: {}",
        res.body_text
    );
}

// ----- ProjectAuth / ProjectViewerAuth (cross-tenant rejection) ------------

#[tokio::test]
async fn get_project_rejects_cross_tenant_access() {
    let server = test_server().await;
    let client = server.client();

    let enterprise_a = factories::new_enterprise_with_owner().await;
    let project_b = factories::new_enterprise_with_project().await; // different enterprise

    client
        .signin(
            &enterprise_a.owner.account.email,
            &enterprise_a.owner.password_plain,
        )
        .await;

    let path = format!(
        "/v1/projects/{}",
        factories::project_id_segment(&project_b.project)
    );
    let res = client.get(&path).await;
    assert!(
        res.status == StatusCode::FORBIDDEN || res.status == StatusCode::NOT_FOUND,
        "expected 403/404 cross-tenant, got {} — {}",
        res.status,
        res.body_text
    );
}

#[tokio::test]
async fn get_project_allows_same_tenant_access() {
    let server = test_server().await;
    let client = server.client();
    let seeded = factories::new_enterprise_with_project().await;

    client
        .signin(
            &seeded.enterprise.owner.account.email,
            &seeded.enterprise.owner.password_plain,
        )
        .await;

    let path = format!(
        "/v1/projects/{}",
        factories::project_id_segment(&seeded.project)
    );
    let res = client.get(&path).await;
    assert_eq!(res.status, StatusCode::OK, "body: {}", res.body_text);
}

// ----- CredentialAuth (Bearer token) --------------------------------------

#[tokio::test]
async fn bearer_with_invalid_api_key_is_unauthorized() {
    let server = test_server().await;
    let client = server.client();
    let seeded = factories::new_enterprise_with_project().await;

    let path = format!(
        "/v1/projects/{}",
        factories::project_id_segment(&seeded.project)
    );
    let res = client.get_with_bearer(&path, "definitely_not_a_real_key").await;
    assert_eq!(res.status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn bearer_with_revoked_credential_is_unauthorized() {
    let server = test_server().await;
    let client = server.client();
    let seeded = factories::new_enterprise_with_project().await;
    let credential = factories::new_credential_for(&seeded.enterprise).await;
    factories::revoke_credential(&credential.credential).await;

    let path = format!(
        "/v1/projects/{}",
        factories::project_id_segment(&seeded.project)
    );
    let res = client
        .get_with_bearer(&path, &credential.api_key_plain)
        .await;
    assert_eq!(res.status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn bearer_with_active_credential_resolves_own_project() {
    let server = test_server().await;
    let client = server.client();
    let seeded = factories::new_enterprise_with_project().await;
    let credential = factories::new_credential_for(&seeded.enterprise).await;

    let path = format!(
        "/v1/projects/{}",
        factories::project_id_segment(&seeded.project)
    );
    let res = client
        .get_with_bearer(&path, &credential.api_key_plain)
        .await;
    assert_eq!(res.status, StatusCode::OK, "body: {}", res.body_text);
}

#[tokio::test]
async fn bearer_credential_cannot_access_other_enterprise_project() {
    let server = test_server().await;
    let client = server.client();
    let enterprise_a = factories::new_enterprise_with_owner().await;
    let project_b = factories::new_enterprise_with_project().await;
    let credential = factories::new_credential_for(&enterprise_a).await;

    let path = format!(
        "/v1/projects/{}",
        factories::project_id_segment(&project_b.project)
    );
    let res = client
        .get_with_bearer(&path, &credential.api_key_plain)
        .await;
    assert!(
        res.status == StatusCode::FORBIDDEN || res.status == StatusCode::NOT_FOUND,
        "expected 403/404 cross-tenant bearer, got {} — {}",
        res.status,
        res.body_text
    );
}
