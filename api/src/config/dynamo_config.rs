#[derive(Debug)]
pub struct DynamoConfig {
    pub endpoint: Option<&'static str>,
    pub table_prefix: &'static str,
}

impl Default for DynamoConfig {
    fn default() -> Self {
        DynamoConfig {
            endpoint: match option_env!("DYNAMO_ENDPOINT") {
                Some(endpoint) => {
                    if endpoint.is_empty() || endpoint == "none" {
                        None
                    } else {
                        Some(endpoint)
                    }
                }
                None => None,
            },
            table_prefix: option_env!("DYNAMO_TABLE_PREFIX").unwrap_or("biyard-local"),
        }
    }
}
