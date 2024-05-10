# rust-template-rs

Here are a few templates for use when creating [Rust] applications.

The templates take full advantage of `cargo-generate` subtemplates, thust the repo contains both
project templates and several smaller templates that are used to add functionality to an already
existing project.

## Project templates

Project templates have optional support for generating IDE scaffolding (currently only Vscode,
contributions for others are welcome).

### Workspace

Simple setup for a base workspace, ready to put your projects in.

### Cli

Binary with a command line (`clap`) interface.

Template is setup to use:
- [`clap`]
- [`tokio`] (Optional)
- [`tracing`]
- [`console-subscriber`], by using `tokio` and enabling the feature [`tokio-console`] (Optional)
- Unit tests, with optional [`tokio-test`] support

### Lib

Library crate.

The library will be setup for use with:
- Support for `no-std`, by disabling the default feature `std`
- [`tracing`]
- Unit tests, with [`tokio-test`] support
- ITests, with [`tokio-test`] support

## Snippets

Several templates are included that function as snippets that integrate into in existing project.

Just try and expand the template repository and let `cargo-generate` guide you.

## Tips'n'tricks

If the template is used on a regular basis, [cargo-generate] allows to setup favorite templates and default variables.

To do this, open or create the file `$CARGO_HOME/cargo-generate.toml`, insert this:

```toml
[values]
gh_username = "your username on github.com"
ide = "none|vscode"

[favorites.rust]
git = "https://github.com/taurr/rust-template-rs"
subfolder = "Projects"

[favorites.snippet]
git = "https://github.com/taurr/rust-template-rs"
subfolder = "Snippets"
```

After this, the template can be expanded using a simple:

```shell
cargo generate rust
```

or:

```shell
cargo generate snippet
```

[Rust]:https://www.rust-lang.org
[cargo-generate]:https://github.com/cargo-generate/cargo-generate
[`clap`]:https://crates.io/crates/clap
[`tokio`]:https://crates.io/crates/tokio
[`tracing`]:https://crates.io/crates/tracing
[`console-subscriber`]:https://crates.io/crates/console-subscriber
[`tokio-test`]:https://crates.io/crates/tokio-test
[`tokio-console`]:https://github.com/tokio-rs/console