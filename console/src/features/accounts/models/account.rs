use crate::common::OrganizationRole;
use crate::common::*;
use crate::features::accounts::{AccountType, PasswordScheme};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct Account {
    pub pk: Partition,
    #[dynamo(index = "gsi2", sk, name = "find_by_email")]
    #[dynamo(index = "gsi3", sk, name = "find_by_enterprise_id")]
    pub sk: EntityType,

    pub name: String,

    #[dynamo(index = "gsi1", pk, prefix = "AC", name = "find_by_email_and_password")]
    #[dynamo(index = "gsi2", pk, prefix = "AC", name = "find_by_email")]
    pub email: String,
    #[dynamo(index = "gsi1", sk, name = "find_by_email_and_password")]
    pub password: String,
    #[serde(default)]
    pub password_scheme: PasswordScheme,

    #[serde(default)]
    #[dynamo(index = "gsi3", pk, name = "find_by_enterprise_id")]
    pub enterprise_id: Partition,

    #[serde(default)]
    pub organization_role: OrganizationRole,

    pub created_at: i64,
    pub updated_at: i64,

    #[serde(default)]
    pub user_type: AccountType,
}

impl Account {
    pub fn new(
        name: String,
        email: String,
        password: String,
        password_scheme: PasswordScheme,
    ) -> Self {
        let now = crate::common::utils::time_utils::get_now();
        let uuid = uuid::Uuid::now_v7().to_string();

        Self {
            pk: Partition::Account(uuid),
            sk: EntityType::Account,
            name,
            email,
            password,
            password_scheme,
            enterprise_id: Partition::None,
            organization_role: OrganizationRole::Owner,
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
            enterprise_id: account.enterprise_id,
            organization_role: account.organization_role,
            user_type: account.user_type,
            created_at: account.created_at,
        }
    }
}
