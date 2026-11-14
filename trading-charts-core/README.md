[![Crates.io]](https://crates.io/crates/trading-charts-core)
[![Docs.rs]](https://docs.rs/trading-charts-core/)

# Trading-Charts core
Framework-agnostic core of the Rust bindings of [Lightweight Charts] 5.3:
the chart handle, the data and option types, and the JS binding.

The [Leptos](https://crates.io/crates/trading-charts) and [Dioxus](https://crates.io/crates/trading-charts-dioxus)
adapters are built on this crate and re-export it: use them in an application of these frameworks.
Use this crate directly to drive charts without a framework, or to write an adapter for another framework.

## Chart handle

A [`ChartHandle`] drives a chart. It is cheap to clone, every clone driving the same chart,
which is destroyed when the last clone is dropped.
Series, watermarks, pane settings, event subscriptions and the legend can be set up before the chart is bound
to a DOM node: they are created when it is bound, and again if it is bound to another node.

```rust
use trading_charts_core::{
    data::{options::ChartOptions, Candlestick, Marker, RangePreset, UTCTimestamp},
    ChartHandle,
    JsError,
};
use web_sys::HtmlDivElement;

fn show(node: HtmlDivElement, candles: Vec<Candlestick>) -> Result<ChartHandle, JsError> {
    let chart = ChartHandle::new(Some(&ChartOptions::new().with_auto_size(true)))?;
    let series = chart.add_candlestick_series(candles, None, None)?;

    series.set_markers(&[Marker::buy("2024-01-02".parse::<UTCTimestamp>()?)])?;
    chart.bind(node)?;
    chart.show_range(RangePreset::Months(3))?;

    Ok(chart)
}
```

The handle methods return a [`JsError`] when the JS side fails; the adapters log these errors on the console.

## Data and options

- The data types (`Candlestick`, `ValueData`, `HistogramData`, `OhlcvData`) take [`UTCTimestamp`] times,
  which parse RFC 3339 strings and dates and convert from `chrono` dates.
  The `whitespace` constructors create items without value, which make gaps on the time scale.
- The option types mirror the options of Lightweight Charts, with their documentation and default values.
  Every option is optional and only sent when set, so applying options only changes the options which are set.
  They are built with `with_*` builders: `ChartOptions::new().with_auto_size(true)`.
- The [`SeriesHandle`] returned when a series is added updates its data, options, markers and price lines.

## Formatters

Lightweight Charts takes JS functions to format prices and times. The `data::format` module describes them in Rust:

- `PriceFormatter`, `TimeFormatter` and `TickMarkFormatter` are predefined behaviors implemented by the JS binding:
  currencies, localized and compact numbers, date patterns, localized date styles...
- Rust closures are registered under a name with `register_price_formatter`, `register_time_formatter` and
  `register_tick_mark_formatter`, then used with the `custom` formatters.
- `DateFormat` and `FontFamily` give predefined values and accept custom ones; `load_web_font` loads a web font.

## Other features

- **Markers**: `register_marker_style` registers a style used by the markers of type `MarkerType::Custom(id)`.
  The styles are global to the page: prefixed identifiers group them, to list them (`marker_style_ids_with_prefix`),
  get them (`marker_style`, `marker_styles_with_prefix`) or unregister them (`unregister_marker_styles_with_prefix`),
  and `clear_marker_styles` restores the built-in styles only.
- **Events**: `ChartHandle::on_crosshair_move`, `on_click`... call Rust callbacks with the event parameters,
  until the returned `Subscription` is dropped.
- **Time zones**: `ChartHandle::set_time_zone` displays the times in an IANA time zone,
  the data and the times exchanged with the handle staying UTC times.
- **Synchronization**: `ChartHandle::set_sync_group` shares the crosshair and the visible range between charts.
- **Ranges and intervals**: `ChartHandle::show_range` shows a `RangePreset`,
  and the `data::resample` module resamples data to a larger `Interval`.
- **Tooltips**: `TooltipPlacement` computes where a tooltip goes, for the overlays of the adapters.
- **Screenshots**: `ChartHandle::take_screenshot`, `take_screenshot_canvas` and `download_screenshot`.

## JS binding

The `bindings` directory is an npm package, bundled with esbuild by the build script of the crate,
which requires [Node.js](https://nodejs.org/) and npm.
The bundle, which includes Lightweight Charts, is inlined in the WebAssembly module by the
[`wasmbind-js-file-macro`](https://crates.io/crates/wasmbind-js-file-macro) macro: there is no JS file to serve.

The `TradingChart` class of the binding (`bindings/src/chart.mjs`) is the JS side of `ChartHandle`. The other modules
handle the series, the legend, the markers, the watermarks, the formatters, the time zones, the events, the
synchronization of the charts and the web fonts.

[Crates.io]: https://img.shields.io/crates/v/trading-charts-core?style=for-the-badge
[Docs.rs]: https://img.shields.io/docsrs/trading-charts-core?style=for-the-badge
[Lightweight Charts]: https://github.com/tradingview/lightweight-charts
[`ChartHandle`]: https://docs.rs/trading-charts-core/latest/trading_charts_core/struct.ChartHandle.html
[`SeriesHandle`]: https://docs.rs/trading-charts-core/latest/trading_charts_core/struct.SeriesHandle.html
[`JsError`]: https://docs.rs/trading-charts-core/latest/trading_charts_core/struct.JsError.html
[`UTCTimestamp`]: https://docs.rs/trading-charts-core/latest/trading_charts_core/data/struct.UTCTimestamp.html
