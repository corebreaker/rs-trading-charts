use super::{base_options, Choice};
use crate::data::{candlesticks, closes, daily_ohlcv, moving_average};

use charts::{
    chart::Chart,
    data::{
        options::LineWidth,
        Candlestick,
        register_marker_style,
        HistogramData,
        LegendOptions,
        Marker,
        MarkerPosition,
        MarkerShape,
        MarkerStyle,
        OhlcvData,
        PriceLineOptions,
        ValueData,
    },
    series::{
        areas::{AreaSeries, AreaSeriesOptions},
        bars::{BarSeries, BarSeriesOptions},
        baselines::{BaselineBaseValue, BaselineSeries, BaselineSeriesOptions},
        candlesticks::{CandleStickSeries, CandlestickOptions},
        histograms::{HistogramSeries, HistogramSeriesOptions},
        lines::{LineSeries, LineSeriesOptions},
    },
};
use dioxus::prelude::*;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Candlestick,
    Bar,
    Line,
    Area,
    Baseline,
    Histogram,
}

/// Range of the items replaced by whitespace items: empty slots on the time scale.
const GAP: std::ops::Range<usize> = 60..68;

fn closes_with_gap(data: &[OhlcvData]) -> Vec<ValueData> {
    closes(data)
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            if GAP.contains(&index) {
                ValueData::whitespace(item.time())
            } else {
                item
            }
        })
        .collect()
}

fn candles_with_gap(data: &[OhlcvData]) -> Vec<Candlestick> {
    candlesticks(data)
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            if GAP.contains(&index) {
                Candlestick::whitespace(item.time())
            } else {
                item
            }
        })
        .collect()
}

fn markers(data: &[OhlcvData]) -> Vec<Marker> {
    vec![
        Marker::buy(data[20].time),
        Marker::sell(data[45].time).with_text("Take profit"),
        Marker::custom(data[80].time, "earnings"),
        Marker::custom(data[100].time, "earnings").with_text("Q2"),
        Marker::styled(
            data[30].time,
            MarkerShape::Circle,
            MarkerPosition::AtPriceMiddle,
            "#f59e0b",
        )
        .at_price(data[30].close, MarkerPosition::AtPriceMiddle)
        .with_text("Close"),
        // Two markers at the same time.
        Marker::styled(data[110].time, MarkerShape::Square, MarkerPosition::AboveBar, "#7c3aed").with_text("A"),
        Marker::styled(data[110].time, MarkerShape::Square, MarkerPosition::BelowBar, "#0ea5e9").with_text("B"),
    ]
}

/// Data of the demo, computed once.
struct SeriesData {
    average:     f64,
    candles:     Vec<Candlestick>,
    values:      Vec<ValueData>,
    histogram:   Vec<HistogramData>,
    ma:          Vec<ValueData>,
    markers:     Vec<Marker>,
    price_lines: Vec<PriceLineOptions>,
}

impl SeriesData {
    fn new() -> Self {
        let data = daily_ohlcv(7, 120, 100.0);
        let average = data.iter().map(|item| item.close).sum::<f64>() / data.len() as f64;

        Self {
            average,
            candles: candles_with_gap(&data),
            values: closes_with_gap(&data),
            histogram: data
                .iter()
                .map(|item| HistogramData::new(item.time, item.close - item.open))
                .collect(),
            ma: moving_average(&data, 20),
            markers: markers(&data),
            price_lines: vec![PriceLineOptions::new(average)
                .with_title("Average")
                .with_color("#64748b")
                .with_line_width(LineWidth::W2)],
        }
    }
}

#[component]
pub fn SeriesDemo() -> Element {
    let data = use_hook(|| {
        let _ = register_marker_style(
            "earnings",
            &MarkerStyle::new()
                .with_shape(MarkerShape::Circle)
                .with_position(MarkerPosition::AboveBar)
                .with_color("#db2777")
                .with_text("E"),
        );

        Rc::new(SeriesData::new())
    });
    let kind = use_signal(|| Kind::Candlestick);

    // Each series type is another component: changing the type drops the series and mounts another one.
    let series = match kind() {
        Kind::Candlestick => rsx! {
            CandleStickSeries {
                data: data.candles.clone(),
                markers: data.markers.clone(),
                price_lines: data.price_lines.clone(),
                options: CandlestickOptions::new().with_title("Candles").with_wick_up_color("#16a34a"),
            }
        },
        Kind::Bar => rsx! {
            BarSeries {
                data: data.candles.clone(),
                markers: data.markers.clone(),
                price_lines: data.price_lines.clone(),
                options: BarSeriesOptions::new().with_title("Bars").with_thin_bars(false),
            }
        },
        Kind::Line => rsx! {
            LineSeries {
                data: data.values.clone(),
                markers: data.markers.clone(),
                price_lines: data.price_lines.clone(),
                options: LineSeriesOptions::new().with_title("Close").with_color("#2563eb"),
            }
        },
        Kind::Area => rsx! {
            AreaSeries {
                data: data.values.clone(),
                markers: data.markers.clone(),
                price_lines: data.price_lines.clone(),
                options: AreaSeriesOptions::new()
                    .with_title("Close")
                    .with_line_color("#0891b2")
                    .with_top_color("rgba(8, 145, 178, 0.4)")
                    .with_bottom_color("rgba(8, 145, 178, 0.02)"),
            }
        },
        Kind::Baseline => rsx! {
            BaselineSeries {
                data: data.values.clone(),
                markers: data.markers.clone(),
                price_lines: data.price_lines.clone(),
                options: BaselineSeriesOptions::new()
                    .with_title("Close vs average")
                    .with_base_value(BaselineBaseValue::price(data.average)),
            }
        },
        Kind::Histogram => rsx! {
            HistogramSeries {
                data: data.histogram.clone(),
                markers: data.markers.clone(),
                options: HistogramSeriesOptions::new().with_title("Close - open").with_color("#6366f1"),
            }
        },
    };

    rsx! {
        Choice {
            label: "Series type",
            value: kind,
            options: vec![
                (Kind::Candlestick, "Candlestick"),
                (Kind::Bar, "Bar"),
                (Kind::Line, "Line"),
                (Kind::Area, "Area"),
                (Kind::Baseline, "Baseline"),
                (Kind::Histogram, "Histogram"),
            ],
        }
        Chart {
            options: base_options(),
            legend: LegendOptions::new().with_text("ACME daily"),
            class: "chart",
            {series}
            LineSeries {
                data: data.ma.clone(),
                options: LineSeriesOptions::new()
                    .with_title("MA 20")
                    .with_color("#f97316")
                    .with_line_width(LineWidth::W1)
                    .with_price_line_visible(false)
                    .with_last_value_visible(false),
            }
        }
    }
}
