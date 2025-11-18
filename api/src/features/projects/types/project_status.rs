use crate::*;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    OperationIo,
    PartialEq,
    Eq,
)]
pub enum ProjectStatus {
    #[default]
    Active,
    Inactive,
}
