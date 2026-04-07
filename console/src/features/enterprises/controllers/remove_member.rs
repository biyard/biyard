use crate::common::Result;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EnterpriseContextAuth, EntityType, OrganizationRole, Partition};
#[cfg(feature = "server")]
use crate::features::accounts::Account;
#[cfg(feature = "server")]
use crate::features::enterprises::controllers::count_owners;
#[cfg(feature = "server")]
use crate::features::enterprises::EnterpriseError;

/// Remove a member from the current enterprise. The target Account is
/// not deleted \u2014 it becomes orphaned (`enterprise_id = None`,
/// `organization_role = Viewer`). The user can still log in and will
/// see a "no enterprise" empty state until they create or join one.
#[delete("/v1/enterprises/members/:account_id", auth: EnterpriseContextAuth)]
pub async fn remove_member_handler(account_id: String) -> Result<()> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    if !auth.role.allows(OrganizationRole::Admin) {
        return Err(crate::common::Error::Forbidden);
    }

    let target_pk = Partition::Account(account_id);
    let target = Account::get(cli, &target_pk, Some(EntityType::Account))
        .await?
        .ok_or(EnterpriseError::MemberNotFound)?;

    if target.enterprise_id != auth.enterprise.pk {
        return Err(EnterpriseError::MemberNotFound.into());
    }

    // Admins cannot remove Owners. Owners can remove anyone (including
    // themselves), subject to the last-owner check below.
    if matches!(target.organization_role, OrganizationRole::Owner)
        && !auth.role.allows(OrganizationRole::Owner)
    {
        return Err(crate::common::Error::Forbidden);
    }

    // Last-owner protection: removing the only Owner would leave the
    // enterprise unmanageable.
    if matches!(target.organization_role, OrganizationRole::Owner) {
        let owner_count = count_owners(cli, &auth.enterprise.pk).await?;
        if owner_count <= 1 {
            return Err(EnterpriseError::LastOwnerCannotLeave.into());
        }
    }

    let now = crate::common::utils::time_utils::get_now();
    Account::updater(target.pk.clone(), target.sk.clone())
        .with_enterprise_id(Partition::None)
        .with_organization_role(OrganizationRole::Viewer)
        .with_updated_at(now)
        .execute(cli)
        .await?;

    Ok(())
}
