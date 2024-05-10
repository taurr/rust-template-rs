#![doc = include_str!("../README.md")]

use clap::Parser;
#[allow(unused)]
use tracing::{error, info, instrument, warn};

mod args;

{% if tokio -%}
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    setup_tracing();
    let args = args::Args::parse();

    for count in 0..args.count {
        info!(count, name=?args.name);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
{% else -%}
fn main() {
    tracing_subscriber::fmt::init();
    let args = args::Args::parse();

    for _ in 0..args.count {
        info!(name=?args.name);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
{% endif -%}

{% if tokio %}
#[cfg(not(feature = "tokio-console"))]
fn setup_tracing() {
    tracing_subscriber::fmt::init();
}

#[cfg(feature = "tokio-console")]
fn setup_tracing() {
    use tracing_subscriber::prelude::*;
    tracing_subscriber::registry()
        .with(console_subscriber::spawn())
        .with(tracing_subscriber::fmt::layer())
        .init();
}
{% endif -%}
