use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct SigninAccountRequest {
    #[schemars(description = "Email address of the account")]
    pub email: String,
    #[schemars(description = "Password of the account")]
    pub password: String,
}
