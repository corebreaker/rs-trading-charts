[![Crates.io]](https://crates.io/crates/trading-charts)
[![Docs.rs]](https://docs.rs/trading-charts/)

# Trading-Charts for Leptos
[Leptos] components for [Lightweight Charts] 5.3, the financial charts library of TradingView.

The crate is built on [`trading-charts-core`](https://crates.io/crates/trading-charts-core), which it re-exports:
`trading_charts::data` holds the data and option types, `trading_charts::series_data` the series options,
and `ChartHandle` drives a chart imperatively.

## Components

| Component                                                | Module                 | Role                                          |
|----------------------------------------------------------|------------------------|-----------------------------------------------|
| `Chart`                                                  | `chart`                | The chart, which renders its children.        |
| `ChartPanel`                                             | `panel`                | A pane, with its series and watermarks.       |
| `CandleStickSeries`, `BarSeries`                         | `series::*`            | OHLC series.                                  |
| `LineSeries`, `AreaSeries`, `BaselineSeries`             | `series::*`            | Single-value series.                          |
| `HistogramSeries`                                        | `series::histograms`   | Histogram series.                             |
| `PriceVolumeSeries`                                      | `series::price_volume` | Prices and volumes on the same chart.         |
| `Tooltip`                                                | `overlay`              | A tooltip following the crosshair.            |
| `ChartOverlay`                                           | `overlay`              | An element over the chart, for custom legends.|
| `TextWatermark`, `ImageWatermark`                        | `watermark`            | Watermarks of a pane.                         |

The props of the components are reactive: they take values or signals, and the chart follows the signals.
The series are removed from the chart when their component is unmounted.

## Example

```rust
use leptos::prelude::*;
use trading_charts::{
    chart::{use_crosshair, Chart},
    data::{options::ChartOptions, Candlestick, LegendOptions, TooltipMode},
    overlay::Tooltip,
    panel::ChartPanel,
    series::candlesticks::CandleStickSeries,
};

#[component]
fn PriceChart(#[prop(into)] data: Signal<Vec<Candlestick>>) -> impl IntoView {
    view! {
        <Chart
            options=ChartOptions::new().with_auto_size(true)
            legend=Some(LegendOptions::new().with_text("ACME"))
            time_zone="Europe/Paris"
            style="height:400px"
        >
            <ChartPanel>
                <CandleStickSeries data=data />
            </ChartPanel>
            <Tooltip mode=TooltipMode::Floating>
                <Price />
            </Tooltip>
        </Chart>
    }
}

/// Tooltip content: the close price under the cursor.
#[component]
fn Price() -> impl IntoView {
    let crosshair = use_crosshair();

    move || {
        crosshair.with(|params| {
            params
                .as_ref()
                .and_then(|params| params.main_series())
                .and_then(|series| series.data.price())
                .map(|price| format!("{price:.2}"))
        })
    }
}
```

## Chart

The chart fills a `div` sized by its `style` and `class` props, which is also the positioning context of the
overlays. Set the `autoSize` option for the chart to follow the size of the `div`.

Its props give the options, the built-in legend, the time zone, the synchronization group of the chart
(the charts of a group share their crosshair and visible range) and the event callbacks:
`on_crosshair_move`, `on_click`, `on_dbl_click`, `on_visible_time_range_change` and
`on_visible_logical_range_change`. The `on_ready` callback gives the `ChartHandle`, to control the chart from outside:
show a range, take a screenshot, move the crosshair...

The children of a chart get the chart handle with `use_chart` and the crosshair state with `use_crosshair`.
The `refit` prop takes [emitix](https://crates.io/crates/emitix) event channels on which a `REFIT_EVENT_KIND` event
scales the time axis to fit the data.

## Panes

The first `ChartPanel` is the main pane, the next ones are stacked below it; the series outside of a panel are drawn
in the main pane. The `options` prop of a panel sets its stretch factor, height and preservation when empty.

[Crates.io]: https://img.shields.io/crates/v/trading-charts?style=for-the-badge
[Docs.rs]: https://img.shields.io/docsrs/trading-charts?style=for-the-badge
[Lightweight Charts]: https://github.com/tradingview/lightweight-charts
[Leptos]: https://leptos.dev/
