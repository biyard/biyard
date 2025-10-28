use tower_sessions::{
    SessionManagerLayer,
    cookie::time::{Duration, OffsetDateTime},
};

use crate::{config::Env, features::session::DynamoSessionStore};

pub fn session_manage_layer(
    cli: aws_sdk_dynamodb::Client,
    conf: &crate::config::Config,
) -> SessionManagerLayer<DynamoSessionStore> {
    let session_store = DynamoSessionStore::new(cli);
    let is_local = conf.env == Env::Local;

    SessionManagerLayer::new(session_store)
        .with_secure(!is_local)
        .with_http_only(!is_local)
        .with_same_site(if is_local {
            tower_sessions::cookie::SameSite::Lax
        } else {
            tower_sessions::cookie::SameSite::None
        })
        .with_name(format!("{}_bsid", conf.env.to_string().to_lowercase()))
        .with_path("/")
        .with_expiry(tower_sessions::Expiry::AtDateTime(
            OffsetDateTime::now_utc()
                .checked_add(Duration::days(30))
                .unwrap(),
        ))
}
