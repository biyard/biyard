use aws_sdk_s3::{
    Client, Config,
    config::{Credentials, Region},
};

use super::AwsConfig;
use crate::common::utils::s3_client::S3Client;
use dioxus::fullstack::Lazy;

pub static S3: Lazy<S3Client> = Lazy::new(|| async move {
    let aws_config = AwsConfig::default();
    let s3_config = S3Config::default();

    let builder = Config::builder()
        .region(Region::new(aws_config.region))
        .behavior_version_latest()
        .credentials_provider(Credentials::new(
            aws_config.access_key_id,
            aws_config.secret_access_key,
            None,
            None,
            "loaded-from-config",
        ));

    let client = Client::from_conf(builder.build());

    dioxus::Ok(S3Client::new(
        client,
        s3_config.bucket_name,
        s3_config.public_url,
        s3_config.expire,
    ))
});

pub struct S3Config {
    pub bucket_name: String,
    /// Public CloudFront URL prefix used to construct download URLs.
    /// e.g., "https://meta.biyard.co".
    pub public_url: String,
    /// Presigned URL expiration in seconds.
    pub expire: u64,
}

impl Default for S3Config {
    fn default() -> Self {
        let bucket_name = option_env!("BUCKET_NAME")
            .unwrap_or("meta.biyard.co")
            .to_string();
        let public_url = option_env!("META_PUBLIC_URL")
            .unwrap_or("https://meta.biyard.co")
            .to_string();
        let expire = option_env!("BUCKET_EXPIRE")
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(3600);

        S3Config {
            bucket_name,
            public_url,
            expire,
        }
    }
}
