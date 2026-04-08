use crate::{
    Error,
    common::OrganizationRole,
    features::accounts::{AccountResponse, AccountType, controllers::get_me_handler},
    features::enterprises::{
        CurrentEnterpriseResponse, controllers::get_current_enterprise_handler,
    },
};
use dioxus::fullstack::Loading;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub struct Context {
    pub account_context: Store<AccountContext>,
}

impl Context {
    pub fn init() -> Result<Self, Loading> {
        let account_ctx = use_loader(move || async move {
            Ok::<_, Error>(match get_me_handler().await {
                Ok(resp) => AccountContext {
                    account: Some(resp),
                    current_enterprise: get_current_enterprise_handler().await.ok(),
                },
                Err(_) => AccountContext::default(),
            })
        })?();

        let ctx = Self {
            account_context: use_store(move || account_ctx),
        };
        use_context_provider(move || ctx);

        Ok(ctx)
    }

    /// Re-fetch the current account and enterprise from the server and
    /// write the fresh values into the shared store. Any component that
    /// reads `use_account_context()` will see the updated fields on
    /// next render.
    ///
    /// Call this after mutations that change the canonical state —
    /// e.g. renaming the enterprise, updating profile info, switching
    /// themes that depend on the server, etc.
    ///
    /// This deliberately does not block: it spawns a task and returns
    /// immediately so the caller's UI stays responsive. The store is
    /// written only after the fetches succeed.
    pub fn refresh(self) {
        let mut ctx = self.account_context;
        spawn(async move {
            let account = get_me_handler().await.ok();
            let enterprise = get_current_enterprise_handler().await.ok();
            let mut w = ctx.write();
            w.account = account;
            w.current_enterprise = enterprise;
        });
    }
}

pub fn use_account_context() -> Store<AccountContext> {
    use_context::<Context>().account_context
}

/// Convenience hook that returns the full `Context` handle, which
/// exposes `refresh()` in addition to the underlying store.
pub fn use_app_context() -> Context {
    use_context::<Context>()
}

#[derive(Store, Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AccountContext {
    pub account: Option<AccountResponse>,
    pub current_enterprise: Option<CurrentEnterpriseResponse>,
}

impl AccountContext {
    pub fn is_logged_in(&self) -> bool {
        self.account.is_some()
    }

    pub fn user_id(&self) -> Option<String> {
        self.account.as_ref().map(|u| u.id())
    }

    pub fn is_system_admin(&self) -> bool {
        self.account
            .as_ref()
            .map(|u| u.user_type == AccountType::SystemAdmin)
            .unwrap_or(false)
    }

    pub fn enterprise_name(&self) -> Option<String> {
        self.current_enterprise
            .as_ref()
            .map(|enterprise| enterprise.enterprise.name.clone())
    }

    /// Role of the current account within the current enterprise. Falls
    /// back to `Viewer` (least-privileged) if the enterprise has not yet
    /// loaded, so write-action UI stays hidden until we know otherwise.
    pub fn role(&self) -> OrganizationRole {
        self.current_enterprise
            .as_ref()
            .map(|e| e.role)
            .unwrap_or(OrganizationRole::Viewer)
    }

    /// True when the current account has at least Admin privileges in
    /// the current enterprise. Use this to gate any write-action UI
    /// (create / invite / edit / delete buttons).
    pub fn can_write(&self) -> bool {
        self.role().allows(OrganizationRole::Admin)
    }
}
