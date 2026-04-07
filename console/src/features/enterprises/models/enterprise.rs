use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct Enterprise {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi1", pk, name = "find_by_owner_account_id")]
    pub owner_account_id: Partition,

    #[dynamo(index = "gsi1", sk, name = "find_by_owner_account_id")]
    pub gsi1_sk: EntityType,

    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub legacy_account_sync_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Enterprise {
    pub fn new(pk: Partition, owner_account_id: Partition, name: String) -> Self {
        let now = crate::common::utils::time_utils::get_now();
        let slug = slugify(&name);

        Self {
            pk,
            sk: EntityType::Enterprise,
            owner_account_id,
            gsi1_sk: EntityType::Enterprise,
            name,
            slug,
            legacy_account_sync_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}

fn slugify(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut last_was_dash = false;

    for ch in input.chars() {
        let lowered = ch.to_ascii_lowercase();
        if lowered.is_ascii_alphanumeric() {
            out.push(lowered);
            last_was_dash = false;
        } else if !last_was_dash {
            out.push('-');
            last_was_dash = true;
        }
    }

    let trimmed = out.trim_matches('-');
    if trimmed.is_empty() {
        "enterprise".to_string()
    } else {
        trimmed.to_string()
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
            slug: enterprise.slug,
            created_at: enterprise.created_at,
            updated_at: enterprise.updated_at,
        }
    }
}
