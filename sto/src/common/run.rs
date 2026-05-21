use dioxus::prelude::Element;

pub fn run(app: fn() -> Element) {
    crate::common::logger::init();

    #[cfg(not(feature = "server"))]
    dioxus::launch(app);

    #[cfg(feature = "server")]
    serve(app);
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
