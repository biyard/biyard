pub mod accounts;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new().nest("/accounts", accounts::route()?))
}
