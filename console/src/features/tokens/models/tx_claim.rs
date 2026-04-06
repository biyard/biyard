use crate::common::*;

/// Record for deduplicating claim requests.
/// pk = project partition, sk = TxClaim#<tx_hash>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct TxClaim {
    pub pk: Partition,
    pub sk: EntityType,

    pub tx_hash: String,
    pub to_address: String,
    pub amount: i64,
    pub chain_id: u64,
    pub created_at: i64,
}

impl TxClaim {
    pub fn new(
        project_id: Partition,
        tx_hash: String,
        to_address: String,
        amount: i64,
        chain_id: u64,
    ) -> Self {
        Self {
            pk: project_id,
            sk: EntityType::TxClaim(tx_hash.clone()),
            tx_hash,
            to_address,
            amount,
            chain_id,
            created_at: crate::common::utils::time_utils::get_now(),
        }
    }
}
