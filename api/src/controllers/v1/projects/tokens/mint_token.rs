use crate::features::accounts::Account;
use crate::features::projects::*;
use crate::features::tokens::*;
use crate::*;
use validator::Validate;

pub async fn mint_token_handler(
    State(AppState { cli, .. }): State<AppState>,
    account: Account,
    Path((project_id, token_id)): Path<(String, String)>,
    Json(req): Json<MintTokenRequest>,
) -> Result<Json<TokenBalanceResponse>> {
    info!("Minting tokens for token: {} in project: {}", token_id, project_id);

    // Validate the request
    req.validate()?;

    let project_pk = Partition::Project(project_id);
    let project = Project::get(&cli, project_pk.clone(), Some(EntityType::Project))
        .await?
        .ok_or(Error::ProjectNotFound)?;

    // Verify ownership
    project.verify_ownership(&account)?;

    // Get the token
    let token_pk = Partition::Token(token_id);
    let mut token = ProjectToken::get(&cli, token_pk.clone(), Some(EntityType::Token))
        .await?
        .ok_or(Error::TokenNotFound)?;

    // Verify token belongs to this project
    if token.project_id != project_pk {
        return Err(Error::TokenNotFound);
    }

    // Mint tokens
    token.mint(req.amount);

    // Update token
    ProjectToken::updater(token.pk.clone(), token.sk.clone())
        .with_total_supply(token.total_supply)
        .with_circulating_supply(token.circulating_supply)
        .with_updated_at(token.updated_at)
        .execute(&cli)
        .await?;

    // Get or create token balance for user
    let balance_pk = format!("{}#USER#{}", token_pk.to_string(), req.meta_user_id);
    let balance_pk = Partition::TokenBalance(balance_pk);

    let mut balance = TokenBalance::get(&cli, balance_pk.clone(), Some(EntityType::TokenBalance))
        .await?
        .unwrap_or_else(|| TokenBalance::new(token_pk, project_pk, req.meta_user_id.clone()));

    // Add tokens to balance
    balance.add_tokens(req.amount);

    // Save balance
    if balance.created_at == balance.updated_at && balance.created_at == time_utils::get_now() {
        // New balance, create it
        balance.create(&cli).await?;
    } else {
        // Update existing balance
        TokenBalance::updater(balance.pk.clone(), balance.sk.clone())
            .with_balance(balance.balance)
            .with_updated_at(balance.updated_at)
            .execute(&cli)
            .await?;
    }

    Ok(Json(balance.into()))
}
