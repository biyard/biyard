use crate::{features::points::TransactionType, *};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo, Validate)]
pub struct TransactPointsRequest {
    #[schemars(description = "Type of transaction to perform")]
    pub tx_type: TransactionType,

    #[validate(range(min = 1))]
    #[schemars(description = "Amount of points to award")]
    pub amount: i64,

    #[schemars(
        description = "Month for point tracking (YYYY-MM format). If not provided, current month is used"
    )]
    pub month: Option<String>,

    #[schemars(description = "Description or memo for this award")]
    pub description: Option<String>,
}
