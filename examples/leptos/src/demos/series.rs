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
use leptos::prelude::*;

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

#[component]
pub fn SeriesDemo() -> impl IntoView {
    let _ = register_marker_style(
        "earnings",
        &MarkerStyle::new()
            .with_shape(MarkerShape::Circle)
            .with_position(MarkerPosition::AboveBar)
            .with_color("#db2777")
            .with_text("E"),
    );

    let data = daily_ohlcv(7, 120, 100.0);
    let average = data.iter().map(|item| item.close).sum::<f64>() / data.len() as f64;
    let kind = RwSignal::new(Kind::Candlestick);
    let candles = Signal::stored(candles_with_gap(&data));
    let values = Signal::stored(closes_with_gap(&data));
    let histogram = Signal::stored(
        data.iter()
            .map(|item| HistogramData::new(item.time, item.close - item.open))
            .collect::<Vec<_>>(),
    );
    let ma = Signal::stored(moving_average(&data, 20));
    let markers = Signal::stored(markers(&data));
    let price_lines = Signal::stored(vec![PriceLineOptions::new(average)
        .with_title("Average")
        .with_color("#64748b")
        .with_line_width(LineWidth::W2)]);
    let legend = Signal::stored(Some(LegendOptions::new().with_text("ACME daily")));

    view! {
        <Choice
            label="Series type"
            value=kind
            options=vec![
                (Kind::Candlestick, "Candlestick"),
                (Kind::Bar, "Bar"),
                (Kind::Line, "Line"),
                (Kind::Area, "Area"),
                (Kind::Baseline, "Baseline"),
                (Kind::Histogram, "Histogram"),
            ]
        />
        <Chart options=base_options() legend=legend class="chart">
            {move || match kind.get() {
                Kind::Candlestick => view! {
                    <CandleStickSeries
                        data=candles
                        markers=markers
                        price_lines=price_lines
                        options=CandlestickOptions::new().with_title("Candles").with_wick_up_color("#16a34a")
                    />
                }
                .into_any(),
                Kind::Bar => view! {
                    <BarSeries
                        data=candles
                        markers=markers
                        price_lines=price_lines
                        options=BarSeriesOptions::new().with_title("Bars").with_thin_bars(false)
                    />
                }
                .into_any(),
                Kind::Line => view! {
                    <LineSeries
                        data=values
                        markers=markers
                        price_lines=price_lines
                        options=LineSeriesOptions::new().with_title("Close").with_color("#2563eb")
                    />
                }
                .into_any(),
                Kind::Area => view! {
                    <AreaSeries
                        data=values
                        markers=markers
                        price_lines=price_lines
                        options=AreaSeriesOptions::new()
                            .with_title("Close")
                            .with_line_color("#0891b2")
                            .with_top_color("rgba(8, 145, 178, 0.4)")
                            .with_bottom_color("rgba(8, 145, 178, 0.02)")
                    />
                }
                .into_any(),
                Kind::Baseline => view! {
                    <BaselineSeries
                        data=values
                        markers=markers
                        price_lines=price_lines
                        options=BaselineSeriesOptions::new()
                            .with_title("Close vs average")
                            .with_base_value(BaselineBaseValue::price(average))
                    />
                }
                .into_any(),
                Kind::Histogram => view! {
                    <HistogramSeries
                        data=histogram
                        markers=markers
                        options=HistogramSeriesOptions::new().with_title("Close - open").with_color("#6366f1")
                    />
                }
                .into_any(),
            }}
            <LineSeries
                data=ma
                options=LineSeriesOptions::new()
                    .with_title("MA 20")
                    .with_color("#f97316")
                    .with_line_width(LineWidth::W1)
                    .with_price_line_visible(false)
                    .with_last_value_visible(false)
            />
        </Chart>
    }
}
