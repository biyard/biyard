use crate::common::ListResponse;
use crate::features::catalog::StoSummary;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, Result};
#[cfg(feature = "server")]
use crate::features::catalog::models::Sto;

/// GET /api/stos — 모든 STO 메타 (현재는 scan 기반, GSI 도입은 후속).
#[server(endpoint = "list_stos")]
pub async fn list_stos() -> std::result::Result<ListResponse<StoSummary>, ServerFnError> {
    let result: Result<ListResponse<StoSummary>> = async {
        let cfg = CommonConfig::default();
        let cli = cfg.dynamodb();

        let mut items: Vec<StoSummary> = Vec::new();
        let mut last_key: Option<
            std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
        > = None;

        loop {
            let mut req = cli
                .scan()
                .table_name(&cfg.table)
                .filter_expression("sk = :sk")
                .expression_attribute_values(
                    ":sk",
                    aws_sdk_dynamodb::types::AttributeValue::S(EntityType::Sto.to_string()),
                );
            if let Some(start) = last_key.take() {
                req = req.set_exclusive_start_key(Some(start));
            }
            let out = req.send().await?;
            for av in out.items.unwrap_or_default() {
                if let Ok(sto) = serde_dynamo::from_item::<_, Sto>(av) {
                    items.push(sto.into());
                }
            }
            last_key = out.last_evaluated_key;
            if last_key.is_none() {
                break;
            }
        }

        items.sort_by(|a, b| b.issued_at.cmp(&a.issued_at));
        Ok((items, None).into())
    }
    .await;

    result.map_err(|e: crate::common::Error| ServerFnError::new(e.to_string()))
}
