use crate::*;
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
)]
pub enum CredentialStatus {
    #[default]
    Active,
    Revoked,
}
