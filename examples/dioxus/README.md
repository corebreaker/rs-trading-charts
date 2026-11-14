[![Crates.io]](https://crates.io/crates/trading-charts-dioxus)
[![Docs.rs]](https://docs.rs/trading-charts-dioxus/)

# Dioxus example of Trading Charts

A gallery of demos of the [`trading-charts-dioxus`](../../trading-charts-dioxus) crate, the [Dioxus] web components
for [Lightweight Charts], one tab per group of features:

- **Series**: the six series types, markers (built-in, custom and price-positioned), price lines, whitespace data.
- **Price & volume**: the price and volume component, with the price style, the bar interval and range presets.
- **Scales**: two visible price scales, the price scale modes, an inverted scale and a hidden time scale.
- **Tooltips & legends**: tooltips rendered by Dioxus components, the built-in legends and a custom legend.
- **Customization**: time zones, locales, date formats, price formatters (including a Rust closure), fonts and
  crosshair modes, on real 5-minute data.
- **Panes & watermarks**: several panes, text and image watermarks, pane swapping and screenshots.
- **Multi-charts**: two charts sharing their crosshair and visible range.

The [Leptos example](../leptos) is the same gallery written with the Leptos components.

## Run

Install [Trunk](https://trunkrs.dev/) and the WebAssembly target:

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
```

Then serve the example from the workspace root with [cargo-make](https://github.com/sagiegurari/cargo-make):

```bash
cargo make serve-example-dioxus
```

or from this directory with `trunk serve`, and open <http://127.0.0.1:1422>.

[![Screenshot]][Screenshot]

[Crates.io]: https://img.shields.io/crates/v/trading-charts-dioxus?style=for-the-badge
[Docs.rs]: https://img.shields.io/docsrs/trading-charts-dioxus?style=for-the-badge
[Lightweight Charts]: https://github.com/tradingview/lightweight-charts
[Dioxus]: https://dioxuslabs.com/
[Screenshot]: assets/example.png
