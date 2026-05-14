//! Shared integration test harness (Ratel-style oneshot).
//!
//! Instead of binding a real TCP listener and running `axum::serve`, we
//! hold the `axum::Router` in a `OnceLock` and dispatch each request
//! with `tower::ServiceExt::oneshot`. This removes per-test TCP setup
//! cost and makes tests independent of any port availability.

#![allow(dead_code)]

pub mod factories;
pub mod http;
pub mod tables;

use std::sync::OnceLock;
use tokio::sync::OnceCell;

pub struct TestServer {
    router: axum::Router,
}

impl TestServer {
    pub fn client(&self) -> http::Client {
        http::Client::new(self.router.clone())
    }
}

static SERVER: OnceCell<TestServer> = OnceCell::const_new();

pub async fn test_server() -> &'static TestServer {
    SERVER
        .get_or_init(|| async {
            ensure_tables_once().await;
            let router = console::common::build_router(console::App);
            TestServer { router }
        })
        .await
}

static TABLES_READY: OnceLock<()> = OnceLock::new();

async fn ensure_tables_once() {
    if TABLES_READY.get().is_some() {
        return;
    }
    tables::create_main_table().await;
    let _ = TABLES_READY.set(());
}
