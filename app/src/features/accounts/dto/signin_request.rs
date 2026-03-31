use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigninAccountRequest {
    pub email: String,
    pub password: String,
}
