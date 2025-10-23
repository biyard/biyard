use crate::{features::credentials::CredentialStatus, *};

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema, Default)]
pub struct CredentialResponse {
    #[schemars(description = "ID of the credential")]
    pub pk: Partition,

    #[schemars(description = "Name of the credential")]
    pub name: String,

    #[schemars(description = "API key prefix for display")]
    pub api_key_prefix: String,

    #[schemars(description = "Status of the credential")]
    pub status: CredentialStatus,

    #[schemars(description = "Creation timestamp")]
    pub created_at: i64,

    #[schemars(description = "Last used timestamp")]
    pub last_used_at: Option<i64>,

    #[schemars(description = "The full API key (only returned on creation)")]
    pub api_key: Option<String>,
}
