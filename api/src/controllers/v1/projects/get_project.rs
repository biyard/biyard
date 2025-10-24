use crate::features::accounts::Account;
use crate::features::projects::*;
use crate::*;

pub async fn get_project_handler(
    Extension(project): Extension<Project>,
    Path(_p): ProjectPath,
) -> Result<Json<ProjectResponse>> {
    Ok(Json(project.into()))
}
