#![doc = include_str!("../README.md")]

use anyhow::Result;
use clap::StructOpt;
use tracing::info;

mod args;
mod log;

use self::args::Args;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();
    log::setup_tracing();

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
        log::setup_tracing();
        tracing::warn!("No actual unit tests yet");
        assert_eq!(4, 3+1);
        Ok(())
    }
}
