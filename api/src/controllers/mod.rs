use crate::*;

pub mod m1;
pub mod v1;

pub fn route(app_state: AppState) -> Result<Router<AppState>> {
    Ok(Router::new()
        // v1 is endpoints for users
        .nest("/v1", v1::route()?)
        // m1 is service operation admin endpoints
        .nest("/m1", m1::route()?)
        .with_state(app_state))
}
