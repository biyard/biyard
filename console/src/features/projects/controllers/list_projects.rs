use crate::common::{ListResponse, Result};
use crate::features::projects::ProjectResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::CommonConfig;
#[cfg(feature = "server")]
use crate::common::EnterpriseContextAuth;
#[cfg(feature = "server")]
use crate::features::projects::{Project, ProjectQueryOption};

#[get("/v1/projects?limit&bookmark", auth: EnterpriseContextAuth)]
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

    let (projects, bookmark) =
        Project::find_by_organization_id(cli, &auth.enterprise.pk, opt).await?;

    let projects: Vec<ProjectResponse> = projects.into_iter().map(|p| p.into()).collect();

    Ok((projects, bookmark).into())
}
