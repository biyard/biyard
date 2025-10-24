#![allow(unused_imports)]
pub mod api_main;
pub mod app_state;
pub mod config;
pub mod controllers;
pub mod error;
pub mod features;
pub(crate) mod macros;
pub mod types;
pub mod utils;

pub use app_state::*;

pub type Result<T> = std::result::Result<T, error::Error>;
pub type Error = error::Error;

use aide::{NoApi, OperationIo};
use by_axum;
use by_axum::axum;
use by_axum::axum::{
    Json, Router,
    body::Body,
    extract::*,
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
};
use by_axum::*;
use by_macros::*;
use macros::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use tracing::{debug, error, info, trace, warn};
use types::*;
use utils::*;
use validator::Validate;

use by_axum::axum::routing::*;

#[cfg(test)]
pub mod tests;
