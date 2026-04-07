use crate::common::OrganizationRole;
use serde::{Deserialize, Serialize};

use super::EnterpriseResponse;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct CurrentEnterpriseResponse {
    pub enterprise: EnterpriseResponse,
    pub role: OrganizationRole,
}
