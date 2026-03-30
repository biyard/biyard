use crate::common::*;

#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum, PartialEq, Eq)]
pub enum ProjectStatus {
    #[default]
    Active,
    Inactive,
}
