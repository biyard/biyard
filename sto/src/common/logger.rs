#[cfg(feature = "server")]
pub fn init() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .try_init();
}

#[cfg(not(feature = "server"))]
pub fn init() {
    tracing_wasm::set_as_global_default();
}
