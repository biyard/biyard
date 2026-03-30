use crate::common::{CommonConfig, ListResponse, Result};
use crate::features::accounts::Account;
use crate::features::projects::{Project, ProjectQueryOption, ProjectResponse};
use dioxus::prelude::get;

#[get("/v1/projects", account: Account)]
pub async fn list_projects_handler(
    limit: i32,
    bookmark: Option<String>,
) -> Result<ListResponse<ProjectResponse>> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let mut opt = ProjectQueryOption::builder().limit(limit);
    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (projects, bookmark) = Project::find_by_account_id(cli, &account.pk, opt).await?;
    let projects: Vec<ProjectResponse> = projects.into_iter().map(|p| p.into()).collect();

    Ok((projects, bookmark).into())
}
