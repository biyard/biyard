use crate::common::{CommonConfig, Deserialize, Result, Serialize};
use crate::features::contacts::Update;
use dioxus::prelude::post;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowUpdatesResponse {
    pub email: String,
}

#[post("/v1/landing/updates")]
pub async fn follow_updates_handler(email: String) -> Result<FollowUpdatesResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let u = Update::new(email);
    u.create(cli).await?;

    Ok(FollowUpdatesResponse { email: u.email })
}
