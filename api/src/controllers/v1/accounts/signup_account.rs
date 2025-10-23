use crate::features::accounts::*;
use crate::*;

pub async fn signup_account_handler(
    State(AppState { .. }): State<AppState>,
    Json(_req): Json<SignupAccountRequest>,
) -> Result<Json<SignupAccountResponse>> {
    debug!("Handling request: {:?}", _req);
    // TODO: Implement the handler logic here

    unimplemented!()
}
