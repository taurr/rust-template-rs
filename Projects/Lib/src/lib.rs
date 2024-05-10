#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[allow(unused)]
use tracing::{debug, error, info, instrument, warn};

/// TODO: We should have proper comments for all public fn.
#[instrument]
pub {% if tokio %}async {% endif %}fn find_lines<'a>(
    contents: &'a str,
    pattern: &'a str,
) -> impl Iterator<Item = (usize, &'a str)> + 'a {
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
    //! Leaving the private functions for these tests
    use super::*;
    use test_log::test;

    {% if tokio -%}
    #[test(tokio::test)]
    async fn it_works() {
        assert_eq!(
            vec![(2usize, "Hello World")],
            find_lines("Dummy\nHello World\nline", "World")
            .await
            .collect::<Vec<_>>()
        );
    }{% else -%}
    #[test]
    fn it_works() {
        assert_eq!(
            vec![(2usize, "Hello World")],
            find_lines("Dummy\nHello World\nline", "World")
            .collect::<Vec<_>>()
        );
    }{% endif %}
}
