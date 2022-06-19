# rust-template-rs

Here are a few opinionated `cargo-generate` templates for use when creating [Rust] applications.

## templates

### cli

Binary with a command line interface.

Template is setup to use:
- [`clap`]
- [`tokio`] (Optional)
- [`tracing`]
- [`console-subscriber`], by enabling the feature [`tokio-console`]  (Optional)
- Unit tests, with optional [`tokio-test`] support

### lib

Library crate.

The library will be setup for use with:
- Support for `no-std`, by disabling the default feature `std`
- [`tracing`]
- Unit tests, with [`tokio-test`] support
- ITests, with [`tokio-test`] support

## Tips'n'tricks

If the template is used on a regular basis, [cargo-generate] allows to setup favorite templates and default variables.

To do this, open or create the file `$CARGO_HOME/cargo-generate.toml`, insert this:
```toml
[favorites.rust]
git = "https://github.com/taurr/rust-template-rs"
```

After this, the template can be expanded using a simple:

```shell
cargo generate rust
```

[Rust]:https://www.rust-lang.org
[cargo-generate]:https://github.com/cargo-generate/cargo-generate
[`clap`]:https://crates.io/crates/clap
[`tokio`]:https://crates.io/crates/tokio
[`tracing`]:https://crates.io/crates/tracing
[`console-subscriber`]:https://crates.io/crates/console-subscriber
[`tokio-test`]:https://crates.io/crates/tokio-test
[`tokio-console`]:https://github.com/tokio-rs/console