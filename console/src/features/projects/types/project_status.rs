use crate::common::*;

#[derive(
    Debug, Clone, Copy, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum, PartialEq, Eq,
)]
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum ProjectStatus {
    #[default]
    Active,
    Inactive,
}
