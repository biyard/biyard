use std::sync::Arc;

use aws_config::Region;
use aws_sdk_dynamodb::{Config, config::Credentials};

use crate::config;

#[derive(Clone)]
pub struct AppState {
    pub cli: aws_sdk_dynamodb::Client,
}

impl AppState {
    pub fn new(conf: &config::Config) -> Self {
        let mut builder = Config::builder()
            .credentials_provider(
                Credentials::builder()
                    .access_key_id(conf.aws.access_key_id)
                    .secret_access_key(conf.aws.secret_access_key)
                    .provider_name("biyard")
                    .build(),
            )
            .region(Region::new(conf.aws.region))
            .behavior_version_latest();

        if let Some(endpoint) = conf.dynamo.endpoint {
            builder = builder.endpoint_url(endpoint.to_string());
        }
        let aws_config = builder.build();

        let cli = aws_sdk_dynamodb::Client::from_conf(aws_config);

        AppState { cli }
    }
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}
