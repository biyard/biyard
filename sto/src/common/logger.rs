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
    // 브라우저는 RUST_LOG 환경변수를 못 읽으니 max_level 을 빌드 타임에 결정.
    // debug 빌드는 DEBUG, release 는 INFO. 디폴트(TRACE)는 dioxus-core 내부
    // 로그까지 콘솔에 쏟아져 사용 불가능 수준이라 무조건 낮춰서 박는다.
    let max_level = if cfg!(debug_assertions) {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };
    let config = tracing_wasm::WASMLayerConfigBuilder::new()
        .set_max_level(max_level)
        .build();
    tracing_wasm::set_as_global_default_with_config(config);
}
