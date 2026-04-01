use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct PurchaseRequest {
    #[schemars(description = "Meta user ID managed by the customer")]
    pub meta_user_id: String,

    #[schemars(description = "Purchase amount in the project's currency")]
    pub amount: i64,

    #[schemars(description = "Name of the purchased item")]
    pub item_name: String,

    #[schemars(
        description = "Reward rate as a percentage (e.g. 5.0 means 5% of purchase amount is awarded as points)"
    )]
    pub reward_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct PurchaseResponse {
    #[schemars(description = "Original purchase amount")]
    pub purchase_amount: i64,

    #[schemars(description = "Points awarded to the user (amount * reward_rate / 100)")]
    pub reward_points: i64,

    #[schemars(description = "Amount contributed to the project treasury")]
    pub treasury_contribution: i64,
}
