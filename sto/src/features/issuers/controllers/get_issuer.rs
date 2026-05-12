use crate::features::issuers::IssuerDetailResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::features::issuers::IssuerSummary;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, Partition, Result};
#[cfg(feature = "server")]
use crate::features::catalog::StoSummary;
#[cfg(feature = "server")]
use crate::features::catalog::models::Sto;
#[cfg(feature = "server")]
use crate::features::issuers::Issuer;

/// GET /api/issuers/:id — 발행사 메타 + 해당 발행사의 STO 목록
#[server(endpoint = "get_issuer")]
pub async fn get_issuer(
    issuer_id: String,
) -> std::result::Result<IssuerDetailResponse, ServerFnError> {
    let result: Result<IssuerDetailResponse> = async {
        let cfg = CommonConfig::default();
        let cli = cfg.dynamodb();

        // 1) Issuer META
        let pk = Partition::Issuer(issuer_id.clone()).to_string();
        let out = cli
            .get_item()
            .table_name(&cfg.table)
            .key("pk", aws_sdk_dynamodb::types::AttributeValue::S(pk))
            .key(
                "sk",
                aws_sdk_dynamodb::types::AttributeValue::S(EntityType::Issuer.to_string()),
            )
            .send()
            .await?;

        let av = out
            .item
            .ok_or_else(|| crate::common::Error::NotFound(format!("Issuer: {issuer_id}")))?;
        let issuer: Issuer = serde_dynamo::from_item(av)?;
        let summary = IssuerSummary {
            issuer_id: issuer.issuer_id.clone(),
            name: issuer.name,
            region: issuer.region,
            country: issuer.country,
            category: issuer.category,
            description: issuer.description,
            status: issuer.status,
            sandbox: issuer.sandbox,
            chain: issuer.chain,
            website: issuer.website,
        };

        // 2) GSI3 로 발행사 → STO 역참조
        let out2 = cli
            .query()
            .table_name(&cfg.table)
            .index_name("gsi3")
            .key_condition_expression("gsi3_pk = :p")
            .expression_attribute_values(
                ":p",
                aws_sdk_dynamodb::types::AttributeValue::S(format!("ISSUER#{issuer_id}")),
            )
            .send()
            .await?;

        let mut stos: Vec<StoSummary> = Vec::new();
        for av in out2.items.unwrap_or_default() {
            if let Ok(sto) = serde_dynamo::from_item::<_, Sto>(av) {
                stos.push(sto.into());
            }
        }
        stos.sort_by(|a, b| b.issued_at.cmp(&a.issued_at));

        Ok(IssuerDetailResponse {
            issuer: summary,
            stos,
        })
    }
    .await;

    result.map_err(|e: crate::common::Error| ServerFnError::new(e.to_string()))
}
