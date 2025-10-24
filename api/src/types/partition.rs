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

    // Credential
    Credential(String),

    // Project
    Project(String),

    // Point Feature
    #[schemars(description = "Customer's mapping key")]
    MetaUser(String),

    // Token
    Token(String),

    // Token Balance
    TokenBalance(String),
}
