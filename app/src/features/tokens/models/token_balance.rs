use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct TokenBalance {
    pub pk: Partition,
    pub sk: EntityType,

    pub meta_user_id: String,
    pub balance: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl TokenBalance {
    pub fn new(project_id: Partition, meta_user_id: String) -> Self {
        let now = crate::common::utils::time_utils::get_now();

        Self {
            pk: project_id,
            sk: EntityType::User(meta_user_id.clone()),
            meta_user_id,
            balance: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn keys(project_id: Partition, meta_user_id: String) -> (Partition, EntityType) {
        (project_id, EntityType::User(meta_user_id))
    }

    pub fn add_tokens(&mut self, amount: i64) {
        self.balance += amount;
        self.updated_at = crate::common::utils::time_utils::get_now();
    }

    pub fn deduct_tokens(&mut self, amount: i64) -> crate::common::Result<()> {
        if self.balance < amount {
            return Err(crate::features::tokens::TokenError::InsufficientTokens.into());
        }
        self.balance -= amount;
        self.updated_at = crate::common::utils::time_utils::get_now();
        Ok(())
    }
}

impl From<TokenBalance> for crate::features::tokens::TokenBalanceResponse {
    fn from(balance: TokenBalance) -> Self {
        Self {
            project_id: balance.pk,
            meta_user_id: balance.meta_user_id,
            balance: balance.balance,
            created_at: balance.created_at,
            updated_at: balance.updated_at,
        }
    }
}
