use crate::{features::accounts::AccountType, *};

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, JsonSchema, OperationIo, Default,
)]
pub struct Account {
    pub pk: Partition,
    pub sk: EntityType,

    pub name: String,

    pub created_at: i64,
    pub updated_at: i64,

    pub user_type: AccountType,
}
