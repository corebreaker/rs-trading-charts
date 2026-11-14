[![Crates.io]](https://crates.io/crates/trading-charts)
[![Docs.rs]](https://docs.rs/trading-charts/)

# Trading-Charts
Rust bindings of [Lightweight Charts] with shared core support for [Leptos] and [Dioxus].

## Workspace

- `trading-charts-core`: shared `wasm-bindgen` runtime, JS bridge, data types, and series options.
- `trading-charts`: Leptos adapter crate.
- `trading-charts-dioxus`: Dioxus web adapter crate.
- `examples/leptos`: Leptos example.
- `examples/dioxus`: Dioxus web example.

## Example
The workspace includes side-by-side examples for the Leptos and Dioxus adapters.

## Development

The workspace tasks are run with [cargo-make](https://github.com/sagiegurari/cargo-make)
(`cargo install cargo-make`).
Run `cargo make` from the workspace root to list them, for instance:

- `cargo make fmt`: format the workspace (requires the nightly toolchain).
- `cargo make check-wasm`: check the workspace on the `wasm32-unknown-unknown` target.
- `cargo make serve-example` / `cargo make serve-example-dioxus`: serve an example with Trunk.
- `cargo make verify`: format, check and build both examples.

## Updating Lightweight Charts

Run `cargo make update-lightweight-charts` from the workspace root to refresh the binding package to the latest npm release
and update its lockfile.

Run `cargo make update-lightweight-charts -e LIGHTWEIGHT_CHARTS_VERSION=5.3.0` to pin a specific upstream version instead.

[Crates.io]: https://img.shields.io/crates/v/trading-charts?style=for-the-badge
[Docs.rs]: https://img.shields.io/docsrs/trading-charts?style=for-the-badge
[Lightweight Charts]: https://github.com/tradingview/lightweight-charts
[Leptos]: https://leptos.dev/
[Dioxus]: https://dioxuslabs.com/
