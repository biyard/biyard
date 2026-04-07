use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupAccountRequest {
    pub name: String,
    pub email: String,
    #[serde(alias = "hashed_password")]
    pub password: String,
}
