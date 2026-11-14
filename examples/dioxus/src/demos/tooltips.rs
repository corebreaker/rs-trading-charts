use super::{base_options, Choice};
use crate::data::{candlesticks, daily_ohlcv, moving_average};

use charts::{
    chart::{use_crosshair, Chart},
    data::{
        options::{crosshair::CrosshairMode, crosshair::CrosshairOptions},
        LegendLayout,
        LegendOptions,
        LegendPosition,
        MouseEventParams,
        TooltipMode,
    },
    overlay::{ChartOverlay, Tooltip},
    series::{
        candlesticks::{CandleStickSeries, CandlestickOptions},
        lines::{LineSeries, LineSeriesOptions},
    },
};
use dioxus::prelude::*;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq)]
enum Legend {
    Series,
    ThreeLines,
    Custom,
}

/// OHLC prices of the candle under the cursor, if any.
fn ohlc(params: &MouseEventParams) -> Option<Element> {
    let candle = params
        .series_data
        .iter()
        .find(|data| data.series_type == "candlestick")?;
    let item = &candle.data;
    let title = candle.title.clone().unwrap_or_default();
    let change = item.change_percent().unwrap_or_default();
    let color = if change >= 0.0 { "#16a34a" } else { "#dc2626" };
    let date = params.time?.into_datetime().format("%a %d %b %Y").to_string();
    let open = format!("{:.2}", item.open.unwrap_or_default());
    let high = format!("{:.2}", item.high.unwrap_or_default());
    let low = format!("{:.2}", item.low.unwrap_or_default());
    let close = format!("{:.2}", item.close.unwrap_or_default());

    Some(rsx! {
        div { class: "tooltip-title", "{title}" }
        div { class: "tooltip-date", "{date}" }
        table {
            tr {
                td { "Open" }
                td { "{open}" }
            }
            tr {
                td { "High" }
                td { "{high}" }
            }
            tr {
                td { "Low" }
                td { "{low}" }
            }
            tr {
                td { "Close" }
                td { "{close}" }
            }
        }
        div { style: "color:{color};font-weight:600", {format!("{change:+.2}%")} }
    })
}

/// Content of the tooltip: the OHLC prices of the candle under the cursor.
#[component]
fn OhlcTooltip() -> Element {
    let crosshair = use_crosshair();
    let content = crosshair.read().as_ref().and_then(ohlc);

    rsx! {
        {content}
    }
}

/// Custom legend: the values of every series at the crosshair position.
#[component]
fn CustomLegend() -> Element {
    let crosshair = use_crosshair();
    let lines = crosshair
        .read()
        .as_ref()
        .map(|params| {
            params
                .series_data
                .iter()
                .map(|data| {
                    let title = data.title.clone().unwrap_or_else(|| data.series_type.clone());
                    let price = data.data.price().unwrap_or_default();

                    format!("{title}: {price:.2}")
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    rsx! {
        div { class: "custom-legend",
            strong { "Custom legend" }
            for line in lines {
                div { "{line}" }
            }
        }
    }
}

#[component]
pub fn TooltipsDemo() -> Element {
    let data = use_hook(|| {
        let data = daily_ohlcv(5, 150, 80.0);

        Rc::new((candlesticks(&data), moving_average(&data, 10)))
    });
    let mode = use_signal(|| TooltipMode::Floating);
    let legend_kind = use_signal(|| Legend::Series);
    let legend = match legend_kind() {
        Legend::Series => Some(LegendOptions::new().with_text("Built-in legend")),
        Legend::ThreeLines => Some(
            LegendOptions::new()
                .with_text("ACME Corp.")
                .with_layout(LegendLayout::ThreeLines)
                .with_position(LegendPosition::TopLeft)
                .with_time_format("EEEE d MMMM yyyy"),
        ),
        Legend::Custom => None,
    };
    let options = base_options().with_crosshair(CrosshairOptions::new().with_mode(CrosshairMode::Normal));

    rsx! {
        Choice {
            label: "Tooltip",
            value: mode,
            options: vec![
                (TooltipMode::Tracking, "Tracking"),
                (TooltipMode::Floating, "Floating"),
                (TooltipMode::Magnifier, "Magnifier"),
            ],
        }
        Choice {
            label: "Legend",
            value: legend_kind,
            options: vec![(Legend::Series, "Series"), (Legend::ThreeLines, "3 lines"), (Legend::Custom, "Custom")],
        }
        Chart { options, legend, class: "chart",
            CandleStickSeries { data: data.0.clone(), options: CandlestickOptions::new().with_title("ACME") }
            LineSeries {
                data: data.1.clone(),
                options: LineSeriesOptions::new()
                    .with_title("MA 10")
                    .with_color("#f97316")
                    .with_price_line_visible(false),
            }
            Tooltip { mode: mode(), class: "tooltip", OhlcTooltip {} }
            if legend_kind() == Legend::Custom {
                ChartOverlay { style: "top:8px;left:12px", CustomLegend {} }
            }
        }
    }
}
