//! Phase 3 — Tokens lifecycle tests.
//!
//! Covers create/update/get/balance (non-chain paths). Deploy and mint
//! are chain-dependent and gated behind additional stub wiring — kept
//! out of this phase to avoid blowing up test scope.

mod common;

use common::{factories, test_server};
use reqwest::StatusCode;
use serde_json::json;

// ----- create_token -------------------------------------------------------

#[tokio::test]
async fn create_token_writes_initial_state() {
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
        "/v1/projects/{}/tokens",
        factories::project_id_segment(&seeded.project)
    );
    let res = client
        .post_json(
            &path,
            &json!({
                "name": "TestToken",
                "symbol": "TT",
                "description": "Phase 3 test",
                "monthly_emission": 1_000_000,
                "decay_rate_bps": 500,
                "distribution_slots": [],
                "start_month": "2026-04",
            }),
        )
        .await;
    assert_eq!(res.status, StatusCode::OK, "body: {}", res.body_text);
    assert!(res.body_text.contains("TestToken"));
    assert!(res.body_text.contains("TT"));
}

#[tokio::test]
async fn create_token_rejects_duplicate_for_same_project() {
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
        "/v1/projects/{}/tokens",
        factories::project_id_segment(&seeded.project)
    );
    let body = json!({
        "name": "TestToken",
        "symbol": "TT",
        "monthly_emission": 1_000_000,
        "decay_rate_bps": 500,
        "distribution_slots": [],
    });

    let first = client.post_json(&path, &body).await;
    assert_eq!(first.status, StatusCode::OK, "first: {}", first.body_text);

    let second = client.post_json(&path, &body).await;
    assert!(
        second.status.is_client_error(),
        "expected 4xx on duplicate, got {} — {}",
        second.status,
        second.body_text
    );
}

#[tokio::test]
async fn create_token_rejected_for_other_enterprise() {
    let server = test_server().await;
    let client = server.client();
    let enterprise_a = factories::new_enterprise_with_owner().await;
    let project_b = factories::new_enterprise_with_project().await;

    client
        .signin(
            &enterprise_a.owner.account.email,
            &enterprise_a.owner.password_plain,
        )
        .await;

    let path = format!(
        "/v1/projects/{}/tokens",
        factories::project_id_segment(&project_b.project)
    );
    let res = client
        .post_json(
            &path,
            &json!({
                "name": "Hijack",
                "symbol": "HJ",
                "monthly_emission": 100,
                "decay_rate_bps": 0,
                "distribution_slots": [],
            }),
        )
        .await;
    assert!(
        res.status == StatusCode::FORBIDDEN || res.status == StatusCode::NOT_FOUND,
        "expected cross-tenant rejection, got {} — {}",
        res.status,
        res.body_text
    );
}

// ----- get_token ----------------------------------------------------------

#[tokio::test]
async fn get_token_returns_none_when_absent() {
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
        "/v1/projects/{}/tokens",
        factories::project_id_segment(&seeded.project)
    );
    let res = client.get(&path).await;
    assert_eq!(res.status, StatusCode::OK, "body: {}", res.body_text);
    // Returns `null` / None when no token exists.
    assert!(
        res.body_text.trim() == "null" || res.body_text.trim().is_empty(),
        "expected null body, got: {}",
        res.body_text
    );
}

#[tokio::test]
async fn get_token_returns_created_token() {
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
        "/v1/projects/{}/tokens",
        factories::project_id_segment(&seeded.project)
    );
    let create = client
        .post_json(
            &path,
            &json!({
                "name": "GetToken",
                "symbol": "GT",
                "monthly_emission": 500,
                "decay_rate_bps": 250,
                "distribution_slots": [],
            }),
        )
        .await;
    assert_eq!(create.status, StatusCode::OK);

    let fetched = client.get(&path).await;
    assert_eq!(fetched.status, StatusCode::OK);
    assert!(fetched.body_text.contains("GetToken"));
    assert!(fetched.body_text.contains("\"symbol\":\"GT\""));
}

// ----- update_token -------------------------------------------------------

#[tokio::test]
async fn update_token_applies_partial_updates() {
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
        "/v1/projects/{}/tokens",
        factories::project_id_segment(&seeded.project)
    );
    let create = client
        .post_json(
            &path,
            &json!({
                "name": "Orig",
                "symbol": "OR",
                "monthly_emission": 100,
                "decay_rate_bps": 0,
                "distribution_slots": [],
            }),
        )
        .await;
    assert_eq!(create.status, StatusCode::OK);

    let update = client
        .put_json(
            &path,
            &json!({
                "description": "updated description",
                "monthly_emission": 200,
            }),
        )
        .await;
    assert_eq!(update.status, StatusCode::OK, "body: {}", update.body_text);
    assert!(update.body_text.contains("updated description"));
    assert!(update.body_text.contains("\"monthly_emission\":200"));
    // Name/symbol untouched.
    assert!(update.body_text.contains("\"name\":\"Orig\""));
    assert!(update.body_text.contains("\"symbol\":\"OR\""));
}

#[tokio::test]
async fn update_token_rejects_after_deployment() {
    let server = test_server().await;
    let client = server.client();
    let seeded = factories::new_enterprise_with_project().await;

    // Seed a token that is "already deployed" (has contract_address set).
    factories::new_deployed_token(&seeded.project, "2026-01").await;

    client
        .signin(
            &seeded.enterprise.owner.account.email,
            &seeded.enterprise.owner.password_plain,
        )
        .await;

    let path = format!(
        "/v1/projects/{}/tokens",
        factories::project_id_segment(&seeded.project)
    );
    let res = client
        .put_json(
            &path,
            &json!({
                "name": "NewName",
            }),
        )
        .await;
    assert!(
        res.status.is_client_error(),
        "expected 4xx AlreadyDeployed, got {} — {}",
        res.status,
        res.body_text
    );
    assert!(
        res.body_text.to_lowercase().contains("deployed")
            || res.body_text.to_lowercase().contains("already"),
        "expected AlreadyDeployed message, got: {}",
        res.body_text
    );
}

// ----- get_token_balance --------------------------------------------------

#[tokio::test]
async fn get_token_balance_not_found_for_unknown_user() {
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
        "/v1/projects/{}/tokens/balance/{}",
        factories::project_id_segment(&seeded.project),
        "no-such-user",
    );
    let res = client.get(&path).await;
    assert_eq!(res.status, StatusCode::NOT_FOUND, "body: {}", res.body_text);
}
