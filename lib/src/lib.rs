#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(test, feature = "std")), no_std)]

use tracing::{debug, error, info, instrument, trace, warn};

/// TODO: We should have proper comments for all public fn.
#[instrument]
pub fn find_lines<'a>(
    contents: &'a str,
    pattern: &'a str,
) -> impl Iterator<Item = (usize, &'a str)> + 'a {
    trace!("trace example!");
    debug!("debug example!");
    info!("info  example!");
    warn!("warn  example!");
    error!("error  example!");
    contents
        .lines()
        .enumerate()
        .filter(move |(_line_no, line)| line.contains(pattern))
        .map(|(line_no, line)| (line_no + 1, line))
}

#[cfg(test)]
mod tests {
    //! Consider testing the public API as an ITest to illustrate usecase scenarios.
    use super::*;
    use tokio::test;

    type Result = anyhow::Result<()>;

    #[test]
    async fn it_works() -> Result {
        tracing_subscriber::fmt::init();
        assert_eq!(
            vec![(2usize, "Hello World")],
            find_lines("Dummy\nHello World\nline", "World").collect::<Vec<_>>()
        );
        Ok(())
    }
}
