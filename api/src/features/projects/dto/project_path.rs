use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct ProjectPathParam {
    #[schemars(description = "The unique identifier for the project")]
    pub project_id: String,
}

pub type ProjectPath = Path<ProjectPathParam>;
