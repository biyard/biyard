use crate::*;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo, Validate)]
pub struct UpdateProjectRequest {
    #[validate(length(min = 1, max = 255))]
    #[schemars(description = "Name of the project")]
    pub name: Option<String>,

    #[schemars(description = "Description of the project")]
    pub description: Option<String>,

    #[validate(range(min = 1))]
    #[schemars(description = "Monthly points supply")]
    pub monthly_points_supply: Option<i64>,

    #[validate(range(min = 1))]
    #[schemars(description = "Monthly token supply")]
    pub monthly_token_supply: Option<i64>,

    #[validate(range(min = 0.0))]
    #[schemars(description = "Exchange ratio for point-to-token conversion")]
    pub exchange_ratio: Option<f64>,

    #[schemars(description = "Project status")]
    pub status: Option<String>,
}
