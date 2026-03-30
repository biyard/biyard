use crate::common::*;
use crate::features::contacts::Need;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
pub struct Contact {
    pub pk: Partition,
    pub sk: EntityType,
    pub created_at: i64,

    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub company_name: String,
    pub needs: Need,
    pub help: String,
}

impl Contact {
    pub fn new(
        last_name: String,
        first_name: String,
        email: String,
        company_name: String,
        needs: Need,
        help: String,
    ) -> Self {
        Self {
            pk: Partition::Contact(email.clone()),
            sk: EntityType::Contact,
            created_at: crate::common::utils::time_utils::get_now(),
            last_name,
            first_name,
            email,
            company_name,
            needs,
            help,
        }
    }
}
