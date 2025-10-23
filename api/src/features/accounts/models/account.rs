use crate::{features::accounts::AccountType, *};

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, JsonSchema, OperationIo, Default,
)]
pub struct Account {
    #[schemars(description = "ID of the created account")]
    pub pk: Partition,
    #[schemars(description = "Entity type of the created account")]
    #[dynamo(index = "gsi2", sk, name = "find_by_email")]
    pub sk: EntityType,

    #[schemars(description = "Name of the created account")]
    pub name: String,

    #[dynamo(index = "gsi1", pk, prefix = "AC", name = "find_by_email_and_password")]
    #[dynamo(index = "gsi2", pk, prefix = "AC", name = "find_by_email")]
    #[schemars(description = "Email of the created account")]
    pub email: String,
    #[dynamo(index = "gsi1", sk, name = "find_by_email_and_password")]
    #[schemars(description = "Server-side Hashed password of the created account")]
    pub password: String,

    #[schemars(description = "Creation timestamp of the account")]
    pub created_at: i64,
    #[schemars(description = "Last update timestamp of the account")]
    pub updated_at: i64,

    #[schemars(description = "Type of the user account")]
    pub user_type: AccountType,
}

impl Account {
    pub fn new(name: String, email: String, password: String) -> Self {
        let now = time_utils::get_now();
        let uuid = uuid::Uuid::new_v4().to_string();

        Self {
            pk: Partition::Account(uuid),
            sk: EntityType::Account,
            name,
            email,
            password,
            created_at: now,
            updated_at: now,
            user_type: AccountType::User,
        }
    }
}
