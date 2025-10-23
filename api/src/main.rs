use api::{api_main::api_main, *};
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let app = api_main().await?;

    let port = env::var("PORT").unwrap_or("3000".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}
