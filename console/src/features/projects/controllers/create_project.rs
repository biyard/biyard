use crate::common::Result;
use crate::features::projects::ProjectResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::CommonConfig;
#[cfg(feature = "server")]
use crate::common::EnterpriseContextAuth;
#[cfg(feature = "server")]
use crate::common::OrganizationRole;
#[cfg(feature = "server")]
use crate::features::projects::Project;

#[post("/v1/projects", auth: EnterpriseContextAuth)]
pub async fn create_project_handler(
    name: String,
    description: Option<String>,
    brand_logo_url: Option<String>,
    monthly_token_supply: i64,
    treasury_reserve_rate: f64,
) -> Result<ProjectResponse> {
    // Only Admin or higher can create a brand within an enterprise.
    // Viewers have read-only access to existing brands and cannot
    // provision new ones.
    if !auth.role.allows(OrganizationRole::Admin) {
        return Err(crate::common::Error::Forbidden);
    }

    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let project = Project::new(
        auth.account.pk,
        auth.enterprise.pk,
        name,
        description,
        monthly_token_supply,
        brand_logo_url,
        treasury_reserve_rate.clamp(0.0, 1.0),
    );

    project.create(cli).await?;

    Ok(project.into())
}
