use crate::common::{ProjectPartition, Result};
use crate::features::projects::ProjectResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::ProjectViewerAuth;

#[get("/v1/projects/:project_id", auth: ProjectViewerAuth)]
pub async fn get_project_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
) -> Result<ProjectResponse> {
    Ok(auth.project.into())
}
