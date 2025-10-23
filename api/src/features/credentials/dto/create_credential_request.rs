use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct CreateCredentialRequest {
    #[schemars(description = "Name/description for the credential")]
    pub name: String,
}
