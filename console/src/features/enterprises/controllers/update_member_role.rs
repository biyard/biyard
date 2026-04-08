use crate::common::OrganizationRole;
use crate::common::Result;
use crate::features::enterprises::MemberResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EnterpriseContextAuth, EntityType, Partition};
#[cfg(feature = "server")]
use crate::features::accounts::{Account, AccountQueryOption};
#[cfg(feature = "server")]
use crate::features::enterprises::EnterpriseError;

#[patch("/v1/enterprises/members/:account_id", auth: EnterpriseContextAuth)]
pub async fn update_member_role_handler(
    account_id: String,
    role: OrganizationRole,
) -> Result<MemberResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    // Only Owner can change roles \u2014 demoting/promoting an Admin is
    // potentially destructive (e.g. demoting the last owner), so the
    // authority comes from the Owner tier only.
    if !auth.role.allows(OrganizationRole::Owner) {
        return Err(crate::common::Error::Forbidden);
    }

    let target_pk = Partition::Account(account_id);
    let target = Account::get(cli, &target_pk, Some(EntityType::Account))
        .await?
        .ok_or(EnterpriseError::MemberNotFound)?;

    // Target must be a member of *this* enterprise.
    if target.enterprise_id != auth.enterprise.pk {
        return Err(EnterpriseError::MemberNotFound.into());
    }

    // Last-owner protection: if the target is currently the only Owner
    // and the requested role is anything other than Owner, refuse.
    if matches!(target.organization_role, OrganizationRole::Owner)
        && !matches!(role, OrganizationRole::Owner)
    {
        let owner_count = count_owners(cli, &auth.enterprise.pk).await?;
        if owner_count <= 1 {
            return Err(EnterpriseError::LastOwnerCannotDemote.into());
        }
    }

    let now = crate::common::utils::time_utils::get_now();
    let updated = Account::updater(target.pk.clone(), target.sk.clone())
        .with_organization_role(role)
        .with_updated_at(now)
        .execute(cli)
        .await?;

    Ok(MemberResponse {
        account_id: match &updated.pk {
            Partition::Account(id) => id.clone(),
            _ => String::new(),
        },
        name: updated.name,
        email: updated.email,
        role: updated.organization_role,
        joined_at: updated.created_at,
    })
}

#[cfg(feature = "server")]
pub(crate) async fn count_owners(
    cli: &aws_sdk_dynamodb::Client,
    enterprise_pk: &Partition,
) -> Result<usize> {
    let (accounts, _) = Account::find_by_enterprise_id(
        cli,
        enterprise_pk,
        AccountQueryOption::builder().limit(200),
    )
    .await?;

    Ok(accounts
        .into_iter()
        .filter(|a| matches!(a.organization_role, OrganizationRole::Owner))
        .count())
}
