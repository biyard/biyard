use crate::common::Result;
use crate::features::accounts::AccountResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::SystemAdminAuth;

#[get("/v1/admin/me", auth: SystemAdminAuth)]
pub async fn get_admin_me_handler() -> Result<AccountResponse> {
    Ok(auth.account.into())
}
