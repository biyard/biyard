use super::*;

#[derive(Debug, Clone, Copy)]
pub struct WebConfig {
    pub env: Environment,
    pub log_level: LogLevel,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            env: Default::default(),
            log_level: Default::default(),
        }
    }
}
