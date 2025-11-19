use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct FollowUpdatesRequest {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct FollowUpdatesResponse {
    pub email: String,
}

pub async fn follow_updates_handler(
    State(AppState { cli, .. }): State<AppState>,
    Json(req): Json<FollowUpdatesRequest>,
) -> Result<Json<FollowUpdatesResponse>> {
    tracing::debug!("Handling request: {:?}", req);

    let u = Update::new(req.email);
    u.create(&cli).await?;

    Ok(Json(FollowUpdatesResponse { email: u.email }))
}
