use crate::common::*;
use crate::features::tokens::DistributionSlotEntry;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct ProjectToken {
    pub pk: Partition,
    pub sk: EntityType,

    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub circulating_supply: i64,
    pub description: Option<String>,

    pub contract_address: Option<String>,
    pub treasury_contract_address: Option<String>,
    pub multisig_address: Option<String>,
    pub stable_token_address: Option<String>,
    pub chain_id: Option<u64>,
    pub deployment_tx_hash: Option<String>,
    pub treasury_deployment_tx_hash: Option<String>,
    pub multisig_deployment_tx_hash: Option<String>,
    #[serde(default)]
    pub treasury_reserve_bps: u64,

    #[serde(default)]
    pub monthly_emission: i64,

    #[serde(default = "default_decay_rate_bps")]
    pub decay_rate_bps: u16,

    #[serde(default)]
    pub distribution_slots: Vec<DistributionSlotEntry>,

    #[serde(default)]
    pub last_minted_month: Option<String>,

    #[serde(default)]
    pub deploying: bool,

    /// "YYYY-MM" start month for emission (used to compute startTimestamp on deploy).
    #[serde(default)]
    pub start_month: Option<String>,

    pub created_at: i64,
    pub updated_at: i64,
}

fn default_decay_rate_bps() -> u16 {
    500
}

impl ProjectToken {
    pub fn new(
        project_id: Partition,
        name: String,
        symbol: String,
        decimals: u8,
        description: Option<String>,
        monthly_emission: i64,
        decay_rate_bps: u16,
        distribution_slots: Vec<DistributionSlotEntry>,
        stable_token_address: Option<String>,
        chain_id: Option<u64>,
        start_month: Option<String>,
    ) -> Self {
        let now = crate::common::utils::time_utils::get_now();

        Self {
            pk: project_id,
            sk: EntityType::Token,
            name,
            symbol,
            decimals,
            circulating_supply: 0,
            description,
            contract_address: None,
            treasury_contract_address: None,
            multisig_address: None,
            stable_token_address,
            chain_id,
            deployment_tx_hash: None,
            treasury_deployment_tx_hash: None,
            multisig_deployment_tx_hash: None,
            treasury_reserve_bps: 0,
            monthly_emission,
            decay_rate_bps,
            distribution_slots,
            last_minted_month: None,
            deploying: false,
            start_month,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn keys(project_id: Partition) -> (Partition, EntityType) {
        (project_id, EntityType::Token)
    }

    pub fn mint(&mut self, amount: i64) {
        self.circulating_supply += amount;
        self.updated_at = crate::common::utils::time_utils::get_now();
    }

    pub fn burn(&mut self, amount: i64) -> crate::common::Result<()> {
        if self.circulating_supply < amount {
            return Err(crate::features::tokens::TokenError::InsufficientTokens.into());
        }
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
            circulating_supply: token.circulating_supply,
            description: token.description,
            contract_address: token.contract_address,
            treasury_contract_address: token.treasury_contract_address,
            multisig_address: token.multisig_address,
            stable_token_address: token.stable_token_address,
            chain_id: token.chain_id,
            deployment_tx_hash: token.deployment_tx_hash,
            treasury_deployment_tx_hash: token.treasury_deployment_tx_hash,
            multisig_deployment_tx_hash: token.multisig_deployment_tx_hash,
            treasury_reserve_bps: token.treasury_reserve_bps,
            monthly_emission: token.monthly_emission,
            decay_rate_bps: token.decay_rate_bps,
            distribution_slots: token.distribution_slots,
            last_minted_month: token.last_minted_month,
            deploying: token.deploying,
            start_month: token.start_month,
            created_at: token.created_at,
            updated_at: token.updated_at,
        }
    }
}
