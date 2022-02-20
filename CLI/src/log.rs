#[cfg(not(feature = "tokio-console"))]
pub fn setup_tracing() {
    tracing_subscriber::fmt::init();
}

#[cfg(feature = "tokio-console")]
pub fn setup_tracing() {
    use tracing_subscriber::prelude::*;
    tracing_subscriber::registry()
        .with(console_subscriber::spawn())
        .with(tracing_subscriber::fmt::layer())
        .init();
}
