use crate::features::accounts::Account;
use crate::features::projects::*;
use crate::*;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct ListProjectsResponse {
    pub projects: Vec<ProjectResponse>,
}

pub async fn list_projects_handler(
    State(AppState { cli, .. }): State<AppState>,
    NoApi(account): NoApi<Account>,
    Query(params): PaginationQuery,
) -> Result<Json<ListResponse<ProjectResponse>>> {
    debug!("Listing projects for account: {:?}", account.pk);
    let mut opt = ProjectQueryOption::builder().limit(params.limit.unwrap_or(10));

    if let Some(bookmark) = params.bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (projects, bookmark) = Project::find_by_account_id(&cli, &account.pk, opt).await?;

    let projects: Vec<ProjectResponse> = projects.into_iter().map(|p| p.into()).collect();

    Ok(Json((projects, bookmark).into()))
}
