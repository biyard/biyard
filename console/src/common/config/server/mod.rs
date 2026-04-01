#![allow(static_mut_refs)]
pub mod dynamodb;

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct ServerConfig {
    pub env: Environment,
    pub log_level: LogLevel,
    pub aws: AwsConfig,
}

impl ServerConfig {
    pub fn dynamodb(&self) -> &aws_sdk_dynamodb::Client {
        &dynamodb::DB
    }
}

static mut CONFIG: Option<ServerConfig> = None;

impl Default for ServerConfig {
    fn default() -> Self {
        unsafe {
            if CONFIG.is_none() {
                let obj = Self {
                    env: Default::default(),
                    log_level: Default::default(),
                    aws: Default::default(),
                };

                CONFIG = Some(obj);
            }
            CONFIG.clone().unwrap()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AwsConfig {
    pub region: &'static str,
    pub access_key_id: &'static str,
    pub secret_access_key: &'static str,
}

impl Default for AwsConfig {
    fn default() -> Self {
        let region = option_env!("AWS_REGION").unwrap_or("ap-northeast-2");
        let region_str = std::env::var("REGION").unwrap_or_else(|_| region.to_string());

        AwsConfig {
            region: Box::leak(region_str.into_boxed_str()),
            access_key_id: option_env!("AWS_ACCESS_KEY_ID").unwrap_or(""),
            secret_access_key: option_env!("AWS_SECRET_ACCESS_KEY").unwrap_or(""),
        }
    }
}
