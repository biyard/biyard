use crate::common::*;
use crate::features::accounts::AccountType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct Account {
    pub pk: Partition,
    #[dynamo(index = "gsi2", sk, name = "find_by_email")]
    pub sk: EntityType,

    pub name: String,

    #[dynamo(index = "gsi1", pk, prefix = "AC", name = "find_by_email_and_password")]
    #[dynamo(index = "gsi2", pk, prefix = "AC", name = "find_by_email")]
    pub email: String,
    #[dynamo(index = "gsi1", sk, name = "find_by_email_and_password")]
    pub password: String,

    pub created_at: i64,
    pub updated_at: i64,

    pub user_type: AccountType,
}

impl Account {
    pub fn new(name: String, email: String, password: String) -> Self {
        let now = crate::common::utils::time_utils::get_now();
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

impl From<Account> for crate::features::accounts::AccountResponse {
    fn from(account: Account) -> Self {
        Self {
            pk: account.pk,
            name: account.name,
            email: account.email,
            created_at: account.created_at,
        }
    }
}
