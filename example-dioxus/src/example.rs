use super::dataset::Dataset;
use charts::{
    chart::{Chart, use_chart},
    data::{
        PriceLineOptions,
        options::{
            background::Background,
            layout::{LayoutOptions, LayoutPanesOptions},
            ChartOptions, TimeScaleOptions,
        },
    },
    panel::ChartPanel,
    series::{
        areas::{AreaSeries, AreaSeriesOptions},
        bars::{BarSeries, BarSeriesOptions},
        candlesticks::CandleStickSeries,
        histograms::{HistogramSeries, HistogramSeriesOptions},
        lines::{LineSeries, LineSeriesOptions},
    },
};
use dioxus::prelude::*;
use log::error;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::window;

pub fn app() -> Element {
    let range_text = use_signal(|| String::from("Visible range: pending"));
    let chart_options = use_signal(|| {
        ChartOptions::new()
            .with_time_scale(TimeScaleOptions::new().with_time_visible(true))
            .with_layout(
                LayoutOptions::new()
                    .with_background(Background::new_solid_color(String::from("white")))
                    .with_panes(LayoutPanesOptions::new())
                    .with_text_color(String::from("black")),
            )
            .with_auto_size(true)
    });
    let line_options = LineSeriesOptions::new()
        .with_title(String::from("Close"))
        .with_color(String::from("#1d4ed8"))
        .with_price_line_visible(false);
    let area_options = AreaSeriesOptions::new()
        .with_title(String::from("Mid"))
        .with_top_color(String::from("rgba(14, 165, 233, 0.28)"))
        .with_bottom_color(String::from("rgba(14, 165, 233, 0.02)"))
        .with_line_color(String::from("rgba(2, 132, 199, 0.95)"))
        .with_price_line_visible(false);
    let bar_options = BarSeriesOptions::new()
        .with_title(String::from("Bar"))
        .with_open_visible(true)
        .with_price_line_visible(false);
    let histogram_options = HistogramSeriesOptions::new()
        .with_title(String::from("Delta"))
        .with_base(0.0)
        .with_price_line_visible(false);
    let line_price_lines = vec![PriceLineOptions::new(10.5)
        .with_color(String::from("#1d4ed8"))
        .with_title(String::from("Close line"))];
    let bar_price_lines = vec![PriceLineOptions::new(10.0)
        .with_color(String::from("#dc2626"))
        .with_title(String::from("Bar ref"))];
    let mut data = use_signal(Dataset::new);

    rsx! {
        div {
            style: "margin-top:10px;padding:10px",
            h1 { "Dioxus trading charts example" }
            div {
                style: "border:1px dashed black;height:768px",
                Chart {
                    options: Some(chart_options()),
                    style: Some(String::from("width:100%;height:100%")),
                    ChartPanel {
                        CandleStickSeries {
                            data: data.read().data_up().clone(),
                            markers: data.read().markers().clone(),
                        }
                        LineSeries {
                            options: Some(line_options.clone()),
                            data: data.read().line_up(),
                            markers: Vec::new(),
                            price_lines: line_price_lines.clone(),
                        }
                        AreaSeries {
                            options: Some(area_options.clone()),
                            data: data.read().area_up(),
                            markers: Vec::new(),
                        }
                    }
                    ChartPanel {
                        BarSeries {
                            options: Some(bar_options.clone()),
                            data: data.read().data_down().clone(),
                            markers: data.read().markers().clone(),
                            price_lines: bar_price_lines.clone(),
                        }
                        HistogramSeries {
                            options: Some(histogram_options.clone()),
                            data: data.read().histogram_down(),
                            markers: Vec::new(),
                        }
                    }
                    VisibleRangeProbe {
                        recent_range: data.read().recent_range(),
                        range_text,
                    }
                }
            }

            div {
                style: "margin-top:10px;display:flex;flex-direction:column;row-gap:10px;",
                span { "{range_text()}" }
                div {
                    style: "display:flex;flex-direction:row;column-gap:10px;",
                button {
                    onclick: move |_| data.write().inc(),
                    "Change markers"
                }
                button {
                    onclick: move |_| match Dataset::load("data1") {
                        Ok(new_data) => data.set(new_data),
                        Err(err) => error!("Failed to load dataset 1: {err}"),
                    },
                    "Load dataset 1"
                }
                button {
                    onclick: move |_| match Dataset::load("data2") {
                        Ok(new_data) => data.set(new_data),
                        Err(err) => error!("Failed to load dataset 2: {err}"),
                    },
                    "Load dataset 2"
                }
                }
            }
        }
    }
}

#[derive(Clone, Props)]
struct VisibleRangeProbeProps {
    recent_range: Option<charts::data::TimeRange>,
    range_text: Signal<String>,
}

impl PartialEq for VisibleRangeProbeProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[allow(non_snake_case)]
fn VisibleRangeProbe(props: VisibleRangeProbeProps) -> Element {
    let chart = use_chart();
    let mut last_applied_range = use_signal(|| None::<charts::data::TimeRange>);

    {
        let chart = chart.clone();
        let recent_range = props.recent_range;
        let mut range_text = props.range_text;
        use_effect(move || {
            let Some(range) = recent_range else {
                range_text.set(String::from("Visible range: unavailable"));
                return;
            };

            if last_applied_range() == Some(range) {
                return;
            }

            last_applied_range.set(Some(range));
            schedule_visible_range_probe(chart.clone(), range, range_text, 6);
        });
    }

    rsx! {}
}

fn schedule_visible_range_probe(
    chart: charts::ChartHandle,
    range: charts::data::TimeRange,
    mut range_text: Signal<String>,
    retries_left: usize,
) {
    let callback = Closure::<dyn FnMut()>::new(move || {
        if let Err(err) = chart.set_visible_range(&range) {
            err.with_prefix("Failed to set visible range").log();
            range_text.set(String::from("Visible range: set failed"));
            return;
        }

        match chart.get_visible_range() {
            Ok(Some(current)) if current == range => {
                range_text.set(format!("Visible range: {} -> {}", current.from(), current.to()));
            }
            Ok(Some(current)) if retries_left > 0 => {
                range_text.set(format!("Visible range: waiting {} -> {}", current.from(), current.to()));
                schedule_visible_range_probe(chart.clone(), range, range_text, retries_left - 1);
            }
            Ok(Some(current)) => {
                range_text.set(format!("Visible range: {} -> {}", current.from(), current.to()));
            }
            Ok(None) if retries_left > 0 => {
                range_text.set(String::from("Visible range: waiting"));
                schedule_visible_range_probe(chart.clone(), range, range_text, retries_left - 1);
            }
            Ok(None) => range_text.set(String::from("Visible range: none")),
            Err(err) => {
                err.with_prefix("Failed to get visible range").log();
                range_text.set(String::from("Visible range: read failed"));
            }
        }
    });

    match window() {
        Some(window) => {
            if let Err(err) =
                window.set_timeout_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 100)
            {
                charts::JsError::from(err)
                    .with_prefix("Failed to schedule visible range probe")
                    .log();
                range_text.set(String::from("Visible range: probe scheduling failed"));
            }
        }

        None => {
            charts::JsError::new_from_str("window is not available")
                .with_prefix("Failed to schedule visible range probe")
                .log();
            range_text.set(String::from("Visible range: window unavailable"));
        }
    }

    callback.forget();
}
