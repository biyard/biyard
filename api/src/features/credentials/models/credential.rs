use crate::{features::credentials::CredentialStatus, *};

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, JsonSchema, OperationIo, Default,
)]
pub struct Credential {
    #[schemars(description = "ID of the credential")]
    pub pk: Partition,
    #[schemars(description = "Entity type of the credential")]
    pub sk: EntityType,

    #[schemars(description = "Account ID that owns this credential")]
    #[dynamo(index = "gsi1", pk, name = "find_by_account_id")]
    pub account_id: Partition,

    #[schemars(description = "GSI1 sort key (EntityType)")]
    #[dynamo(index = "gsi1", sk, name = "find_by_account_id")]
    pub gsi1_sk: EntityType,

    #[schemars(description = "Name/description of the credential")]
    pub name: String,

    #[schemars(description = "Hashed API key")]
    #[dynamo(index = "gsi2", pk, prefix = "KEY", name = "find_by_api_key_hash")]
    pub api_key_hash: String,

    #[schemars(description = "GSI2 sort key (EntityType)")]
    #[dynamo(index = "gsi2", sk, name = "find_by_api_key_hash")]
    pub gsi2_sk: EntityType,

    #[schemars(description = "Prefix of the API key for display (e.g., 'biyard_abc...')")]
    pub api_key_prefix: String,

    #[schemars(description = "Status of the credential")]
    pub status: CredentialStatus,

    #[schemars(description = "Creation timestamp")]
    pub created_at: i64,

    #[schemars(description = "Last update timestamp")]
    pub updated_at: i64,

    #[schemars(description = "Last used timestamp")]
    pub last_used_at: Option<i64>,
}

impl Credential {
    pub fn new(account_id: Partition, name: String, api_key: &str) -> Self {
        let now = time_utils::get_now();
        let uuid = uuid::Uuid::new_v4().to_string();
        let api_key_hash = password_utils::hash_password(api_key);

        // Store first 12 chars for display
        let api_key_prefix = if api_key.len() > 12 {
            api_key[..12].to_string()
        } else {
            api_key.to_string()
        };

        Self {
            pk: Partition::Credential(uuid),
            sk: EntityType::Credential,
            account_id,
            gsi1_sk: EntityType::Credential,
            name,
            api_key_hash,
            gsi2_sk: EntityType::Credential,
            api_key_prefix,
            status: CredentialStatus::Active,
            created_at: now,
            updated_at: now,
            last_used_at: None,
        }
    }
}
