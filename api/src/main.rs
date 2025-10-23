use api::{api_main::api_main, *};
use by_axum::{
    axum::{self, Extension},
    cors::*,
};
use std::{env, sync::Arc};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from(option_env!("RUST_LOG").unwrap_or("info")))
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_ansi(option_env!("RUST_ANSI").unwrap_or("true") == "true")
        .try_init();

    let app = api_main().await?;

    let port = env::var("PORT").unwrap_or("3000".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    let app = app.layer(
        CorsLayer::new()
            .allow_origin(AllowOrigin::mirror_request())
            .allow_credentials(true)
            .allow_methods(AllowMethods::mirror_request())
            .allow_headers(AllowHeaders::mirror_request()),
    );

    let mut api = app.open_api;
    let state = AppState::new(&config::Config::default());
    let app = app
        .inner
        .finish_api(&mut api)
        .layer(Extension(Arc::new(api)))
        .with_state(state);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
