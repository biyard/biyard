use crate::features::projects::*;
use crate::features::tokens::*;
use crate::*;
use validator::Validate;

pub async fn mint_token_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(project): Extension<Project>,
    Path(ProjectUserPathParam { meta_user_id, .. }): ProjectUserPath,
    Json(req): Json<MintTokenRequest>,
) -> Result<Json<TokenBalanceResponse>> {
    info!("Minting tokens for project: {:?}", project.pk);

    // Validate the request
    req.validate()?;

    // Get the token (1:1 with project)
    let (token_pk, token_sk) = ProjectToken::keys(project.pk.clone());
    let mut token = ProjectToken::get(&cli, &token_pk, Some(token_sk))
        .await?
        .ok_or(Error::TokenNotFound)?;

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
    let (balance_pk, balance_sk) =
        TokenBalance::keys(project.pk.clone().into(), meta_user_id.clone());

    let mut balance = TokenBalance::get(&cli, &balance_pk, Some(balance_sk.clone()))
        .await?
        .unwrap_or_else(|| TokenBalance::new(project.pk.clone(), meta_user_id));

    // Add tokens to balance
    balance.add_tokens(req.amount);

    // Save balance
    if balance.created_at == balance.updated_at {
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
