// pub mod create_token;
// pub use create_token::*;

// pub mod list_tokens;
// pub use list_tokens::*;

pub mod get_token;
pub use get_token::*;

pub mod mint_token;
pub use mint_token::*;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/", get(get_token_handler).put(mint_token_handler))
        .route("/balance/:meta_user_id", get(get_token_balance_handler)))
}
