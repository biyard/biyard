use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ProjectPointPathParam {
    #[schemars(description = "The unique identifier for the meta user from customer's service")]
    pub meta_user_id: String,
}

pub type ProjectPointPath = Path<ProjectPointPathParam>;
