use crate::common::ListResponse;
use crate::features::catalog::StoSummary;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, Result};
#[cfg(feature = "server")]
use crate::features::catalog::models::Sto;

/// GET /api/stos
///   - GSI1 (STATUS) 으로 모든 상태 카테고리를 차례로 쿼리해 합산.
///   - 1차 구현: 페이지네이션 없이 전부.
#[server(endpoint = "list_stos")]
pub async fn list_stos() -> std::result::Result<ListResponse<StoSummary>, ServerFnError> {
    let result: Result<ListResponse<StoSummary>> = async {
        let cfg = CommonConfig::default();
        let cli = cfg.dynamodb();

        let mut items: Vec<StoSummary> = Vec::new();
        let mut last_key: Option<std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>> = None;

        loop {
            let mut req = cli
                .scan()
                .table_name(&cfg.table)
                .filter_expression("sk = :sk")
                .expression_attribute_values(
                    ":sk",
                    aws_sdk_dynamodb::types::AttributeValue::S(
                        EntityType::Sto.to_string(),
                    ),
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
