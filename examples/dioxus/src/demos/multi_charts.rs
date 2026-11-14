use super::{base_options, Toggle};
use crate::data::{candlesticks, closes, daily_ohlcv};

use charts::{
    chart::{use_crosshair, Chart},
    data::{SyncOptions, TooltipMode},
    overlay::Tooltip,
    series::{
        areas::{AreaSeries, AreaSeriesOptions},
        candlesticks::{CandleStickSeries, CandlestickOptions},
    },
};
use dioxus::prelude::*;
use std::rc::Rc;

const SYNC_GROUP: &str = "multi-charts-demo";

/// Magnifier tooltip content: the price of the first series under the crosshair.
#[component]
fn PriceTooltip() -> Element {
    let crosshair = use_crosshair();
    let price = crosshair
        .read()
        .as_ref()
        .and_then(|params| params.series_data.first())
        .and_then(|data| data.data.price())
        .map(|price| format!("{price:.2}"))
        .unwrap_or_default();

    rsx! { "{price}" }
}

#[component]
pub fn MultiChartsDemo() -> Element {
    let data = use_hook(|| {
        Rc::new((
            candlesticks(&daily_ohlcv(8, 250, 100.0)),
            closes(&daily_ohlcv(9, 250, 40.0)),
        ))
    });
    let crosshair = use_signal(|| true);
    let range = use_signal(|| true);
    let sync = SyncOptions::new()
        .with_crosshair(crosshair())
        .with_logical_range(range());

    rsx! {
        div { class: "toolbar",
            Toggle { label: "Synchronize the crosshair", value: crosshair }
            Toggle { label: "Synchronize the visible range", value: range }
        }
        div { class: "grid",
            Chart {
                options: base_options(),
                class: "chart half",
                sync_group: SYNC_GROUP,
                sync_options: sync.clone(),
                CandleStickSeries { data: data.0.clone(), options: CandlestickOptions::new().with_title("ACME") }
                Tooltip { mode: TooltipMode::Magnifier, class: "tooltip small", PriceTooltip {} }
            }
            Chart {
                options: base_options(),
                class: "chart half",
                sync_group: SYNC_GROUP,
                sync_options: sync,
                AreaSeries {
                    data: data.1.clone(),
                    options: AreaSeriesOptions::new()
                        .with_title("GLOBEX")
                        .with_line_color("#16a34a")
                        .with_top_color("rgba(22, 163, 74, 0.3)")
                        .with_bottom_color("rgba(22, 163, 74, 0.02)"),
                }
                Tooltip { mode: TooltipMode::Magnifier, class: "tooltip small", PriceTooltip {} }
            }
        }
    }
}
