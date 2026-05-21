//! Direct-to-DynamoDB seeders for integration tests.
//!
//! These write rows with the same pk/sk shape as production code paths so
//! real handlers and extractors can read them back. Every factory uses
//! UUIDv7 so runs are isolated from each other within the shared
//! `biyard-test-main` table.

use console::common::{CommonConfig, EntityType, OrganizationRole, Partition};
use console::features::accounts::{Account, AccountType, PasswordScheme};
use console::features::credentials::{Credential, CredentialStatus};
use console::features::enterprises::Enterprise;
use console::features::points::{MonthlyPointAggregation, PointBalance};
use console::features::projects::{Project, ProjectStatus};
use console::features::tokens::ProjectToken;
use uuid::Uuid;

pub struct SeededAccount {
    pub account: Account,
    pub password_plain: String,
}

pub struct SeededEnterprise {
    pub owner: SeededAccount,
    pub enterprise: Enterprise,
}

pub struct SeededProject {
    pub enterprise: SeededEnterprise,
    pub project: Project,
}

pub struct SeededCredential {
    pub credential: Credential,
    pub api_key_plain: String,
}

const TEST_PASSWORD: &str = "TestP@ssw0rd!";

/// LocalStack's DynamoDB occasionally returns transient `unhandled error`s
/// under parallel load. Retry only for those; propagate deterministic
/// failures (ConditionalCheckFailedException, ValidationException, etc.)
/// immediately.
async fn with_retry<T, F, Fut>(label: &str, mut op: F) -> T
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, console::common::Error>>,
{
    let mut last_err = None;
    for attempt in 0..5 {
        match op().await {
            Ok(v) => return v,
            Err(e) => {
                let msg = format!("{e:?}");
                let is_transient = msg.contains("unhandled error")
                    && !msg.contains("ConditionalCheckFailed")
                    && !msg.contains("ValidationException");
                last_err = Some(e);
                if !is_transient {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(50 * (attempt + 1))).await;
            }
        }
    }
    panic!("{label}: {:?}", last_err.unwrap());
}

/// Read-after-write 검증 — DynamoEntity::get 이 Query 기반(eventually
/// consistent)이라 LocalStack 에서 막 PUT 한 row 가 즉시 안 보일 수 있다.
/// signin 직후 첫 핸들러가 auth extractor 단계에서 row 를 못 찾고 NotFound
/// 를 던지는 race 를 막기 위해, factory 마다 create 직후 polling 으로
/// 가시성을 확보한다.
async fn verify_read<T, F, Fut>(label: &str, mut probe: F)
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<Option<T>, console::common::Error>>,
{
    for attempt in 0..10 {
        match probe().await {
            Ok(Some(_)) => return,
            Ok(None) => {
                tokio::time::sleep(std::time::Duration::from_millis(20 * (attempt + 1))).await;
            }
            Err(e) => {
                let msg = format!("{e:?}");
                let is_transient = msg.contains("unhandled error");
                if !is_transient {
                    panic!("{label}: read probe failed: {e:?}");
                }
                tokio::time::sleep(std::time::Duration::from_millis(20 * (attempt + 1))).await;
            }
        }
    }
    panic!("{label}: write not visible after retries");
}

pub async fn new_account(role: OrganizationRole, user_type: AccountType) -> SeededAccount {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    let uuid = Uuid::now_v7().to_string();
    let email = format!("acc-{uuid}@biyard.test");
    let password_hash = bcrypt::hash(TEST_PASSWORD, bcrypt::DEFAULT_COST).expect("bcrypt hash");
    let now = chrono::Utc::now().timestamp_millis();

    let account = Account {
        pk: Partition::Account(uuid),
        sk: EntityType::Account,
        name: "Test Account".to_string(),
        email,
        password: password_hash,
        password_scheme: PasswordScheme::BcryptV1,
        enterprise_id: Partition::None,
        organization_role: role,
        created_at: now,
        updated_at: now,
        user_type,
    };

    with_retry("seed account", || {
        let a = account.clone();
        async move { a.create(cli).await }
    })
    .await;

    verify_read("account read-after-write", || {
        let pk = account.pk.clone();
        async move { Account::get(cli, &pk, Some(EntityType::Account)).await }
    })
    .await;

    SeededAccount {
        account,
        password_plain: TEST_PASSWORD.to_string(),
    }
}

