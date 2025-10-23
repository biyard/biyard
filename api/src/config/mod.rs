use crate::*;
use aws_config::AwsConfig;
use dynamo_config::DynamoConfig;

pub mod aws_config;
pub mod dynamo_config;

#[derive(Debug)]
pub struct Config {
    pub env: Env,
    pub aws: AwsConfig,
    pub dynamo: DynamoConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV")
                .unwrap_or("local")
                .parse()
                .expect("env: dev, local, prod"),
            aws: AwsConfig::default(),
            dynamo: DynamoConfig::default(),
        }
    }
}

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub enum Env {
    #[default]
    Local,
    Dev,
    Prod,
}

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        &CONFIG.as_ref().unwrap()
    }
}
