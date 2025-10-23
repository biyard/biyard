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
pub enum Partition {
    #[default]
    None,

    // Account
    Account(String),

    // Session
    Session(String),
}
