use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct SignupAccountRequest {
    #[schemars(description = "Name of the entity")]
    pub name: String,
}
