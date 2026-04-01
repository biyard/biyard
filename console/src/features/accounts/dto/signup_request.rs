use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupAccountRequest {
    pub name: String,
    pub email: String,
    pub hashed_password: String,
}
