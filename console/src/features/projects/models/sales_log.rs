use crate::common::*;

/// A single revenue/sale record for a brand project.
///
/// Real revenue flows into this table when the brand's POS/backend
/// calls the sales log API. The treasury page also exposes a manual
/// entry form so operators can enter historical rows during onboarding
/// or for demos.
///
/// This is **not** a simulation: every row represents a real sale
/// that should have triggered an on-chain `recordPurchase` call on
/// the treasury contract. The current treasury balance itself lives
/// on-chain and is not mirrored into DynamoDB.
///
/// Access pattern: list sales logs by project, newest first. Backed
/// by `gsi1` with `project_id` as the partition and `created_at` as
/// the range (prefixed `TS` so sorting is numeric/lexicographic safe).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct SalesLog {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi1", prefix = "SALES", pk, name = "find_by_project")]
    pub project_id: Partition,

    /// Sale amount in the brand's operating currency (KRW, integer).
    pub amount: i64,

    /// Optional free-form memo (e.g. order id, channel, item).
    pub memo: Option<String>,

    /// Unix epoch ms at creation time. Also used as the `gsi1` sort
    /// key (`TS#<created_at>`) so listing by project is naturally
    /// time-ordered without application-side sorting.
    #[dynamo(index = "gsi1", sk, prefix = "TS")]
    pub created_at: i64,
}

impl SalesLog {
    pub fn new(project_pk: Partition, amount: i64, memo: Option<String>) -> Self {
        let created_at = crate::common::utils::time_utils::get_now();
        let uuid = uuid::Uuid::now_v7().to_string();

        Self {
            pk: Partition::SalesLog(uuid),
            sk: EntityType::SalesLog,
            project_id: project_pk,
            amount,
            memo,
            created_at,
        }
    }
}

impl From<SalesLog> for crate::features::projects::SalesLogResponse {
    fn from(log: SalesLog) -> Self {
        let id = match &log.pk {
            Partition::SalesLog(id) => id.clone(),
            _ => String::new(),
        };

        Self {
            id,
            amount: log.amount,
            memo: log.memo,
            created_at: log.created_at,
        }
    }
}
