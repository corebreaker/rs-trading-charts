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
use leptos::prelude::*;

const SYNC_GROUP: &str = "multi-charts-demo";

/// Magnifier tooltip content: the price of the first series under the crosshair.
#[component]
fn PriceTooltip() -> impl IntoView {
    let crosshair = use_crosshair();

    move || {
        crosshair.with(|params| {
            params
                .as_ref()
                .and_then(|params| params.series_data.first())
                .and_then(|data| data.data.price())
                .map(|price| format!("{price:.2}"))
                .unwrap_or_default()
        })
    }
}

#[component]
pub fn MultiChartsDemo() -> impl IntoView {
    let acme = Signal::stored(candlesticks(&daily_ohlcv(8, 250, 100.0)));
    let globex = Signal::stored(closes(&daily_ohlcv(9, 250, 40.0)));
    let crosshair = RwSignal::new(true);
    let range = RwSignal::new(true);
    let sync = Signal::derive(move || {
        SyncOptions::new()
            .with_crosshair(crosshair.get())
            .with_logical_range(range.get())
    });

    view! {
        <div class="toolbar">
            <Toggle label="Synchronize the crosshair" value=crosshair />
            <Toggle label="Synchronize the visible range" value=range />
        </div>
        <div class="grid">
            <Chart options=base_options() class="chart half" sync_group=SYNC_GROUP sync_options=sync>
                <CandleStickSeries data=acme options=CandlestickOptions::new().with_title("ACME") />
                <Tooltip mode=TooltipMode::Magnifier class="tooltip small">
                    <PriceTooltip />
                </Tooltip>
            </Chart>
            <Chart options=base_options() class="chart half" sync_group=SYNC_GROUP sync_options=sync>
                <AreaSeries
                    data=globex
                    options=AreaSeriesOptions::new()
                        .with_title("GLOBEX")
                        .with_line_color("#16a34a")
                        .with_top_color("rgba(22, 163, 74, 0.3)")
                        .with_bottom_color("rgba(22, 163, 74, 0.02)")
                />
                <Tooltip mode=TooltipMode::Magnifier class="tooltip small">
                    <PriceTooltip />
                </Tooltip>
            </Chart>
        </div>
    }
}
