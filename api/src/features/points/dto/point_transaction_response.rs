use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct PointTransactionResponse {
    #[schemars(description = "Project ID")]
    pub project_id: Partition,

    #[schemars(description = "Meta user ID")]
    pub meta_user_id: String,

    #[schemars(description = "Month in YYYY-MM format")]
    pub month: String,

    #[schemars(description = "Transaction type")]
    pub transaction_type: crate::features::points::TransactionType,

    #[schemars(description = "Amount of points")]
    pub amount: i64,

    #[schemars(description = "Target user ID for transfers")]
    pub target_user_id: Option<String>,

    #[schemars(description = "Description or memo")]
    pub description: Option<String>,

    #[schemars(description = "Creation timestamp")]
    pub created_at: i64,
}

impl From<crate::features::points::PointTransaction> for PointTransactionResponse {
    fn from(tx: crate::features::points::PointTransaction) -> Self {
        let project_id = match tx.project_id {
            Partition::Project(id) => Partition::Project(id),
            _ => panic!("Invalid project_id partition type"),
        };

        Self {
            project_id,
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