pub async fn new_enterprise_with_owner() -> SeededEnterprise {
    new_enterprise_with_owner_role(OrganizationRole::Owner).await
}

pub async fn new_enterprise_with_owner_role(role: OrganizationRole) -> SeededEnterprise {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    let mut owner = new_account(role, AccountType::User).await;
    let enterprise_uuid = Uuid::now_v7().to_string();
    let enterprise_pk = Partition::Enterprise(enterprise_uuid);

    let enterprise = Enterprise::new(
        enterprise_pk.clone(),
        owner.account.pk.clone(),
        "Test Enterprise".to_string(),
    );
    with_retry("seed enterprise", || {
        let e = enterprise.clone();
        async move { e.create(cli).await }
    })
    .await;

    verify_read("enterprise read-after-write", || {
        let pk = enterprise.pk.clone();
        async move { Enterprise::get(cli, &pk, Some(EntityType::Enterprise)).await }
    })
    .await;

    // Link account back to the enterprise.
    let updated = with_retry("link account to enterprise", || {
        let pk = owner.account.pk.clone();
        let ent_pk = enterprise_pk.clone();
        async move {
            Account::updater(pk, EntityType::Account)
                .with_enterprise_id(ent_pk)
                .with_organization_role(role)
                .execute(cli)
                .await
        }
    })
    .await;
    owner.account = updated;

    // 업데이트 직후에도 같은 race 가 가능하다 — enterprise_id 가 실제로
    // 보일 때까지 polling.
    verify_read("account link read-after-write", || {
        let pk = owner.account.pk.clone();
        async move {
            let row = Account::get(cli, &pk, Some(EntityType::Account)).await?;
            Ok(row.filter(|a| a.enterprise_id != Partition::None))
        }
    })
    .await;

    SeededEnterprise {
        owner,
        enterprise,
    }
}

pub async fn new_project(ent: &SeededEnterprise) -> Project {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    let project = Project::new(
        ent.owner.account.pk.clone(),
        ent.enterprise.pk.clone(),
        "Test Project".to_string(),
        Some("Seeded by integration tests".to_string()),
        10_000,
        None,
        0.1,
    );

    with_retry("seed project", || {
        let p = project.clone();
        async move { p.create(cli).await }
    })
    .await;

    verify_read("project read-after-write", || {
        let pk = project.pk.clone();
        async move { Project::get(cli, &pk, Some(EntityType::Project)).await }
    })
    .await;

    project
}

pub async fn new_enterprise_with_project() -> SeededProject {
    let ent = new_enterprise_with_owner().await;
    let project = new_project(&ent).await;
    SeededProject {
        enterprise: ent,
        project,
    }
}

pub async fn new_credential_for(enterprise: &SeededEnterprise) -> SeededCredential {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    // Bearer token callers will hand back the raw api key; we keep it
    // alongside the DynamoDB row (which only stores the SHA3 hash).
    let api_key_plain = format!("by_test_{}", Uuid::now_v7().simple());

    let credential = Credential::new(
        enterprise.owner.account.pk.clone(),
        enterprise.enterprise.pk.clone(),
        "Test API Key".to_string(),
        &api_key_plain,
    );

    with_retry("seed credential", || {
        let c = credential.clone();
        async move { c.create(cli).await }
    })
    .await;

    verify_read("credential read-after-write", || {
        let pk = credential.pk.clone();
        async move { Credential::get(cli, &pk, Some(EntityType::Credential)).await }
    })
    .await;

    SeededCredential {
        credential,
        api_key_plain,
    }
}

pub async fn revoke_credential(credential: &Credential) {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    let _ = with_retry("revoke credential", || {
        let pk = credential.pk.clone();
        async move {
            Credential::updater(pk, EntityType::Credential)
                .with_status(CredentialStatus::Revoked)
                .execute(cli)
                .await
        }
    })
    .await;
}

pub fn project_id_segment(project: &Project) -> String {
    match &project.pk {
        Partition::Project(id) => id.clone(),
        other => panic!("expected Project partition, got {other:?}"),
    }
}

pub fn _force_active(_: ProjectStatus) {}

