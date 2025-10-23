use crate::{features::accounts::Account, *};

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema, Default)]
pub struct AccountResponse {
    #[schemars(description = "ID of the account")]
    pub pk: Partition,

    #[schemars(description = "Name of the account")]
    pub name: String,
    #[schemars(description = "Email of the account")]
    pub email: String,
    #[schemars(description = "Creation timestamp of the account")]
    pub created_at: i64,
}

impl From<Account> for AccountResponse {
    fn from(account: Account) -> Self {
        Self {
            pk: account.pk,
            name: account.name,
            email: account.email,
            created_at: account.created_at,
        }
    }
}
