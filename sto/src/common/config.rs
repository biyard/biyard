//! Server-only — DynamoDB 클라이언트 + 환경 설정.
#![cfg(feature = "server")]

use aws_sdk_dynamodb::Client as DynamoClient;

pub struct CommonConfig {
    pub table: String,
    pub endpoint: Option<String>,
    pub region: String,
}

impl Default for CommonConfig {
    fn default() -> Self {
        Self {
            table: std::env::var("DYNAMO_TABLE")
                .unwrap_or_else(|_| "biyard-local-sto".to_string()),
            endpoint: std::env::var("DYNAMO_ENDPOINT").ok(),
            region: std::env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
        }
    }
}

impl CommonConfig {
    pub async fn dynamodb(&self) -> DynamoClient {
        let mut loader = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(aws_config::Region::new(self.region.clone()));

        if let Some(ref endpoint) = self.endpoint {
            loader = loader.endpoint_url(endpoint.clone());
        }

        // localstack 용 dummy credentials
        if self.endpoint.is_some() {
            loader = loader.credentials_provider(
                aws_credential_types::Credentials::new("test", "test", None, None, "static"),
            );
        }

        let config = loader.load().await;
        DynamoClient::new(&config)
    }
}
