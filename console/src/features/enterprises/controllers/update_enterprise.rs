use crate::common::Result;
use crate::features::enterprises::EnterpriseResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EnterpriseContextAuth, OrganizationRole};
#[cfg(feature = "server")]
use crate::features::enterprises::{Enterprise, EnterpriseError};

/// Update general settings for the current enterprise (name only).
///
/// - **Role:** Owner only. Renaming the enterprise is a billing/identity-
///   adjacent action, so it's gated tighter than brand-level edits.
/// - **Scope:** Always operates on the enterprise resolved from
///   `EnterpriseContextAuth`. The client does not pass an enterprise id.
#[patch("/v1/enterprises/current", auth: EnterpriseContextAuth)]
pub async fn update_enterprise_handler(name: Option<String>) -> Result<EnterpriseResponse> {
    if !auth.role.allows(OrganizationRole::Owner) {
        return Err(crate::common::Error::Forbidden);
    }

    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let mut updater = Enterprise::updater(auth.enterprise.pk.clone(), auth.enterprise.sk.clone());

    if let Some(name) = name {
        let trimmed = name.trim().to_string();
        if trimmed.is_empty() {
            return Err(EnterpriseError::InvalidEnterpriseName.into());
        }
        updater = updater.with_name(trimmed);
    }

    updater = updater.with_updated_at(crate::common::utils::time_utils::get_now());

    let updated = updater.execute(cli).await?;

    Ok(updated.into())
}
