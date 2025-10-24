use crate::*;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo, Validate)]
pub struct CreateTokenRequest {
    #[validate(length(min = 1, max = 255))]
    #[schemars(description = "Token name")]
    pub name: String,

    #[validate(length(min = 1, max = 10))]
    #[schemars(description = "Token symbol")]
    pub symbol: String,

    #[validate(range(max = 18))]
    #[schemars(description = "Number of decimals")]
    pub decimals: u8,

    #[schemars(description = "Token description")]
    pub description: Option<String>,
}
