use crate::features::points::TransactionType;
use crate::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, JsonSchema, OperationIo, Default,
)]
pub struct PointTransaction {
    #[schemars(description = "Composite key: PROJECT#<project_id>#META_USER#<meta_user_id>")]
    pub pk: CompositePartition,
    #[schemars(description = "Transaction ID: POINT_TRANSACTION#<transaction id>")]
    pub sk: EntityType,

    #[schemars(description = "Project ID")]
    #[dynamo(index = "gsi1", prefix = "PT", pk, name = "find_by_project")]
    pub project_id: Partition,

    #[schemars(description = "Meta user ID")]
    #[dynamo(index = "gsi2", pk, prefix = "PT", name = "find_by_meta_user")]
    pub meta_user_id: String,

    #[schemars(description = "Month in YYYY-MM format")]
    #[dynamo(index = "gsi1", sk, prefix = "MONTH", name = "find_by_project")]
    #[dynamo(index = "gsi2", sk, name = "find_by_meta_user")]
    pub month: String,

    #[schemars(description = "Transaction type")]
    pub transaction_type: TransactionType,

    #[schemars(description = "Amount of points")]
    pub amount: i64,

    #[schemars(description = "Target user ID for transfers")]
    pub target_user_id: Option<String>,

    #[schemars(description = "Description or memo")]
    pub description: Option<String>,

    #[schemars(description = "Creation timestamp")]
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
        let created_at = time_utils::get_now();
        let uuid = uuid::Uuid::new_v4().to_string();
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
