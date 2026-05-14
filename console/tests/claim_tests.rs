//! Phase 2b — Claim lifecycle tests.
//!
//! These cover the logic driven by the external PaaS consumer: balance
//! deduction atomicity, double-claim prevention, month-not-ended guard,
//! and cross-tenant rejection. On-chain reads (`current_month`,
//! `monthly_ceiling`) are redirected to `chain_stub` via the
//! `disable-chain` feature.
//!
//! `chain_stub` is a process-wide mutable state so these tests serialise
//! their access with a `Mutex`.

#![cfg(feature = "disable-chain")]

mod common;

use common::{factories, test_server};
use console::common::blockchain::chain_stub;
use once_cell::sync::Lazy;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

static CHAIN_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

#[derive(Debug, Deserialize)]
struct ClaimableResponse {
    months: Vec<ClaimableMonth>,
}
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ClaimableMonth {
    month: String,
    user_points: i64,
    total_points: i64,
    claimable_tokens: String,
    already_claimed: String,
    remaining: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ClaimSignatureResponse {
    month_index: String,
    amount: String,
    max_claimable: String,
    nonce: String,
    deadline: String,
    signature: String,
    contract_address: String,
    chain_id: u64,
}

const MONTH_CEILING: u128 = 1_000_000_000_000_000_000_000; // 1_000 tokens @ 18 decimals
const MONTH_STR: &str = "2026-02";
const START_MONTH: &str = "2026-01";
const META_USER: &str = "meta-user-claim-test";

// ----- get_claimable_handler ---------------------------------------------

#[tokio::test]
async fn claimable_rejects_when_month_not_ended() {
    let _guard = CHAIN_LOCK.lock().await;
    let server = test_server().await;
    let client = server.client();

    let seeded = factories::new_enterprise_with_project().await;
    factories::new_deployed_token(&seeded.project, START_MONTH).await;
    factories::new_point_balance(&seeded.project, META_USER, MONTH_STR, 100, 100).await;
    factories::new_monthly_aggregation(&seeded.project, MONTH_STR, 100).await;

    chain_stub::reset();
    // month index for MONTH_STR vs START_MONTH is 1. Stub says "current = 1"
    // → the balance's month (index 1) is NOT strictly less than current,
    //   so claimable must exclude it.
    chain_stub::set_current_month(1);
    chain_stub::set_monthly_ceiling(1, MONTH_CEILING);

    client
        .signin(
            &seeded.enterprise.owner.account.email,
            &seeded.enterprise.owner.password_plain,
        )
        .await;

    let path = format!(
        "/v1/projects/{}/tokens/claimable?meta_user_id={}",
        factories::project_id_segment(&seeded.project),
        META_USER,
    );
    let res = client.get(&path).await;
    assert_eq!(res.status, StatusCode::OK, "body: {}", res.body_text);
    let body: ClaimableResponse = res.json();
    assert!(
        body.months.is_empty(),
        "expected no claimable months while month is still current, got {:?}",
        body.months
    );
}

#[tokio::test]
async fn claimable_includes_ended_month() {
    let _guard = CHAIN_LOCK.lock().await;
    let server = test_server().await;
    let client = server.client();

    let seeded = factories::new_enterprise_with_project().await;
    factories::new_deployed_token(&seeded.project, START_MONTH).await;
    factories::new_point_balance(&seeded.project, META_USER, MONTH_STR, 100, 100).await;
    factories::new_monthly_aggregation(&seeded.project, MONTH_STR, 100).await;

    chain_stub::reset();
    // Move on-chain current_month to 2; MONTH_STR (index 1) is now past.
    chain_stub::set_current_month(2);
    chain_stub::set_monthly_ceiling(1, MONTH_CEILING);

    client
        .signin(
            &seeded.enterprise.owner.account.email,
            &seeded.enterprise.owner.password_plain,
        )
        .await;

    let path = format!(
        "/v1/projects/{}/tokens/claimable?meta_user_id={}",
        factories::project_id_segment(&seeded.project),
        META_USER,
    );
    let res = client.get(&path).await;
    assert_eq!(res.status, StatusCode::OK, "body: {}", res.body_text);
    let body: ClaimableResponse = res.json();
    assert_eq!(body.months.len(), 1, "expected one claimable month");
    assert_eq!(body.months[0].month, MONTH_STR);
    assert_eq!(body.months[0].user_points, 100);
    // user_pool_bps = 10000 (no distribution slots) → full ceiling.
    assert_eq!(body.months[0].remaining, MONTH_CEILING.to_string());
}

// ----- get_claim_signature_handler ---------------------------------------

#[tokio::test]
async fn claim_signature_rejects_month_not_ended() {
    let _guard = CHAIN_LOCK.lock().await;
    let server = test_server().await;
    let client = server.client();

    let seeded = factories::new_enterprise_with_project().await;
    factories::new_deployed_token(&seeded.project, START_MONTH).await;
    factories::new_point_balance(&seeded.project, META_USER, MONTH_STR, 100, 100).await;
    factories::new_monthly_aggregation(&seeded.project, MONTH_STR, 100).await;

    chain_stub::reset();
    chain_stub::set_current_month(1); // target month = 1, still current
    chain_stub::set_monthly_ceiling(1, MONTH_CEILING);

    client
        .signin(
            &seeded.enterprise.owner.account.email,
            &seeded.enterprise.owner.password_plain,
        )
        .await;

    let path = format!(
        "/v1/projects/{}/tokens/claim-signature",
        factories::project_id_segment(&seeded.project)
    );
    let res = client
        .post_json(
            &path,
            &json!({
                "meta_user_id": META_USER,
                "month": MONTH_STR,
                "wallet_address": "0x000000000000000000000000000000000000dead",
            }),
        )
        .await;
    assert!(
        res.status.is_client_error() || res.status.is_server_error(),
        "expected error for month-not-ended, got {} — {}",
        res.status,
        res.body_text
    );
    assert!(
        res.body_text.contains("not ended") || res.body_text.to_lowercase().contains("month"),
        "expected month-not-ended message, got: {}",
        res.body_text
    );
}

#[tokio::test]
async fn claim_signature_deducts_balance_atomically() {
    let _guard = CHAIN_LOCK.lock().await;
    let server = test_server().await;
    let client = server.client();

    let seeded = factories::new_enterprise_with_project().await;
    factories::new_deployed_token(&seeded.project, START_MONTH).await;
    factories::new_point_balance(&seeded.project, META_USER, MONTH_STR, 100, 100).await;
    factories::new_monthly_aggregation(&seeded.project, MONTH_STR, 100).await;

    chain_stub::reset();
    chain_stub::set_current_month(2); // month 1 ended
    chain_stub::set_monthly_ceiling(1, MONTH_CEILING);

    client
        .signin(
            &seeded.enterprise.owner.account.email,
            &seeded.enterprise.owner.password_plain,
        )
        .await;

    let path = format!(
        "/v1/projects/{}/tokens/claim-signature",
        factories::project_id_segment(&seeded.project)
    );
    let res = client
        .post_json(
            &path,
            &json!({
                "meta_user_id": META_USER,
                "month": MONTH_STR,
                "wallet_address": "0x000000000000000000000000000000000000dead",
            }),
        )
        .await;
    assert_eq!(res.status, StatusCode::OK, "body: {}", res.body_text);
    let body: ClaimSignatureResponse = res.json();
    assert_eq!(body.amount, MONTH_CEILING.to_string());

    // DB state after success:
    let bal = factories::fetch_point_balance(&seeded.project, META_USER, MONTH_STR)
        .await
        .expect("balance row");
    assert_eq!(bal.balance, 0, "balance must be zero after full claim");
    assert_eq!(bal.total_spent, 100, "total_spent must reflect deduction");

    let agg = factories::fetch_monthly_aggregation(&seeded.project, MONTH_STR)
        .await
        .expect("aggregation row");
    assert_eq!(agg.exchanged_points, 100);
}

#[tokio::test]
async fn claim_signature_double_claim_is_rejected() {
    let _guard = CHAIN_LOCK.lock().await;
    let server = test_server().await;
    let client = server.client();

    let seeded = factories::new_enterprise_with_project().await;
    factories::new_deployed_token(&seeded.project, START_MONTH).await;
    factories::new_point_balance(&seeded.project, META_USER, MONTH_STR, 100, 100).await;
    factories::new_monthly_aggregation(&seeded.project, MONTH_STR, 100).await;

    chain_stub::reset();
    chain_stub::set_current_month(2);
    chain_stub::set_monthly_ceiling(1, MONTH_CEILING);

    client
        .signin(
            &seeded.enterprise.owner.account.email,
            &seeded.enterprise.owner.password_plain,
        )
        .await;

    let path = format!(
        "/v1/projects/{}/tokens/claim-signature",
        factories::project_id_segment(&seeded.project)
    );
    let body = json!({
        "meta_user_id": META_USER,
        "month": MONTH_STR,
        "wallet_address": "0x000000000000000000000000000000000000dead",
    });

    let first = client.post_json(&path, &body).await;
    assert_eq!(first.status, StatusCode::OK, "first claim: {}", first.body_text);

    let second = client.post_json(&path, &body).await;
    assert!(
        second.status.is_client_error() || second.status.is_server_error(),
        "expected double-claim rejection, got {} — {}",
        second.status,
        second.body_text
    );
    assert!(
        second.body_text.to_lowercase().contains("no remaining")
            || second.body_text.to_lowercase().contains("no points"),
        "expected no-remaining-points message, got: {}",
        second.body_text
    );
}

#[tokio::test]
async fn claim_signature_rejects_cross_tenant() {
    let _guard = CHAIN_LOCK.lock().await;
    let server = test_server().await;
    let client = server.client();

    let enterprise_a = factories::new_enterprise_with_owner().await;
    let project_b = factories::new_enterprise_with_project().await;
    factories::new_deployed_token(&project_b.project, START_MONTH).await;
    factories::new_point_balance(&project_b.project, META_USER, MONTH_STR, 100, 100).await;
    factories::new_monthly_aggregation(&project_b.project, MONTH_STR, 100).await;

    chain_stub::reset();
    chain_stub::set_current_month(2);
    chain_stub::set_monthly_ceiling(1, MONTH_CEILING);

    // Sign in as enterprise A, try to claim on project B.
    client
        .signin(
            &enterprise_a.owner.account.email,
            &enterprise_a.owner.password_plain,
        )
        .await;

    let path = format!(
        "/v1/projects/{}/tokens/claim-signature",
        factories::project_id_segment(&project_b.project)
    );
    let res = client
        .post_json(
            &path,
            &json!({
                "meta_user_id": META_USER,
                "month": MONTH_STR,
                "wallet_address": "0x000000000000000000000000000000000000dead",
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

#[tokio::test]
async fn claim_signature_rejects_zero_balance() {
    let _guard = CHAIN_LOCK.lock().await;
    let server = test_server().await;
    let client = server.client();

    let seeded = factories::new_enterprise_with_project().await;
    factories::new_deployed_token(&seeded.project, START_MONTH).await;
    // Balance row exists but has zero remaining.
    factories::new_point_balance(&seeded.project, META_USER, MONTH_STR, 0, 50).await;
    factories::new_monthly_aggregation(&seeded.project, MONTH_STR, 50).await;

    chain_stub::reset();
    chain_stub::set_current_month(2);
    chain_stub::set_monthly_ceiling(1, MONTH_CEILING);

    client
        .signin(
            &seeded.enterprise.owner.account.email,
            &seeded.enterprise.owner.password_plain,
        )
        .await;

    let path = format!(
        "/v1/projects/{}/tokens/claim-signature",
        factories::project_id_segment(&seeded.project)
    );
    let res = client
        .post_json(
            &path,
            &json!({
                "meta_user_id": META_USER,
                "month": MONTH_STR,
                "wallet_address": "0x000000000000000000000000000000000000dead",
            }),
        )
        .await;
    assert!(
        res.status.is_client_error() || res.status.is_server_error(),
        "expected rejection for zero balance, got {} — {}",
        res.status,
        res.body_text
    );
}
