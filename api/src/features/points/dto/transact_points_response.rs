use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct TransactPointsResponse {
    #[schemars(description = "Unique transaction ID")]
    pub transaction_id: String,

    #[schemars(description = "Month in YYYY-MM format")]
    pub month: String,

    #[schemars(description = "Meta user ID")]
    pub meta_user_id: String,

    #[schemars(description = "Transaction type")]
    pub transaction_type: String,

    #[schemars(description = "Amount of points")]
    pub amount: i64,
}
