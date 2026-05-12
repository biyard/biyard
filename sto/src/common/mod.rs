pub mod types;
mod logger;
mod run;

#[cfg(feature = "server")]
mod config;
#[cfg(feature = "server")]
pub use config::CommonConfig;

pub use run::run;
pub use types::*;

pub use dioxus::logger::tracing::{debug, error, info, warn};
pub use dioxus::prelude::*;
pub use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
pub use aws_sdk_dynamodb;
#[cfg(feature = "server")]
pub use serde_dynamo;

pub type Result<T, E = Error> = std::result::Result<T, E>;
