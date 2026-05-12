//! GET /api/stos — DynamoDB 에서 모든 STO 항목 조회.
//! 1차 검증용 — scan + filter (origin in {DART, MUSICOW, PRESS}, sk=STO).
//! 후속에 GSI 기반 쿼리·필터·페이지네이션으로 교체.

use dioxus::prelude::*;

use crate::features::catalog::StoListResponse;
#[cfg(feature = "server")]
use crate::features::catalog::StoSummary;

#[server(endpoint = "list_stos")]
pub async fn list_stos() -> Result<StoListResponse, ServerFnError> {
    use crate::common::CommonConfig;
    use crate::features::catalog::models::Sto;

    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb().await;

    let mut items: Vec<StoSummary> = Vec::new();
    let mut last_key = None;

    loop {
        let mut req = cli
            .scan()
            .table_name(&cfg.table)
            .filter_expression("sk = :sk")
            .expression_attribute_values(
                ":sk",
                aws_sdk_dynamodb::types::AttributeValue::S("STO".to_string()),
            );
        if let Some(start_key) = last_key.take() {
            req = req.set_exclusive_start_key(Some(start_key));
        }
        let out = req
            .send()
            .await
            .map_err(|e| ServerFnError::new(format!("dynamo scan: {e}")))?;

        for av in out.items.unwrap_or_default() {
            let sto: Sto = match serde_dynamo::from_item(av) {
                Ok(s) => s,
                Err(e) => {
                    tracing::warn!(error = %e, "skip malformed sto");
                    continue;
                }
            };
            items.push(sto.into());
        }

        last_key = out.last_evaluated_key;
        if last_key.is_none() {
            break;
        }
    }

    let total = items.len();
    items.sort_by(|a, b| b.issued_at.cmp(&a.issued_at));
    Ok(StoListResponse { items, total })
}
