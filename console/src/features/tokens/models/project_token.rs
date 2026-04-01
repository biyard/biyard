use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct ProjectToken {
    pub pk: Partition,
    pub sk: EntityType,

    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: i64,
    pub circulating_supply: i64,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl ProjectToken {
    pub fn new(
        project_id: Partition,
        name: String,
        symbol: String,
        decimals: u8,
        description: Option<String>,
    ) -> Self {
        let now = crate::common::utils::time_utils::get_now();

        Self {
            pk: project_id,
            sk: EntityType::Token,
            name,
            symbol,
            decimals,
            total_supply: 0,
            circulating_supply: 0,
            description,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn keys(project_id: Partition) -> (Partition, EntityType) {
        (project_id, EntityType::Token)
    }

    pub fn mint(&mut self, amount: i64) {
        self.total_supply += amount;
        self.circulating_supply += amount;
        self.updated_at = crate::common::utils::time_utils::get_now();
    }

    pub fn burn(&mut self, amount: i64) -> crate::common::Result<()> {
        if self.circulating_supply < amount {
            return Err(crate::features::tokens::TokenError::InsufficientTokens.into());
        }
        self.total_supply -= amount;
        self.circulating_supply -= amount;
        self.updated_at = crate::common::utils::time_utils::get_now();
        Ok(())
    }
}

impl From<ProjectToken> for crate::features::tokens::TokenResponse {
    fn from(token: ProjectToken) -> Self {
        Self {
            pk: token.pk,
            name: token.name,
            symbol: token.symbol,
            decimals: token.decimals,
            total_supply: token.total_supply,
            circulating_supply: token.circulating_supply,
            description: token.description,
            created_at: token.created_at,
            updated_at: token.updated_at,
        }
    }
}
