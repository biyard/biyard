use crate::features::catalog::StoDetailResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, Partition, Result};
#[cfg(feature = "server")]
use crate::features::catalog::FilingSummary;
#[cfg(feature = "server")]
use crate::features::catalog::models::{Sto, StoMetaBundle, StoPartitionRow};

#[get("/v1/stos/:sto_id")]
pub async fn get_sto_handler(sto_id: String) -> Result<StoDetailResponse> {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();
    let pk = Partition::Sto(sto_id.clone()).to_string();

    let rows = StoPartitionRow::query(cli, pk).await?;

    let mut sto: Option<Sto> = None;
    let mut bundle = StoMetaBundle::default();
    let mut filings: Vec<FilingSummary> = Vec::new();

    for row in rows {
        match row {
            StoPartitionRow::Sto(s) => sto = Some(s),
            StoPartitionRow::MetaMusic(m) => bundle.music = Some(m),
            StoPartitionRow::MetaArt(m) => bundle.art = Some(m),
            StoPartitionRow::MetaRealEstate(m) => bundle.real_estate = Some(m),
            StoPartitionRow::MetaLivestock(m) => bundle.livestock = Some(m),
            StoPartitionRow::Filing(f) => filings.push(f.into()),
        }
    }

    let sto = sto.ok_or_else(|| crate::common::Error::NotFound(format!("STO: {sto_id}")))?;
    filings.sort_by(|a, b| b.filed_at.cmp(&a.filed_at));

    Ok(sto.into_detail(bundle, filings))
}
