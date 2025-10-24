pub mod create_token;
pub mod mint_token;
pub mod get_token;
pub mod list_tokens;

use crate::*;

pub use create_token::*;
pub use mint_token::*;
pub use get_token::*;
pub use list_tokens::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/", post(create_token_handler))
        .route("/", get(list_tokens_handler))
        .route("/:token_id", get(get_token_handler))
        .route("/:token_id/mint", post(mint_token_handler))
        .route("/:token_id/balance/:meta_user_id", get(get_token_balance_handler)))
}
