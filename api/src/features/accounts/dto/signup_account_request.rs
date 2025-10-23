use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct SignupAccountRequest {
    #[schemars(description = "Name of the entity")]
    pub name: String,
    #[schemars(description = "Email address of the account")]
    pub email: String,
    #[schemars(description = "Hashed password of the account")]
    pub hashed_password: String,
}
