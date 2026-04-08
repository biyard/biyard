use crate::common::Result;
use crate::features::enterprises::CurrentEnterpriseResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, OrganizationRole, Partition};
#[cfg(feature = "server")]
use crate::features::accounts::Account;
#[cfg(feature = "server")]
use crate::features::credentials::{Credential, CredentialQueryOption};
#[cfg(feature = "server")]
use crate::features::enterprises::Enterprise;
#[cfg(feature = "server")]
use crate::features::projects::{Project, ProjectQueryOption};

#[get("/v1/enterprises/current", account: Account)]
pub async fn get_current_enterprise_handler() -> Result<CurrentEnterpriseResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let (account, enterprise) = ensure_current_enterprise_for_account(cli, &account).await?;

    Ok(CurrentEnterpriseResponse {
        enterprise: enterprise.into(),
        role: account.organization_role,
    })
}

#[cfg(feature = "server")]
pub async fn ensure_current_enterprise_for_account(
    cli: &aws_sdk_dynamodb::Client,
    account: &Account,
) -> Result<(Account, Enterprise)> {
    if let Some((role, mut enterprise)) = load_existing_enterprise(cli, account).await? {
        if should_backfill_legacy_resources(&enterprise, account) {
            backfill_legacy_resources(cli, account, &enterprise).await?;
            enterprise = mark_legacy_sync_complete(cli, &enterprise).await?;
        }
        let account = sync_account_enterprise(cli, account, &enterprise, role).await?;
        return Ok((account, enterprise));
    }

    // Personal-enterprise naming convention. Future migration to
    // multi-membership uses this prefix to distinguish auto-created
    // workspaces from team-created ones.
    let enterprise = Enterprise::new(
        personal_enterprise_partition(account),
        account.pk.clone(),
        format!("{} Personal", account.name),
    );

    enterprise.create(cli).await?;
    backfill_legacy_resources(cli, account, &enterprise).await?;
    let enterprise = mark_legacy_sync_complete(cli, &enterprise).await?;
    let account =
        sync_account_enterprise(cli, account, &enterprise, OrganizationRole::Owner).await?;

    Ok((account, enterprise))
}

#[cfg(feature = "server")]
async fn load_existing_enterprise(
    cli: &aws_sdk_dynamodb::Client,
    account: &Account,
) -> Result<Option<(OrganizationRole, Enterprise)>> {
    // Single-membership: an Account is always tied to at most one
    // Enterprise. Two paths can locate that enterprise:
    //   1. Account.enterprise_id explicitly set (post-backfill).
    //   2. The deterministic personal enterprise key derived from
    //      account.pk (legacy auto-create flow).
    if let Some(current) = load_account_enterprise(cli, account).await? {
        return Ok(Some(current));
    }

    load_personal_enterprise(cli, account).await
}

#[cfg(feature = "server")]
async fn load_account_enterprise(
    cli: &aws_sdk_dynamodb::Client,
    account: &Account,
) -> Result<Option<(OrganizationRole, Enterprise)>> {
    if matches!(account.enterprise_id, Partition::None) {
        return Ok(None);
    }

    let enterprise =
        Enterprise::get(cli, &account.enterprise_id, Some(EntityType::Enterprise)).await?;

    Ok(enterprise.map(|enterprise| (account.organization_role, enterprise)))
}

#[cfg(feature = "server")]
async fn load_personal_enterprise(
    cli: &aws_sdk_dynamodb::Client,
    account: &Account,
) -> Result<Option<(OrganizationRole, Enterprise)>> {
    let enterprise_pk = personal_enterprise_partition(account);
    let enterprise = Enterprise::get(cli, &enterprise_pk, Some(EntityType::Enterprise)).await?;

    Ok(enterprise.map(|enterprise| (OrganizationRole::Owner, enterprise)))
}

#[cfg(feature = "server")]
fn personal_enterprise_partition(account: &Account) -> Partition {
    let enterprise_id = match &account.pk {
        Partition::Account(id) => format!("acct-{id}"),
        _ => "acct-default".to_string(),
    };

    Partition::Enterprise(enterprise_id)
}

