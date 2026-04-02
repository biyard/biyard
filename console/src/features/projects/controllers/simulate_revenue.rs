use crate::common::{ProjectPartition, Result};
use crate::features::projects::ProjectResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, ProjectAuth};
#[cfg(feature = "server")]
use crate::features::projects::Project;

#[post("/v1/projects/:project_id/treasury/simulate-revenue", auth: ProjectAuth)]
pub async fn simulate_revenue_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    revenue_amount: i64,
) -> Result<ProjectResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;

    let revenue_amount = revenue_amount.max(0);
    let reserve_add =
        ((revenue_amount as f64) * project.treasury_reserve_rate.clamp(0.0, 1.0)).round() as i64;

    let updated_project = Project::updater(project.pk, EntityType::Project)
        .with_simulated_sales_total(project.simulated_sales_total + revenue_amount)
        .with_treasury_balance(project.treasury_balance + reserve_add)
        .with_updated_at(crate::common::utils::time_utils::get_now())
        .execute(cli)
        .await?;

    Ok(updated_project.into())
}
