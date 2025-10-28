use crate::*;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo, Validate)]
pub struct CreateProjectRequest {
    #[validate(length(min = 1, max = 255))]
    #[schemars(description = "Name of the project")]
    pub name: String,

    #[schemars(description = "Description of the project")]
    pub description: Option<String>,

    #[validate(range(min = 0))]
    #[schemars(description = "Monthly token supply, 0 means manual provisioning")]
    pub monthly_token_supply: i64,
}
