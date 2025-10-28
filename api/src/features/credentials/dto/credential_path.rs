use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct CredentialPathParam {
    #[schemars(description = "The unique identifier for the project")]
    pub credential_id: String,
}

pub type CredentialPath = Path<CredentialPathParam>;
