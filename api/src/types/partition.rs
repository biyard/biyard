use crate::*;

use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(
    Debug,
    Clone,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    PartialEq,
    Eq,
    OperationIo,
)]
#[dynamo_enum(error = "crate::Error2")]
pub enum Partition {
    #[default]
    None,

    // Account
    Account(String),
}
