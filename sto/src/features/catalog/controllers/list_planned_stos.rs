use crate::features::catalog::PlannedStoListResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, Partition, Result};
#[cfg(feature = "server")]
use crate::features::catalog::models::{PlannedSto, PlannedStoQueryOption};
#[cfg(feature = "server")]
use crate::features::catalog::PlannedStoSummary;

#[get("/v1/planned-stos")]
pub async fn list_planned_stos_handler() -> Result<PlannedStoListResponse> {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    let pk = Partition::Planned.to_string();
    let opt = PlannedStoQueryOption::builder().limit(100);
    let (rows, _bookmark) = PlannedSto::query(cli, pk, opt).await?;

    let mut items: Vec<PlannedStoSummary> = rows
        .into_iter()
        .map(|p| PlannedStoSummary {
            planned_id: p.planned_id,
            name: p.name,
            category: p.category,
            country: p.country,
            issuer_id: p.issuer_id,
            issuer_name: p.issuer_name,
            broker: p.broker,
            broker_role: p.broker_role,
            expected_amount: p.expected_amount,
            expected_window: p.expected_window,
            registered_at: p.registered_at,
        })
        .collect();
    items.sort_by(|a, b| b.registered_at.cmp(&a.registered_at));

    let total = items.len();
    Ok(PlannedStoListResponse { items, total })
}
