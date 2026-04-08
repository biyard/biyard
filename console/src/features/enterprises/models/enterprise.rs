use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct Enterprise {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi1", pk, prefix = "OWNER", name = "find_by_owner_account_id")]
    pub owner_account_id: Partition,

    pub name: String,
    #[serde(default)]
    pub legacy_account_sync_at: Option<i64>,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,

    pub updated_at: i64,
}

impl Enterprise {
    pub fn new(pk: Partition, owner_account_id: Partition, name: String) -> Self {
        let now = crate::common::utils::time_utils::get_now();

        Self {
            pk,
            sk: EntityType::Enterprise,
            owner_account_id,
            name,
            legacy_account_sync_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}

impl From<Enterprise> for crate::features::enterprises::EnterpriseResponse {
    fn from(enterprise: Enterprise) -> Self {
        let id = match &enterprise.pk {
            Partition::Enterprise(id) => id.clone(),
            _ => String::new(),
        };

        Self {
            id,
            pk: enterprise.pk,
            owner_account_id: enterprise.owner_account_id,
            name: enterprise.name,
            created_at: enterprise.created_at,
            updated_at: enterprise.updated_at,
        }
    }
}
