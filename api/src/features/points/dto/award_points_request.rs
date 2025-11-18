use crate::{features::points::TransactionType, *};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo, Validate)]
pub struct TransactPointsRequest {
    #[schemars(
        description = "Month for point tracking (YYYY-MM format). If not provided, current month is used"
    )]
    #[serde(default = "TransactPointsRequest::default_month")]
    pub month: String,

    #[schemars(description = "Description or memo for this award")]
    pub description: Option<String>,

    #[serde(flatten)]
    pub tx: Transaction,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(tag = "tx_type")]
pub enum Transaction {
    #[schemars(description = "Award monthly points to a meta user.")]
    Award {
        #[schemars(description = "Meta user id managed by each customer")]
        to: String,
        #[schemars(description = "Amount of points to award")]
        amount: i64,
    },
    #[schemars(description = "Deduct monthly points from a meta user")]
    Deduct {
        #[schemars(description = "Meta user id managed by each customer")]
        from: String,
        #[schemars(description = "Amount of points to deduct")]
        amount: i64,
    },
    #[schemars(description = "Transfer monthly points between meta users")]
    Transfer {
        #[schemars(description = "Meta user id of the sender")]
        from: String,
        #[schemars(description = "Meta user id of the recipient")]
        to: String,
        #[schemars(description = "Amount of points to transfer")]
        amount: i64,
    },
    #[schemars(description = "Exchange monthly rewards points to monthly project tokens")]
    Exchange {
        #[schemars(description = "Meta user id managed by each customer")]
        from: String,
        #[schemars(description = "Amount of points to exchange")]
        amount: i64,
    },
}

impl TransactPointsRequest {
    pub fn default_month() -> String {
        time_utils::timestamp_to_yyyy_mm()
    }
}
