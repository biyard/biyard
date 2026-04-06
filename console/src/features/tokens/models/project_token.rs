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

    pub contract_address: Option<String>,
    pub chain_id: Option<u64>,
    pub deployment_tx_hash: Option<String>,

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
        initial_supply: i64,
    ) -> Self {
        let now = crate::common::utils::time_utils::get_now();
        let initial_supply = initial_supply.max(0);

        Self {
            pk: project_id,
            sk: EntityType::Token,
            name,
            symbol,
            decimals,
            total_supply: initial_supply,
            circulating_supply: initial_supply,
            description,
            contract_address: None,
            chain_id: None,
            deployment_tx_hash: None,
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
            contract_address: token.contract_address,
            chain_id: token.chain_id,
            deployment_tx_hash: token.deployment_tx_hash,
            created_at: token.created_at,
            updated_at: token.updated_at,
        }
    }
}
