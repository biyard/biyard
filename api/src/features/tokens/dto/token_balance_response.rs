use crate::*;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo, Clone)]
pub struct TokenBalanceResponse {
    #[schemars(description = "Project ID")]
    pub project_id: Partition,

    #[schemars(description = "Meta user ID")]
    pub meta_user_id: String,

    #[schemars(description = "Current balance")]
    pub balance: i64,

    #[schemars(description = "Creation timestamp")]
    pub created_at: i64,

    #[schemars(description = "Last update timestamp")]
    pub updated_at: i64,
}

impl From<crate::features::tokens::TokenBalance> for TokenBalanceResponse {
    fn from(balance: crate::features::tokens::TokenBalance) -> Self {
        Self {
            project_id: balance.pk,
            meta_user_id: balance.meta_user_id,
            balance: balance.balance,
            created_at: balance.created_at,
            updated_at: balance.updated_at,
        }
    }
}
