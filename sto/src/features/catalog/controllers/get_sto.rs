use crate::features::catalog::StoDetailResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, Partition, Result};
#[cfg(feature = "server")]
use crate::features::catalog::FilingSummary;
#[cfg(feature = "server")]
use crate::features::catalog::models::{Sto, StoMetaRow};
#[cfg(feature = "server")]
use crate::features::filings::Filing;

/// GET /api/stos/:id — STO 메타 + 묶인 모든 공시까지 한 번에.
#[server(endpoint = "get_sto")]
pub async fn get_sto(sto_id: String) -> std::result::Result<StoDetailResponse, ServerFnError> {
    let result: Result<StoDetailResponse> = async {
        let cfg = CommonConfig::default();
        let cli = cfg.dynamodb();
        let pk = Partition::Sto(sto_id.clone()).to_string();

        let out = cli
            .query()
            .table_name(&cfg.table)
            .key_condition_expression("pk = :p")
            .expression_attribute_values(
                ":p",
                aws_sdk_dynamodb::types::AttributeValue::S(pk),
            )
            .send()
            .await?;

        let mut sto: Option<Sto> = None;
        let mut meta: Option<StoMetaRow> = None;
        let mut filings: Vec<FilingSummary> = Vec::new();

        for av in out.items.unwrap_or_default() {
            let sk = av.get("sk").and_then(|v| v.as_s().ok()).cloned().unwrap_or_default();
            if sk == EntityType::Sto.to_string() {
                sto = serde_dynamo::from_item(av).ok();
            } else if sk.starts_with("STO_META#") {
                meta = serde_dynamo::from_item(av).ok();
            } else if sk.starts_with("Filing#") || sk.starts_with("FILING#") {
                if let Ok(f) = serde_dynamo::from_item::<_, Filing>(av) {
                    filings.push(f.into());
                }
            }
        }

        let sto = sto.ok_or_else(|| crate::common::Error::NotFound(format!("STO: {sto_id}")))?;
        filings.sort_by(|a, b| b.filed_at.cmp(&a.filed_at));

        Ok(sto.into_detail(meta.map(|m| m.meta), filings))
    }
    .await;

    result.map_err(|e: crate::common::Error| ServerFnError::new(e.to_string()))
}
