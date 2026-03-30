#![allow(unused_imports)]
#[cfg(feature = "server")]
pub mod auth;
pub mod config;
mod logger;
pub mod macros;
mod run;
pub mod types;
pub mod utils;

pub mod components;

#[cfg(feature = "server")]
pub mod middlewares;
#[cfg(feature = "server")]
pub mod models;

#[cfg(feature = "server")]
pub use auth::ProjectAuth;
pub use config::CommonConfig;
#[cfg(feature = "server")]
pub use macros::DynamoEntity;
pub use macros::DynamoEnum;
pub use run::run;
pub use types::*;

pub use dioxus::logger::tracing::{debug, error, info, warn};
pub use dioxus::prelude::*;
pub use serde::{Deserialize, Serialize};
pub use serde_with::{DeserializeFromStr, SerializeDisplay};

// Server-only re-exports
#[cfg(feature = "server")]
pub use aws_sdk_dynamodb;
#[cfg(feature = "server")]
pub use base64;
#[cfg(feature = "server")]
pub use dioxus::fullstack::axum::extract::Extension;
#[cfg(feature = "server")]
pub use serde_dynamo;

pub type Result<T, E = Error> = std::result::Result<T, E>;
