use anyhow::Result;
use clap::StructOpt;
use tracing::info;

mod args;
mod log;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    log::setup_tracing();

    let args = args::Args::parse();

    for _ in 0..args.count {
        info!(name=?args.name);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use tokio::test;

    #[test]
    async fn dummy() -> Result<()> {
        crate::log::setup_tracing();
        tracing::warn!("No actual unit tests yet");
        assert_eq!(4, 3+1);
    }
}