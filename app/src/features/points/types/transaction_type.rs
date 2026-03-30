use crate::common::*;

#[derive(Debug, Clone, PartialEq, Eq, Default, SerializeDisplay, DeserializeFromStr, DynamoEnum)]
pub enum TransactionType {
    #[default]
    Award,
    Deduct,
    Transfer,
    Exchange,
}
