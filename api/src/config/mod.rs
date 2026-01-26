use crate::*;
use aws_config::AwsConfig;
use dynamo_config::DynamoConfig;

pub mod aws_config;
pub mod dynamo_config;

#[derive(Debug)]
pub struct PageConfig {
    pub index_js: &'static str,
    pub index_css: &'static str,
}
#[derive(Debug)]
pub struct Config {
    pub env: Env,
    pub aws: AwsConfig,
    pub dynamo: DynamoConfig,
    pub domain: &'static str,
    pub landing: PageConfig,
    pub console: PageConfig,
    pub web_build: bool,
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
            web_build: option_env!("WEB_BUILD")
                .unwrap_or("true")
                .parse()
                .unwrap_or(true),
            domain: option_env!("DOMAIN").unwrap_or("dev.biyard.co"),
            landing: PageConfig {
                index_js: option_env!("LANDING_INDEX_JS").unwrap_or("index.js"),
                index_css: option_env!("LANDING_INDEX_CSS").unwrap_or("index.css"),
            },
            console: PageConfig {
                index_js: option_env!("CONSOLE_INDEX_JS").unwrap_or("index.js"),
                index_css: option_env!("CONSOLE_INDEX_CSS").unwrap_or("index.css"),
            },
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
    PartialEq,
    Eq,
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
