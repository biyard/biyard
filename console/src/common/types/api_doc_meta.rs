pub struct ApiEndpointMeta {
    pub method: &'static str,
    pub path: &'static str,
    pub group: &'static str,
    pub summary: &'static str,
    pub description: &'static str,
    pub auth: &'static str,
    pub path_params: &'static [(&'static str, &'static str)],
    pub query_params: &'static [(&'static str, &'static str, bool)],
    pub body_params: &'static [(&'static str, &'static str)],
    pub response_type: &'static str,
}

inventory::collect!(ApiEndpointMeta);

#[cfg(feature = "server")]
pub fn all_api_docs() -> Vec<&'static ApiEndpointMeta> {
    inventory::iter::<ApiEndpointMeta>.into_iter().collect()
}

#[cfg(feature = "server")]
pub fn schema_for_type(type_name: &str) -> Option<serde_json::Value> {
    use schemars::schema_for;

    let schema = match type_name {
        "TransactPointsRequest" | "Vec<TransactPointsRequest>" => {
            serde_json::to_value(schema_for!(crate::features::points::TransactPointsRequest)).ok()
        }
        "TransactPointsResponse" | "Vec<TransactPointsResponse>" => {
            serde_json::to_value(schema_for!(crate::features::points::TransactPointsResponse)).ok()
        }
        "MonthlyPointAggregationResponse" => {
            serde_json::to_value(schema_for!(crate::features::points::MonthlyPointAggregationResponse)).ok()
        }
        "PointBalanceResponse" => {
            serde_json::to_value(schema_for!(crate::features::points::PointBalanceResponse)).ok()
        }
        "PointTransactionResponse" => {
            serde_json::to_value(schema_for!(crate::features::points::PointTransactionResponse)).ok()
        }
        "MonthlySummariesResponse" => {
            serde_json::to_value(schema_for!(crate::features::points::MonthlySummariesResponse)).ok()
        }
        "Option<TokenResponse>" | "TokenResponse" => {
            serde_json::to_value(schema_for!(crate::features::tokens::TokenResponse)).ok()
        }
        "TokenBalanceResponse" => {
            serde_json::to_value(schema_for!(crate::features::tokens::TokenBalanceResponse)).ok()
        }
        "ProjectResponse" => {
            serde_json::to_value(schema_for!(crate::features::projects::ProjectResponse)).ok()
        }
        "TreasuryStatusResponse" => {
            serde_json::to_value(schema_for!(crate::features::projects::TreasuryStatusResponse)).ok()
        }
        _ => None,
    };

    schema
}
