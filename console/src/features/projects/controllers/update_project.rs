use crate::common::{ProjectPartition, Result};
use crate::features::projects::{ProjectResponse, ProjectStatus};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::projects::Project;

#[put("/v1/projects/:project_id", auth: ProjectAdminAuth)]
pub async fn update_project_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    name: Option<String>,
    description: Option<String>,
    brand_logo_url: Option<String>,
    monthly_token_supply: Option<i64>,
    treasury_reserve_rate: Option<f64>,
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
    if let Some(brand_logo_url) = brand_logo_url {
        updater = updater.with_brand_logo_url(brand_logo_url);
    }
    if let Some(monthly_token_supply) = monthly_token_supply {
        updater = updater.with_monthly_token_supply(monthly_token_supply);
    }
    if let Some(treasury_reserve_rate) = treasury_reserve_rate {
        updater = updater.with_treasury_reserve_rate(treasury_reserve_rate.clamp(0.0, 1.0));
    }
    if let Some(status) = status {
        updater = updater.with_status(status);
    }

    updater = updater.with_updated_at(crate::common::utils::time_utils::get_now());

    let updated_project = updater.execute(cli).await?;

    Ok(updated_project.into())
}
