use crate::{
    Error,
    features::accounts::{AccountResponse, AccountType, controllers::get_me_handler},
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
}

pub fn use_account_context() -> Store<AccountContext> {
    use_context::<Context>().account_context
}

#[derive(Store, Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AccountContext {
    pub account: Option<AccountResponse>,
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
}
