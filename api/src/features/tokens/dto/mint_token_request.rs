use crate::*;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo, Validate)]
pub struct MintTokenRequest {
    #[schemars(description = "Meta user ID to mint tokens to")]
    pub meta_user_id: String,

    #[validate(range(min = 1))]
    #[schemars(description = "Amount of tokens to mint")]
    pub amount: i64,

    #[schemars(description = "Description or memo")]
    pub description: Option<String>,
}
