use dioxus::prelude::*;

mod app;
mod components;
mod pages;
mod route;

pub use app::App;
pub use route::Route;

fn main() {
    dioxus::logger::init(tracing::Level::INFO).expect("failed to init logger");

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    serve(App);
}

#[cfg(feature = "server")]
fn serve(app: fn() -> Element) {
    let dioxus_router = dioxus::server::router(app);

    #[cfg(not(feature = "lambda"))]
    dioxus::serve(move || {
        let app = dioxus_router.clone();
        async move { Ok(app) }
    });

    #[cfg(feature = "lambda")]
    {
        let app_future = async move { lambda_http::run(dioxus_router).await };

        tracing::info!("Starting server in Lambda environment");
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            let _ = handle.block_on(app_future);
        } else {
            let _ = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(app_future);
        }
    }
}
