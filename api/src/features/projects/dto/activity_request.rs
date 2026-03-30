use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct ActivityRequest {
    #[schemars(description = "Meta user ID managed by the customer")]
    pub meta_user_id: String,

    #[schemars(description = "Type of activity (e.g. 'login', 'purchase', 'referral')")]
    pub activity_type: String,

    #[schemars(
        description = "Raw value of the activity (divided by steps_per_point to yield earned points)"
    )]
    pub value: i64,

    #[schemars(description = "Human-readable description of the activity")]
    pub description: String,

    #[schemars(
        description = "Number of activity value units required per point (default: 100)"
    )]
    #[serde(default = "ActivityRequest::default_steps_per_point")]
    pub steps_per_point: i64,
}

impl ActivityRequest {
    pub fn default_steps_per_point() -> i64 {
        100
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct ActivityResponse {
    #[schemars(description = "Points earned for this activity (value / steps_per_point)")]
    pub points_earned: i64,

    #[schemars(description = "User's total points balance after this activity")]
    pub total_points: i64,
}
