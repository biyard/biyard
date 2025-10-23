use crate::{features::accounts::Account, *};

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema, Default)]
pub struct SignupAccountResponse {
    #[schemars(description = "ID of the created account")]
    pub pk: Partition,
    #[schemars(description = "Entity type of the created account")]
    pub sk: EntityType,

    #[schemars(description = "Name of the created account")]
    pub name: String,
    #[schemars(description = "Email of the created account")]
    pub email: String,
    #[schemars(description = "Creation timestamp of the account")]
    pub created_at: i64,
}

impl From<Account> for SignupAccountResponse {
    fn from(account: Account) -> Self {
        Self {
            pk: account.pk,
            sk: account.sk,
            name: account.name,
            email: account.email,
            created_at: account.created_at,
        }
    }
}
