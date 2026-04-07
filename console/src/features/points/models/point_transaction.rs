use crate::common::*;
use crate::features::points::TransactionType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct PointTransaction {
    pub pk: CompositePartition,
    pub sk: EntityType,

    #[dynamo(index = "gsi1", prefix = "PT", pk, name = "find_by_project")]
    #[dynamo(index = "gsi3", prefix = "PT", pk, name = "find_by_project_time")]
    pub project_id: Partition,

    #[dynamo(index = "gsi2", pk, prefix = "PT", name = "find_by_meta_user")]
    pub meta_user_id: String,

    #[dynamo(index = "gsi1", sk, prefix = "MONTH", name = "find_by_project")]
    #[dynamo(index = "gsi2", sk, name = "find_by_meta_user")]
    pub month: String,

    pub transaction_type: TransactionType,
    pub amount: i64,
    pub target_user_id: Option<String>,
    pub description: Option<String>,

    /// Also used as the `gsi3` sort key (prefix `TS`) so the list
    /// handler can return transactions in chronological order without
    /// application-side sorting. See `find_by_project_time`.
    #[dynamo(index = "gsi3", sk, prefix = "TS")]
    pub created_at: i64,
}

impl PointTransaction {
    pub fn new(
        project_pk: Partition,
        meta_user_id: String,
        month: String,
        transaction_type: TransactionType,
        amount: i64,
        target_user_id: Option<String>,
        description: Option<String>,
    ) -> Self {
        let created_at = crate::common::utils::time_utils::get_now();
        let uuid = uuid::Uuid::now_v7().to_string();
        let user_pk = Partition::MetaUser(meta_user_id.clone());

        Self {
            pk: CompositePartition(project_pk.clone(), user_pk),
            sk: EntityType::PointTransaction(uuid),
            project_id: project_pk,
            meta_user_id,
            month,
            transaction_type,
            amount,
            target_user_id,
            description,
            created_at,
        }
    }
}

impl From<PointTransaction> for crate::features::points::PointTransactionResponse {
    fn from(tx: PointTransaction) -> Self {
        Self {
            project_id: tx.project_id,
            meta_user_id: tx.meta_user_id,
            month: tx.month,
            transaction_type: tx.transaction_type,
            amount: tx.amount,
            target_user_id: tx.target_user_id,
            description: tx.description,
            created_at: tx.created_at,
        }
    }
}
