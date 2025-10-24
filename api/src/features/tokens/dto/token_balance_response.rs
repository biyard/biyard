use crate::*;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct TokenBalanceResponse {
    #[schemars(description = "Token ID")]
    pub token_id: Partition,

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
            token_id: balance.token_id,
            project_id: balance.project_id,
            meta_user_id: balance.meta_user_id,
            balance: balance.balance,
            created_at: balance.created_at,
            updated_at: balance.updated_at,
        }
    }
}
