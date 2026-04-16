use crate::common::*;

#[derive(
    Debug, Clone, PartialEq, Eq, Default, SerializeDisplay, DeserializeFromStr, DynamoEnum,
)]
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum TransactionType {
    #[default]
    Award,
    Deduct,
    Transfer,
    Exchange,
}
