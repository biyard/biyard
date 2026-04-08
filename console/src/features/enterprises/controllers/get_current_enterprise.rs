use crate::common::Result;
use crate::features::enterprises::CurrentEnterpriseResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::EnterpriseContextAuth;

#[get("/v1/enterprises/current", auth: EnterpriseContextAuth)]
pub async fn get_current_enterprise_handler() -> Result<CurrentEnterpriseResponse> {
    Ok(CurrentEnterpriseResponse {
        enterprise: auth.enterprise.into(),
        role: auth.role,
    })
}
