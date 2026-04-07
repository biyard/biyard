use crate::common::*;
use crate::features::credentials::CredentialStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct Credential {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi1", prefix = "CRED", pk, name = "find_by_account_id")]
    pub account_id: Partition,

    #[serde(default)]
    #[dynamo(index = "gsi3", prefix = "CRED", pk, name = "find_by_organization_id")]
    pub organization_id: Partition,

    pub name: String,

    #[dynamo(index = "gsi2", pk, prefix = "CRED", name = "find_by_api_key_hash")]
    pub api_key_hash: String,

    pub api_key_prefix: String,
    pub status: CredentialStatus,

    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", sk)]
    #[dynamo(index = "gsi3", sk)]
    pub created_at: i64,

    pub updated_at: i64,
    pub last_used_at: Option<i64>,
}

impl Credential {
    pub fn new(
        account_id: Partition,
        organization_id: Partition,
        name: String,
        api_key: &str,
    ) -> Self {
        let now = crate::common::utils::time_utils::get_now();
        let uuid = uuid::Uuid::now_v7().to_string();
        let api_key_hash = crate::common::utils::password_utils::hash_secret_for_lookup(api_key);

        let api_key_prefix = if api_key.len() > 12 {
            api_key[..12].to_string()
        } else {
            api_key.to_string()
        };

        Self {
            pk: Partition::Credential(uuid),
            sk: EntityType::Credential,
            account_id,
            organization_id,
            name,
            api_key_hash,
            api_key_prefix,
            status: CredentialStatus::Active,
            created_at: now,
            updated_at: now,
            last_used_at: None,
        }
    }
}

impl From<Credential> for crate::features::credentials::CredentialResponse {
    fn from(credential: Credential) -> Self {
        Self {
            id: match credential.pk {
                Partition::Credential(id) => id,
                _ => String::new(),
            },
            name: credential.name,
            api_key_prefix: credential.api_key_prefix,
            status: credential.status,
            created_at: credential.created_at,
            last_used_at: credential.last_used_at,
            api_key: credential.api_key_hash,
        }
    }
}

impl From<Credential> for crate::features::credentials::CredentialSummaryResponse {
    fn from(credential: Credential) -> Self {
        Self {
            id: match credential.pk {
                Partition::Credential(id) => id,
                _ => String::new(),
            },
            name: credential.name,
            api_key_prefix: credential.api_key_prefix,
            status: credential.status,
            created_at: credential.created_at,
            last_used_at: credential.last_used_at,
        }
    }
}
