use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default)]
pub struct Update {
    pub pk: Partition,
    pub sk: EntityType,
    pub created_at: i64,
    pub email: String,
}

impl Update {
    pub fn new(email: String) -> Self {
        let created_at = crate::common::utils::time_utils::get_now();

        Self {
            pk: Partition::Update(email.clone()),
            sk: EntityType::Update,
            created_at,
            email,
        }
    }
}
