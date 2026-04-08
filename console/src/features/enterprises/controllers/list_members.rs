use crate::common::Result;
use crate::features::enterprises::MemberResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EnterpriseContextAuth, OrganizationRole, Partition};
#[cfg(feature = "server")]
use crate::features::accounts::{Account, AccountQueryOption};

#[get("/v1/enterprises/members", auth: EnterpriseContextAuth)]
pub async fn list_members_handler() -> Result<Vec<MemberResponse>> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    if !auth.role.allows(OrganizationRole::Viewer) {
        return Err(crate::common::Error::Forbidden);
    }

    // Single-membership: an Account row carries its enterprise_id
    // directly. We query gsi3 (find_by_enterprise_id) to enumerate
    // every member of the current enterprise without scanning the
    // whole accounts table.
    let (accounts, _) = Account::find_by_enterprise_id(
        cli,
        &auth.enterprise.pk,
        AccountQueryOption::builder().limit(200),
    )
    .await?;

    let responses: Vec<MemberResponse> = accounts
        .into_iter()
        .map(|account| MemberResponse {
            account_id: match &account.pk {
                Partition::Account(id) => id.clone(),
                _ => String::new(),
            },
            name: account.name,
            email: account.email,
            role: account.organization_role,
            joined_at: account.created_at,
        })
        .collect();

    Ok(responses)
}
