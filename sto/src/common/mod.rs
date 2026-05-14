#![allow(unused_imports)]

pub mod types;
mod logger;
mod run;

#[cfg(feature = "server")]
mod config;
#[cfg(feature = "server")]
pub use config::CommonConfig;

#[cfg(feature = "server")]
pub use by_macros::DynamoEntity;
pub use by_macros::DynamoEnum;

pub use run::run;
pub use types::*;

pub use dioxus::logger::tracing::{debug, error, info, warn};
pub use dioxus::prelude::*;
pub use serde::{Deserialize, Serialize};
pub use serde_with::{DeserializeFromStr, SerializeDisplay};

pub use dioxus_translate::{Language, Translate, use_language, use_translate};

#[cfg(feature = "server")]
pub use aws_sdk_dynamodb;
#[cfg(feature = "server")]
pub use serde_dynamo;

pub type Result<T, E = Error> = std::result::Result<T, E>;
