use super::{base_options, Choice};
use crate::data::{candlesticks, daily_ohlcv, moving_average};

use charts::{
    chart::{use_crosshair, Chart},
    data::{
        options::{crosshair::CrosshairMode, crosshair::CrosshairOptions},
        LegendLayout,
        LegendOptions,
        LegendPosition,
        TooltipMode,
    },
    overlay::{ChartOverlay, Tooltip},
    series::{
        candlesticks::{CandleStickSeries, CandlestickOptions},
        lines::{LineSeries, LineSeriesOptions},
    },
};
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Legend {
    Series,
    ThreeLines,
    Custom,
}

/// Content of the tooltip: the OHLC prices of the candle under the cursor.
#[component]
fn OhlcTooltip() -> impl IntoView {
    let crosshair = use_crosshair();

    move || {
        crosshair.with(|params| {
            let params = params.as_ref()?;
            let candle = params
                .series_data
                .iter()
                .find(|data| data.series_type == "candlestick")?;
            let item = &candle.data;
            let change = item.change_percent().unwrap_or_default();
            let color = if change >= 0.0 { "#16a34a" } else { "#dc2626" };
            let date = params.time?.into_datetime().format("%a %d %b %Y").to_string();

            Some(view! {
                <div class="tooltip-title">{candle.title.clone().unwrap_or_default()}</div>
                <div class="tooltip-date">{date}</div>
                <table>
                    <tr><td>"Open"</td><td>{format!("{:.2}", item.open.unwrap_or_default())}</td></tr>
                    <tr><td>"High"</td><td>{format!("{:.2}", item.high.unwrap_or_default())}</td></tr>
                    <tr><td>"Low"</td><td>{format!("{:.2}", item.low.unwrap_or_default())}</td></tr>
                    <tr><td>"Close"</td><td>{format!("{:.2}", item.close.unwrap_or_default())}</td></tr>
                </table>
                <div style=format!("color:{color};font-weight:600")>{format!("{change:+.2}%")}</div>
            })
        })
    }
}

/// Custom legend: the values of every series at the crosshair position.
#[component]
fn CustomLegend() -> impl IntoView {
    let crosshair = use_crosshair();

    view! {
        <div class="custom-legend">
            <strong>"Custom legend"</strong>
            {move || {
                crosshair.with(|params| {
                    params
                        .as_ref()
                        .map(|params| {
                            params
                                .series_data
                                .iter()
                                .map(|data| {
                                    let title = data.title.clone().unwrap_or_else(|| data.series_type.clone());
                                    let price = data.data.price().unwrap_or_default();

                                    view! { <div>{format!("{title}: {price:.2}")}</div> }
                                })
                                .collect_view()
                        })
                        .unwrap_or_default()
                })
            }}
        </div>
    }
}

#[component]
pub fn TooltipsDemo() -> impl IntoView {
    let data = daily_ohlcv(5, 150, 80.0);
    let candles = Signal::stored(candlesticks(&data));
    let ma = Signal::stored(moving_average(&data, 10));
    let mode = RwSignal::new(TooltipMode::Floating);
    let legend_kind = RwSignal::new(Legend::Series);
    let legend = Signal::derive(move || match legend_kind.get() {
        Legend::Series => Some(LegendOptions::new().with_text("Built-in legend")),
        Legend::ThreeLines => Some(
            LegendOptions::new()
                .with_text("ACME Corp.")
                .with_layout(LegendLayout::ThreeLines)
                .with_position(LegendPosition::TopLeft)
                .with_time_format("EEEE d MMMM yyyy"),
        ),
        Legend::Custom => None,
    });
    let options = base_options().with_crosshair(CrosshairOptions::new().with_mode(CrosshairMode::Normal));

    view! {
        <Choice
            label="Tooltip"
            value=mode
            options=vec![
                (TooltipMode::Tracking, "Tracking"),
                (TooltipMode::Floating, "Floating"),
                (TooltipMode::Magnifier, "Magnifier"),
            ]
        />
        <Choice
            label="Legend"
            value=legend_kind
            options=vec![(Legend::Series, "Series"), (Legend::ThreeLines, "3 lines"), (Legend::Custom, "Custom")]
        />
        <Chart options=options legend=legend class="chart">
            <CandleStickSeries data=candles options=CandlestickOptions::new().with_title("ACME") />
            <LineSeries
                data=ma
                options=LineSeriesOptions::new()
                    .with_title("MA 10")
                    .with_color("#f97316")
                    .with_price_line_visible(false)
            />
            {move || {
                let mode = mode.get();

                view! {
                    <Tooltip mode=mode class="tooltip">
                        <OhlcTooltip />
                    </Tooltip>
                }
            }}
            <Show when=move || legend_kind.get() == Legend::Custom>
                <ChartOverlay style="top:8px;left:12px">
                    <CustomLegend />
                </ChartOverlay>
            </Show>
        </Chart>
    }
}
