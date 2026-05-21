use crate::common::ListResponse;
use crate::features::issuers::IssuerSummary;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, Result};
#[cfg(feature = "server")]
use crate::features::issuers::{Issuer, IssuerQueryOption};

#[get("/v1/issuers")]
pub async fn list_issuers_handler() -> Result<ListResponse<IssuerSummary>> {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    let opt = IssuerQueryOption::builder().limit(100);
    let (issuers, bookmark) = Issuer::find_all(cli, EntityType::Issuer, opt).await?;

    let mut items: Vec<IssuerSummary> = issuers
        .into_iter()
        .map(|i| IssuerSummary {
            issuer_id: i.issuer_id,
            name: i.name,
            country: i.country,
            category: i.category,
            description: i.description,
            status: i.status,
            status_note: i.status_note,
            sandbox: i.sandbox,
            chain: i.chain,
            website: i.website,
        })
        .collect();
    items.sort_by(|a, b| a.name.cmp(&b.name));

    Ok((items, bookmark).into())
}
