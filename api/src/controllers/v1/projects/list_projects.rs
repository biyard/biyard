use crate::features::accounts::Account;
use crate::features::projects::*;
use crate::*;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct ListProjectsResponse {
    pub projects: Vec<ProjectResponse>,
}

pub async fn list_projects_handler(
    State(AppState { cli, .. }): State<AppState>,
    account: Account,
) -> Result<Json<ListProjectsResponse>> {
    info!("Listing projects for account: {:?}", account.pk);

    let (projects, _) = Project::find_by_account_id(&cli, &account.pk, ProjectQueryOption::builder())
        .await?;

    let project_responses: Vec<ProjectResponse> = projects.into_iter().map(|p| p.into()).collect();

    Ok(Json(ListProjectsResponse {
        projects: project_responses,
    }))
}
