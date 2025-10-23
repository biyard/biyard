use std::env;

#[derive(Debug)]
pub struct AwsConfig {
    pub region: &'static str,
    pub access_key_id: &'static str,
    pub secret_access_key: &'static str,
}

impl Default for AwsConfig {
    fn default() -> Self {
        let region = env::var("REGION").unwrap_or_else(|_| {
            option_env!("AWS_REGION")
                .unwrap_or("ap_northeast-2")
                .to_string()
        });

        AwsConfig {
            region: Box::leak(region.into_boxed_str()),
            access_key_id: option_env!("AWS_ACCESS_KEY_ID").unwrap_or("test"),
            secret_access_key: option_env!("AWS_SECRET_ACCESS_KEY").unwrap_or("test"),
        }
    }
}
