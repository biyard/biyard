use crate::common::*;

#[derive(
    Debug, Clone, Copy, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum, PartialEq, Eq,
)]
pub enum ProjectStatus {
    #[default]
    Active,
    Inactive,
}
