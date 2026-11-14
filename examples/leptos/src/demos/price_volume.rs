use super::{base_options, keep, Choice};
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
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
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
pub fn PriceVolumeDemo() -> impl IntoView {
    let daily = daily_ohlcv(42, 730, 120.0);
    let style = RwSignal::new(Style::Candles);
    let interval = RwSignal::new(Interval::Days(1));
    let volume_height = RwSignal::new(0.25);
    let chart = StoredValue::new(None::<ChartHandle>);
    let data = Signal::derive(move || {
        let interval = interval.get();

        match interval {
            Interval::Days(1) => daily.clone(),
            interval => resample_ohlcv(&daily, interval),
        }
    });

    // Show all the data when the interval changes.
    Effect::new(move || {
        interval.track();
        if let Some(chart) = chart.get_value() {
            let _ = chart.show_range(RangePreset::All);
        }
    });

    let show = move |preset: RangePreset| {
        if let Some(chart) = chart.get_value() {
            if let Err(err) = chart.show_range(preset) {
                err.log();
            }
        }
    };

    view! {
        <Choice
            label="Price style"
            value=style
            options=vec![
                (Style::Candles, "Candles"),
                (Style::Bars, "Bars"),
                (Style::Line, "Line"),
                (Style::Area, "Area"),
            ]
        />
        <Choice
            label="Interval"
            value=interval
            options=vec![(Interval::Days(1), "1D"), (Interval::Weeks(1), "1W"), (Interval::Months(1), "1M")]
        />
        <Choice
            label="Volume height"
            value=volume_height
            options=vec![(0.15, "15%"), (0.25, "25%"), (0.4, "40%")]
        />
        <div class="toolbar">
            <span class="label">"Range"</span>
            <button on:click=move |_| show(RangePreset::Months(1))>"1M"</button>
            <button on:click=move |_| show(RangePreset::Months(3))>"3M"</button>
            <button on:click=move |_| show(RangePreset::Months(6))>"6M"</button>
            <button on:click=move |_| show(RangePreset::YearToDate)>"YTD"</button>
            <button on:click=move |_| show(RangePreset::Years(1))>"1Y"</button>
            <button on:click=move |_| show(RangePreset::Bars(50))>"50 bars"</button>
            <button on:click=move |_| show(RangePreset::All)>"All"</button>
        </div>
        <Chart options=base_options() class="chart" on_ready=keep(chart)>
            // The type of the price series is chosen at mount: the component is mounted again for another style.
            {move || {
                let style = style.get().series_style();

                view! { <PriceVolumeSeries data=data style=style volume_height=volume_height /> }
            }}
        </Chart>
    }
}
