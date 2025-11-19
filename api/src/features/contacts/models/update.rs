use crate::*;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct Update {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub email: String,
}

impl Update {
    pub fn new(email: String) -> Self {
        let created_at = time_utils::get_now();

        Self {
            pk: Partition::Update(email.clone()),
            sk: EntityType::Update,
            created_at,
            email,
        }
    }
}
