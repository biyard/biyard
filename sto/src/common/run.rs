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
    dioxus::serve(move || {
        let app = dioxus_router.clone();
        async move { Ok(app) }
    });
}
