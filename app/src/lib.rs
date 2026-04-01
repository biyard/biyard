mod app;
pub mod common;
pub mod features;
mod layout;
mod route;

pub use app::App;
pub use route::Route;

// Re-export common types at crate root (required by DynamoEntity macro which references crate::Error)
pub use common::types::Error;

extern crate dioxus_fullstack;
#[cfg(feature = "server")]
extern crate dioxus_server;
