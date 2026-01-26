mod contacts;
mod updates;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .nest("/contacts", contacts::route()?)
        .nest("/updates", updates::route()?))
}
