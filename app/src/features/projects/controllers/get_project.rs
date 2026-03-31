use crate::common::{ProjectPartition, Result};
use crate::features::projects::ProjectResponse;
use dioxus::prelude::get;

#[cfg(feature = "server")]
use crate::common::ProjectAuth;

#[get("/v1/projects/:project_id", auth: ProjectAuth)]
pub async fn get_project_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
) -> Result<ProjectResponse> {
    Ok(auth.project.into())
}
