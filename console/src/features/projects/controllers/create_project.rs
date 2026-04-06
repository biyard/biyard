use crate::common::Result;
use crate::features::projects::ProjectResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::CommonConfig;
#[cfg(feature = "server")]
use crate::features::accounts::Account;
#[cfg(feature = "server")]
use crate::features::projects::Project;
#[cfg(feature = "server")]
use crate::features::tokens::ProjectToken;

#[post("/v1/projects", account: Account)]
pub async fn create_project_handler(
    name: String,
    description: Option<String>,
    token_name: Option<String>,
    brand_logo_url: Option<String>,
    monthly_token_supply: i64,
    treasury_reserve_rate: f64,
    symbol: String,
    decimals: u8,
    initial_token_supply: i64,
) -> Result<ProjectResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let project = Project::new(
        account.pk,
        name.clone(),
        description.clone(),
        monthly_token_supply,
        brand_logo_url,
        treasury_reserve_rate.clamp(0.0, 1.0),
    );

    let token_name = token_name.unwrap_or_else(|| name.clone());

    let token = ProjectToken::new(
        project.pk.clone(),
        token_name,
        symbol,
        decimals,
        description,
        initial_token_supply,
    );

    crate::transact_write_items!(
        cli,
        vec![
            project.create_transact_write_item(),
            token.create_transact_write_item()
        ]
    )?;

    Ok(project.into())
}
