use crate::*;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "lowercase")]
pub enum ExchangeType {
    PointToToken,
    TokenToPoint,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo, Validate)]
pub struct ExchangeRequest {
    #[schemars(description = "Meta user ID to exchange for")]
    pub meta_user_id: String,

    #[schemars(description = "Type of exchange")]
    pub exchange_type: ExchangeType,

    #[validate(range(min = 1))]
    #[schemars(description = "Amount to exchange")]
    pub amount: i64,

    #[schemars(description = "Month for point tracking (YYYY-MM format)")]
    pub month: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct ExchangeResponse {
    #[schemars(description = "Amount exchanged")]
    pub amount_exchanged: i64,

    #[schemars(description = "Amount received")]
    pub amount_received: i64,

    #[schemars(description = "Exchange rate used")]
    pub exchange_rate: f64,

    #[schemars(description = "Remaining balance after exchange")]
    pub remaining_balance: i64,
}
