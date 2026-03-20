[![Crates.io]](https://crates.io/crates/trading-charts)
[![Docs.rs]](https://docs.rs/trading-charts/)

# Trading-Charts
Rust bindings of [Lightweight Charts] with shared core support for [Leptos] and [Dioxus].

## Workspace

- `trading-charts-core`: shared `wasm-bindgen` runtime, JS bridge, data types, and series options.
- `trading-charts`: Leptos adapter crate.
- `trading-charts-dioxus`: Dioxus web adapter crate.
- `example`: Leptos example.
- `example-dioxus`: Dioxus web example.

## Example
The workspace includes side-by-side examples for the Leptos and Dioxus adapters.

[Crates.io]: https://img.shields.io/crates/v/trading-charts?style=for-the-badge
[Docs.rs]: https://img.shields.io/docsrs/trading-charts?style=for-the-badge
[Lightweight Charts]: https://github.com/tradingview/lightweight-charts
[Leptos]: https://leptos.dev/
[Dioxus]: https://dioxuslabs.com/
