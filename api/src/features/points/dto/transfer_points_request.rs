use crate::*;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo, Validate)]
pub struct TransferPointsRequest {
    #[schemars(description = "Source meta user ID")]
    pub from_user_id: String,

    #[schemars(description = "Target meta user ID")]
    pub to_user_id: String,

    #[validate(range(min = 1))]
    #[schemars(description = "Amount of points to transfer")]
    pub amount: i64,

    #[schemars(description = "Month for point tracking (YYYY-MM format). If not provided, current month is used")]
    pub month: Option<String>,

    #[schemars(description = "Description or memo for this transfer")]
    pub description: Option<String>,
}
