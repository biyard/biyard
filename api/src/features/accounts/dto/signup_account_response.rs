use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema, Default)]
pub struct SignupAccountResponse {
    #[schemars(description = "Status of the operation")]
    pub status: String,
}
