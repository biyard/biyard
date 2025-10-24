use crate::*;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo, Validate)]
pub struct TransferTokenRequest {
    #[schemars(description = "Source meta user ID")]
    pub from_user_id: String,

    #[schemars(description = "Target meta user ID")]
    pub to_user_id: String,

    #[validate(range(min = 1))]
    #[schemars(description = "Amount of tokens to transfer")]
    pub amount: i64,

    #[schemars(description = "Description or memo")]
    pub description: Option<String>,
}
