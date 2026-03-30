use crate::common::{CommonConfig, Result};
use crate::features::accounts::Account;
use crate::features::projects::{Project, ProjectResponse};
use crate::features::tokens::ProjectToken;
use dioxus::prelude::post;

#[post("/v1/projects", account: Account)]
pub async fn create_project_handler(
    name: String,
    description: Option<String>,
    monthly_token_supply: i64,
    symbol: String,
    decimals: u8,
) -> Result<ProjectResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let project = Project::new(
        account.pk,
        name.clone(),
        description.clone(),
        monthly_token_supply,
    );

    let token = ProjectToken::new(
        project.pk.clone(),
        name,
        symbol,
        decimals,
        description,
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
