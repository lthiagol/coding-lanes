# coding-lanes

A Cargo workspace for tracking work across coding lanes, with two frontends
sharing one domain model.

## Layout

```
crates/
  core/   # shared domain model + JSON persistence (coding-lanes-core)
  cli/    # command line frontend        (binary: coding-lanes)
  tui/    # terminal UI frontend         (binary: coding-lanes-tui)
```

## Toolchain

Rust is pinned via `rust-toolchain.toml` (stable, minimal profile, with
`rustfmt` and `clippy`). Install with [rustup](https://rustup.rs):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Build and test

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all
```

## Usage

```sh
# CLI
cargo run -p coding-lanes-cli -- add "implement the parser"
cargo run -p coding-lanes-cli -- list
cargo run -p coding-lanes-cli -- done 1
cargo run -p coding-lanes-cli -- --store /tmp/tasks.json list

# TUI
cargo run -p coding-lanes-tui
```

Tasks are persisted as JSON in `$XDG_DATA_HOME/coding-lanes/tasks.json`
(default `~/.local/share/coding-lanes/tasks.json`).

## License

MIT OR Apache-2.0
