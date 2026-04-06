use crate::common::{CommonConfig, Result, Serialize};
use crate::features::landing::{Contact, Need};
use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitContactResponse {
    pub id: String,
}

#[post("/v1/landing/contacts")]
pub async fn submit_contact_handler(
    first_name: String,
    last_name: String,
    email: String,
    company_name: String,
    needs: Need,
    help: String,
) -> Result<SubmitContactResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let c = Contact::new(last_name, first_name, email, company_name, needs, help);
    c.create(cli).await?;

    Ok(SubmitContactResponse {
        id: c.pk.to_string(),
    })
}
