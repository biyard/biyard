use crate::*;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo, Validate)]
pub struct DeductPointsRequest {
    #[schemars(description = "Meta user ID to deduct points from")]
    pub meta_user_id: String,

    #[validate(range(min = 1))]
    #[schemars(description = "Amount of points to deduct")]
    pub amount: i64,

    #[schemars(description = "Month for point tracking (YYYY-MM format). If not provided, current month is used")]
    pub month: Option<String>,

    #[schemars(description = "Description or memo for this deduction")]
    pub description: Option<String>,
}
