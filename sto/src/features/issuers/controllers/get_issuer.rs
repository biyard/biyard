use crate::features::issuers::IssuerDetailResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, Partition, Result};
#[cfg(feature = "server")]
use crate::features::catalog::StoSummary;
#[cfg(feature = "server")]
use crate::features::catalog::models::{Sto, StoQueryOption};
#[cfg(feature = "server")]
use crate::features::issuers::{Issuer, IssuerSummary};

#[get("/v1/issuers/:issuer_id")]
pub async fn get_issuer_handler(issuer_id: String) -> Result<IssuerDetailResponse> {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    let pk = Partition::Issuer(issuer_id.clone()).to_string();
    let issuer = Issuer::get(cli, pk, Some(EntityType::Issuer.to_string()))
        .await?
        .ok_or_else(|| crate::common::Error::NotFound(format!("Issuer: {issuer_id}")))?;

    let summary = IssuerSummary {
        issuer_id: issuer.issuer_id.clone(),
        name: issuer.name,
        country: issuer.country,
        category: issuer.category,
        description: issuer.description,
        status: issuer.status,
        status_note: issuer.status_note,
        sandbox: issuer.sandbox,
        chain: issuer.chain,
        website: issuer.website,
    };

    let opt = StoQueryOption::builder().limit(500);
    let (stos, _) = Sto::find_by_issuer_id(cli, &issuer_id, opt).await?;
    let mut stos: Vec<StoSummary> = stos.into_iter().map(|s| s.into()).collect();
    stos.sort_by(|a, b| b.issued_at.cmp(&a.issued_at));

    Ok(IssuerDetailResponse {
        issuer: summary,
        stos,
    })
}