/// Seed a `ProjectToken` with a fake deployed contract address so Claim
/// handlers pass the "not deployed" guard. Chain-specific reads
/// (current_month, monthly_ceiling) still go through the stub.
pub async fn new_deployed_token(project: &Project, start_month: &str) -> ProjectToken {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();
    let now = chrono::Utc::now().timestamp_millis();

    let token = ProjectToken {
        pk: project.pk.clone(),
        sk: EntityType::Token,
        name: "TestToken".to_string(),
        symbol: "TT".to_string(),
        circulating_supply: 0,
        description: Some("Test token".to_string()),
        // Fake address — chain reads go through the stub under `disable-chain`.
        contract_address: Some("0x0000000000000000000000000000000000000001".to_string()),
        treasury_contract_address: Some("0x0000000000000000000000000000000000000002".to_string()),
        multisig_address: Some("0x0000000000000000000000000000000000000003".to_string()),
        stable_token_address: Some("0x0000000000000000000000000000000000000004".to_string()),
        chain_id: Some(1001),
        deployment_tx_hash: Some("0xdeadbeef".to_string()),
        treasury_deployment_tx_hash: Some("0xdeadbeef".to_string()),
        multisig_deployment_tx_hash: Some("0xdeadbeef".to_string()),
        treasury_reserve_bps: 0,
        monthly_emission: 1_000_000,
        decay_rate_bps: 0,
        distribution_slots: vec![],
        last_minted_month: None,
        deploying: false,
        start_month: Some(start_month.to_string()),
        created_at: now,
        updated_at: now,
    };
    with_retry("seed token", || {
        let t = token.clone();
        async move { t.create(cli).await }
    })
    .await;

    verify_read("token read-after-write", || {
        let pk = token.pk.clone();
        async move { ProjectToken::get(cli, &pk, Some(EntityType::Token)).await }
    })
    .await;

    token
}

pub async fn new_point_balance(
    project: &Project,
    meta_user_id: &str,
    month: &str,
    balance: i64,
    total_earned: i64,
) -> PointBalance {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();
    let now = chrono::Utc::now().timestamp_millis();

    let (pk, sk) = PointBalance::keys(
        project.pk.clone(),
        meta_user_id.to_string(),
        month.to_string(),
    );
    let bal = PointBalance {
        pk,
        sk,
        project_id: project.pk.clone(),
        month: month.to_string(),
        meta_user_id: meta_user_id.to_string(),
        balance,
        total_earned,
        total_spent: total_earned - balance,
        updated_at: now,
    };
    with_retry("seed point balance", || {
        let b = bal.clone();
        async move { b.create(cli).await }
    })
    .await;

    verify_read("point balance read-after-write", || {
        let (pk, sk) = PointBalance::keys(
            project.pk.clone(),
            meta_user_id.to_string(),
            month.to_string(),
        );
        async move { PointBalance::get(cli, &pk, Some(sk)).await }
    })
    .await;

    bal
}

pub async fn new_monthly_aggregation(
    project: &Project,
    month: &str,
    awarded_points: i64,
) -> MonthlyPointAggregation {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();
    let now = chrono::Utc::now().timestamp_millis();

    let (pk, sk) = MonthlyPointAggregation::keys(project.pk.clone(), month.to_string());
    let agg = MonthlyPointAggregation {
        pk,
        sk,
        supplied_points: awarded_points,
        traded_points: 0,
        awarded_points,
        deducted_points: 0,
        exchanged_points: 0,
        updated_at: now,
    };
    with_retry("seed aggregation", || {
        let a = agg.clone();
        async move { a.create(cli).await }
    })
    .await;

    verify_read("aggregation read-after-write", || {
        let (pk, sk) = MonthlyPointAggregation::keys(project.pk.clone(), month.to_string());
        async move { MonthlyPointAggregation::get(cli, &pk, Some(sk)).await }
    })
    .await;

    agg
}

pub async fn fetch_point_balance(
    project: &Project,
    meta_user_id: &str,
    month: &str,
) -> Option<PointBalance> {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    let (pk, sk) = PointBalance::keys(
        project.pk.clone(),
        meta_user_id.to_string(),
        month.to_string(),
    );
    PointBalance::get(cli, pk, Some(sk)).await.ok().flatten()
}

pub async fn fetch_monthly_aggregation(
    project: &Project,
    month: &str,
) -> Option<MonthlyPointAggregation> {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    let (pk, sk) = MonthlyPointAggregation::keys(project.pk.clone(), month.to_string());
    MonthlyPointAggregation::get(cli, pk, Some(sk))
        .await
        .ok()
        .flatten()
}