#[cfg(feature = "server")]
async fn backfill_legacy_resources(
    cli: &aws_sdk_dynamodb::Client,
    account: &Account,
    enterprise: &Enterprise,
) -> Result<()> {
    backfill_projects(cli, account, enterprise).await?;
    backfill_credentials(cli, account, enterprise).await?;
    Ok(())
}

#[cfg(feature = "server")]
async fn sync_account_enterprise(
    cli: &aws_sdk_dynamodb::Client,
    account: &Account,
    enterprise: &Enterprise,
    role: OrganizationRole,
) -> Result<Account> {
    if account.enterprise_id == enterprise.pk && account.organization_role == role {
        return Ok(account.clone());
    }

    let updated = Account::updater(account.pk.clone(), account.sk.clone())
        .with_enterprise_id(enterprise.pk.clone())
        .with_organization_role(role)
        .with_updated_at(crate::common::utils::time_utils::get_now())
        .execute(cli)
        .await?;

    Ok(updated)
}

#[cfg(feature = "server")]
async fn backfill_projects(
    cli: &aws_sdk_dynamodb::Client,
    account: &Account,
    enterprise: &Enterprise,
) -> Result<()> {
    let mut bookmark: Option<String> = None;

    loop {
        let mut query = ProjectQueryOption::builder().limit(200);
        if let Some(ref current_bookmark) = bookmark {
            query = query.bookmark(current_bookmark.clone());
        }

        let (projects, next_bookmark) =
            Project::find_by_account_id(cli, &account.pk, query).await?;

        for project in projects
            .into_iter()
            .filter(|project| matches!(project.organization_id, Partition::None))
        {
            Project::updater(project.pk.clone(), project.sk.clone())
                .with_organization_id(enterprise.pk.clone())
                .with_updated_at(crate::common::utils::time_utils::get_now())
                .execute(cli)
                .await?;
        }

        if next_bookmark.is_none() {
            break;
        }
        bookmark = next_bookmark;
    }

    Ok(())
}

#[cfg(feature = "server")]
fn should_backfill_legacy_resources(enterprise: &Enterprise, account: &Account) -> bool {
    enterprise.owner_account_id == account.pk && enterprise.legacy_account_sync_at.is_none()
}

#[cfg(feature = "server")]
async fn mark_legacy_sync_complete(
    cli: &aws_sdk_dynamodb::Client,
    enterprise: &Enterprise,
) -> Result<Enterprise> {
    if enterprise.legacy_account_sync_at.is_some() {
        return Ok(enterprise.clone());
    }

    let now = crate::common::utils::time_utils::get_now();
    let updated = Enterprise::updater(enterprise.pk.clone(), enterprise.sk.clone())
        .with_legacy_account_sync_at(now)
        .with_updated_at(now)
        .execute(cli)
        .await?;

    Ok(updated)
}

#[cfg(feature = "server")]
async fn backfill_credentials(
    cli: &aws_sdk_dynamodb::Client,
    account: &Account,
    enterprise: &Enterprise,
) -> Result<()> {
    let mut bookmark: Option<String> = None;

    loop {
        let mut query = CredentialQueryOption::builder().limit(200);
        if let Some(ref current_bookmark) = bookmark {
            query = query.bookmark(current_bookmark.clone());
        }

        let (credentials, next_bookmark) =
            Credential::find_by_account_id(cli, &account.pk, query).await?;

        for credential in credentials
            .into_iter()
            .filter(|credential| matches!(credential.organization_id, Partition::None))
        {
            Credential::updater(credential.pk.clone(), credential.sk.clone())
                .with_organization_id(enterprise.pk.clone())
                .with_updated_at(crate::common::utils::time_utils::get_now())
                .execute(cli)
                .await?;
        }

        if next_bookmark.is_none() {
            break;
        }
        bookmark = next_bookmark;
    }

    Ok(())
}
