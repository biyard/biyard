use crate::features::accounts::Account;
use crate::features::projects::*;
use crate::features::tokens::*;
use crate::*;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct ListTokensResponse {
    pub tokens: Vec<TokenResponse>,
}

pub async fn list_tokens_handler(
    State(AppState { cli, .. }): State<AppState>,
    account: Account,
    Path(project_id): Path<String>,
) -> Result<Json<ListTokensResponse>> {
    info!("Listing tokens for project: {}", project_id);

    let project_pk = Partition::Project(project_id);
    let project = Project::get(&cli, project_pk.clone(), Some(EntityType::Project))
        .await?
        .ok_or(Error::ProjectNotFound)?;

    // Verify ownership
    project.verify_ownership(&account)?;

    // Query tokens by project
    let (tokens, _) = ProjectToken::find_by_project(&cli, &project_pk, ProjectTokenQueryOption::builder())
        .await?;

    let token_responses: Vec<TokenResponse> = tokens.into_iter().map(|t| t.into()).collect();

    Ok(Json(ListTokensResponse {
        tokens: token_responses,
    }))
}
