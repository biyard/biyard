use crate::common::ListResponse;
use crate::features::issuers::IssuerSummary;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, Result};
#[cfg(feature = "server")]
use crate::features::issuers::Issuer;

/// GET /api/issuers — 모든 발행사
#[server(endpoint = "list_issuers")]
pub async fn list_issuers() -> std::result::Result<ListResponse<IssuerSummary>, ServerFnError> {
    let result: Result<ListResponse<IssuerSummary>> = async {
        let cfg = CommonConfig::default();
        let cli = cfg.dynamodb();

        let out = cli
            .scan()
            .table_name(&cfg.table)
            .filter_expression("sk = :sk")
            .expression_attribute_values(
                ":sk",
                aws_sdk_dynamodb::types::AttributeValue::S(EntityType::Issuer.to_string()),
            )
            .send()
            .await?;

        let mut items: Vec<IssuerSummary> = Vec::new();
        for av in out.items.unwrap_or_default() {
            if let Ok(i) = serde_dynamo::from_item::<_, Issuer>(av) {
                items.push(IssuerSummary {
                    issuer_id: i.issuer_id,
                    name: i.name,
                    region: i.region,
                    country: i.country,
                    category: i.category,
                    description: i.description,
                    status: i.status,
                    sandbox: i.sandbox,
                    chain: i.chain,
                    website: i.website,
                });
            }
        }
        items.sort_by(|a, b| a.name.cmp(&b.name));
        Ok((items, None).into())
    }
    .await;

    result.map_err(|e: crate::common::Error| ServerFnError::new(e.to_string()))
}
