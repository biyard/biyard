use crate::common::Result;
use crate::features::enterprises::EnterpriseResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EnterpriseContextAuth, OrganizationRole};
#[cfg(feature = "server")]
use crate::features::enterprises::{Enterprise, EnterpriseError};

/// Update general settings for the current enterprise (name, slug).
///
/// - **Role:** Owner only. Renaming the enterprise or changing its slug
///   is a billing/identity-adjacent action, so it's gated tighter than
///   brand-level edits.
/// - **Scope:** Always operates on the enterprise resolved from
///   `EnterpriseContextAuth`. The client does not pass an enterprise id.
#[patch("/v1/enterprises/current", auth: EnterpriseContextAuth)]
pub async fn update_enterprise_handler(
    name: Option<String>,
    slug: Option<String>,
) -> Result<EnterpriseResponse> {
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

    if let Some(slug) = slug {
        let normalized = normalize_slug(&slug);
        if normalized.is_empty() {
            return Err(EnterpriseError::InvalidEnterpriseSlug.into());
        }
        updater = updater.with_slug(normalized);
    }

    updater = updater.with_updated_at(crate::common::utils::time_utils::get_now());

    let updated = updater.execute(cli).await?;

    Ok(updated.into())
}

#[cfg(feature = "server")]
fn normalize_slug(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut last_was_dash = false;

    for ch in input.chars() {
        let lowered = ch.to_ascii_lowercase();
        if lowered.is_ascii_alphanumeric() {
            out.push(lowered);
            last_was_dash = false;
        } else if !last_was_dash {
            out.push('-');
            last_was_dash = true;
        }
    }

    out.trim_matches('-').to_string()
}
