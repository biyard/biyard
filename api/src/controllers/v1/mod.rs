pub mod accounts;
pub mod credentials;
pub mod projects;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .nest("/accounts", accounts::route()?)
        .nest("/credentials", credentials::route()?)
        .nest("/projects", projects::route()?)
    )
}
