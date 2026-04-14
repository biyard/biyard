use dioxus::fullstack::axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use tower_sessions::Session;

use crate::common::{CommonConfig, EntityType, Error, OrganizationRole, Partition};
use crate::features::accounts::{Account, AccountType};
use crate::features::enterprises::{Enterprise, EnterpriseError};

use super::resolve_account_from_parts;

#[derive(Debug, Clone)]
pub struct SystemAdminAuth {
    pub account: Account,
}

impl<S> FromRequestParts<S> for SystemAdminAuth
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = CommonConfig::default();
        let cli = config.dynamodb();

        let account = resolve_account_from_parts(parts, state, cli).await?;
        if account.user_type != AccountType::SystemAdmin {
            return Err(Error::Forbidden);
        }

        Ok(Self { account })
    }
}

#[derive(Debug, Clone)]
pub struct EnterpriseContextAuth {
    pub account: Account,
    pub enterprise: Enterprise,
    pub role: OrganizationRole,
}

impl<S> FromRequestParts<S> for EnterpriseContextAuth
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = CommonConfig::default();
        let cli = config.dynamodb();

        let account = resolve_account_from_parts(parts, state, cli).await?;

        if matches!(account.enterprise_id, Partition::None) {
            return Err(EnterpriseError::EnterpriseNotFound.into());
        }

        let enterprise =
            Enterprise::get(cli, &account.enterprise_id, Some(EntityType::Enterprise))
                .await?
                .ok_or(EnterpriseError::EnterpriseNotFound)?;
        let role = account.organization_role;

        Ok(Self {
            account,
            enterprise,
            role,
        })
    }
}
