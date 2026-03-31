use crate::common::{EntityType, ProjectPartition, Result};
use crate::features::projects::{ProjectResponse, ProjectStatus};
use dioxus::prelude::put;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAuth};
#[cfg(feature = "server")]
use crate::features::projects::Project;

#[put("/v1/projects/:project_id", auth: ProjectAuth)]
pub async fn update_project_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    name: Option<String>,
    description: Option<String>,
    monthly_token_supply: Option<i64>,
    status: Option<ProjectStatus>,
) -> Result<ProjectResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;

    let mut updater = Project::updater(project.pk, EntityType::Project);

    if let Some(name) = name {
        updater = updater.with_name(name);
    }
    if let Some(description) = description {
        updater = updater.with_description(description);
    }
    if let Some(monthly_token_supply) = monthly_token_supply {
        updater = updater.with_monthly_token_supply(monthly_token_supply);
    }
    if let Some(status) = status {
        updater = updater.with_status(status);
    }

    updater = updater.with_updated_at(crate::common::utils::time_utils::get_now());

    let updated_project = updater.execute(cli).await?;

    Ok(updated_project.into())
}
