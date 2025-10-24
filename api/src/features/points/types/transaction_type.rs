use crate::*;

use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
    OperationIo,
    DynamoEnum,
)]
pub enum TransactionType {
    #[default]
    #[schemars(description = "Award points to a meta user")]
    Award,
    #[schemars(description = "Deduct points from a meta user")]
    Deduct,
    #[schemars(description = "Transfer points between meta users")]
    Transfer,
    #[schemars(description = "Exchange points for rewards or services")]
    Exchange,
}
