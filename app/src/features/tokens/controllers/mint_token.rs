use crate::common::{CommonConfig, ProjectAuth, ProjectPartition, Result};
use crate::features::tokens::{ProjectToken, TokenBalance, TokenBalanceResponse, TokenError};
use dioxus::prelude::put;

#[put("/v1/projects/:project_id/tokens/:meta_user_id", auth: ProjectAuth)]
pub async fn mint_token_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    meta_user_id: String,
    amount: i64,
    #[allow(unused_variables)] description: Option<String>,
) -> Result<TokenBalanceResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();
    let project = auth.project;

    let (token_pk, token_sk) = ProjectToken::keys(project.pk.clone());
    let mut token = ProjectToken::get(cli, &token_pk, Some(token_sk))
        .await?
        .ok_or(TokenError::TokenNotFound)?;

    token.mint(amount);

    ProjectToken::updater(token.pk.clone(), token.sk.clone())
        .with_total_supply(token.total_supply)
        .with_circulating_supply(token.circulating_supply)
        .with_updated_at(token.updated_at)
        .execute(cli)
        .await?;

    let (balance_pk, balance_sk) = TokenBalance::keys(project.pk.clone(), meta_user_id.clone());
    let mut balance = TokenBalance::get(cli, &balance_pk, Some(balance_sk))
        .await?
        .unwrap_or_else(|| TokenBalance::new(project.pk.clone(), meta_user_id));

    balance.add_tokens(amount);

    if balance.created_at == balance.updated_at {
        balance.create(cli).await?;
    } else {
        TokenBalance::updater(balance.pk.clone(), balance.sk.clone())
            .with_balance(balance.balance)
            .with_updated_at(balance.updated_at)
            .execute(cli)
            .await?;
    }

    Ok(balance.into())
}
