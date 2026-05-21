use crate::common::ListResponse;
use crate::features::catalog::StoSummary;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, Result};
#[cfg(feature = "server")]
use crate::features::catalog::models::{Sto, StoQueryOption};

#[get("/v1/stos")]
pub async fn list_stos_handler() -> Result<ListResponse<StoSummary>> {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    let opt = StoQueryOption::builder().limit(1000);
    let (stos, bookmark) = Sto::find_all(cli, EntityType::Sto, opt).await?;

    let mut items: Vec<StoSummary> = stos.into_iter().map(|s| s.into()).collect();
    items.sort_by(|a, b| b.issued_at.cmp(&a.issued_at));

    Ok((items, bookmark).into())
}
