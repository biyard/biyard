use crate::{
    features::credentials::{Credential, CredentialStatus},
    *,
};

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema, Default)]
pub struct CredentialSummaryResponse {
    #[schemars(description = "ID of the credential")]
    pub id: String,

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
}

impl From<Credential> for CredentialSummaryResponse {
    fn from(credential: Credential) -> Self {
        Self {
            id: match credential.pk {
                Partition::Credential(id) => id,
                _ => panic!("Invalid partition key for Credential"),
            },
            name: credential.name,
            api_key_prefix: credential.api_key_prefix,
            status: credential.status,
            created_at: credential.created_at,
            last_used_at: credential.last_used_at,
        }
    }
}
