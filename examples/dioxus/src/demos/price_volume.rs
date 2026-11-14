use super::{base_options, with_chart, Choice};
use crate::data::daily_ohlcv;

use charts::{
    chart::Chart,
    data::{
        resample::{resample_ohlcv, Interval},
        PriceSeriesStyle,
        RangePreset,
    },
    series::price_volume::PriceVolumeSeries,
    series_data::{
        areas::AreaSeriesOptions,
        bars::BarSeriesOptions,
        candlesticks::CandlestickOptions,
        lines::LineSeriesOptions,
    },
    ChartHandle,
};
use dioxus::prelude::*;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Style {
    Candles,
    Bars,
    Line,
    Area,
}

impl Style {
    fn series_style(self) -> PriceSeriesStyle {
        match self {
            Style::Candles => PriceSeriesStyle::Candlestick(CandlestickOptions::new().with_title("ACME")),
            Style::Bars => PriceSeriesStyle::Bar(BarSeriesOptions::new().with_title("ACME")),
            Style::Line => PriceSeriesStyle::Line(LineSeriesOptions::new().with_title("ACME").with_color("#2563eb")),
            Style::Area => PriceSeriesStyle::Area(
                AreaSeriesOptions::new()
                    .with_title("ACME")
                    .with_line_color("#2563eb")
                    .with_top_color("rgba(37, 99, 235, 0.35)")
                    .with_bottom_color("rgba(37, 99, 235, 0.02)"),
            ),
        }
    }
}

#[component]
pub fn PriceVolumeDemo() -> Element {
    let daily = use_hook(|| Rc::new(daily_ohlcv(42, 730, 120.0)));
    let style = use_signal(|| Style::Candles);
    let interval = use_signal(|| Interval::Days(1));
    let volume_height = use_signal(|| 0.25);
    let mut chart = use_signal(|| None::<ChartHandle>);
    let data = use_memo(move || match interval() {
        Interval::Days(1) => daily.to_vec(),
        interval => resample_ohlcv(&daily, interval),
    });

    // Show all the data when the interval changes.
    use_effect(move || {
        interval.read();
        with_chart(chart, |chart| chart.show_range(RangePreset::All));
    });

    let show = move |preset: RangePreset| with_chart(chart, |chart| chart.show_range(preset));
    let current_style = style();

    rsx! {
        Choice {
            label: "Price style",
            value: style,
            options: vec![
                (Style::Candles, "Candles"),
                (Style::Bars, "Bars"),
                (Style::Line, "Line"),
                (Style::Area, "Area"),
            ],
        }
        Choice {
            label: "Interval",
            value: interval,
            options: vec![(Interval::Days(1), "1D"), (Interval::Weeks(1), "1W"), (Interval::Months(1), "1M")],
        }
        Choice {
            label: "Volume height",
            value: volume_height,
            options: vec![(0.15, "15%"), (0.25, "25%"), (0.4, "40%")],
        }
        div { class: "toolbar",
            span { class: "label", "Range" }
            button { onclick: move |_| show(RangePreset::Months(1)), "1M" }
            button { onclick: move |_| show(RangePreset::Months(3)), "3M" }
            button { onclick: move |_| show(RangePreset::Months(6)), "6M" }
            button { onclick: move |_| show(RangePreset::YearToDate), "YTD" }
            button { onclick: move |_| show(RangePreset::Years(1)), "1Y" }
            button { onclick: move |_| show(RangePreset::Bars(50)), "50 bars" }
            button { onclick: move |_| show(RangePreset::All), "All" }
        }
        Chart {
            options: base_options(),
            class: "chart",
            on_ready: move |handle| chart.set(Some(handle)),
            // The type of the price series is chosen at mount: the key mounts the component again for another style.
            {
                rsx! {
                    PriceVolumeSeries {
                        key: "{current_style:?}",
                        data: data(),
                        style: current_style.series_style(),
                        volume_height: volume_height(),
                    }
                }
            }
        }
    }
}
