//! Server-only — Lazy<DynamoClient> + 환경 설정. console 패턴 동일.
#![cfg(feature = "server")]

use aws_sdk_dynamodb::{
    Client,
    config::{Credentials, Region},
};
use dioxus::fullstack::Lazy;

pub static DB: Lazy<Client> = Lazy::new(|| async move {
    let endpoint = match option_env!("DYNAMO_ENDPOINT") {
        Some(ep) if ep.to_lowercase() == "none" || ep.is_empty() => None,
        Some(ep) => Some(ep.to_string()),
        None => None,
    };

    let region = option_env!("AWS_REGION")
        .unwrap_or("ap-northeast-2")
        .to_string();

    let mut builder = aws_sdk_dynamodb::Config::builder()
        .region(Region::new(region))
        .behavior_version_latest()
        .credentials_provider(Credentials::new(
            option_env!("AWS_ACCESS_KEY_ID").unwrap_or("test"),
            option_env!("AWS_SECRET_ACCESS_KEY").unwrap_or("test"),
            None,
            None,
            "loaded-from-env",
        ));

    if let Some(ep) = endpoint {
        builder = builder.endpoint_url(ep);
    }

    dioxus::Ok(Client::from_conf(builder.build()))
});

pub struct CommonConfig {
    pub table: String,
}

impl Default for CommonConfig {
    fn default() -> Self {
        let prefix = option_env!("DYNAMO_TABLE_PREFIX").unwrap_or("biyard-local");
        Self {
            table: format!("{prefix}-sto"),
        }
    }
}

impl CommonConfig {
    /// 동기 함수 — static Lazy 참조 반환.
    pub fn dynamodb(&self) -> &'static Client {
        &DB
    }
}
