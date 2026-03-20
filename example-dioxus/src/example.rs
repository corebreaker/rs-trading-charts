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
        baselines::{BaselineBaseValue, BaselineSeries, BaselineSeriesOptions},
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
    let screenshot_status = use_signal(|| String::from("Screenshot: idle"));
    let screenshot_data_url = use_signal(|| None::<String>);
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
    let baseline_options = BaselineSeriesOptions::new()
        .with_title(String::from("Spread"))
        .with_base_value(BaselineBaseValue::price(10.0))
        .with_top_fill_color1(String::from("rgba(16, 185, 129, 0.22)"))
        .with_top_fill_color2(String::from("rgba(16, 185, 129, 0.04)"))
        .with_top_line_color(String::from("rgba(5, 150, 105, 0.95)"))
        .with_bottom_fill_color1(String::from("rgba(239, 68, 68, 0.04)"))
        .with_bottom_fill_color2(String::from("rgba(239, 68, 68, 0.22)"))
        .with_bottom_line_color(String::from("rgba(220, 38, 38, 0.95)"))
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
                        BaselineSeries {
                            options: Some(baseline_options.clone()),
                            data: data.read().line_up(),
                            markers: Vec::new(),
                            price_lines: Vec::new(),
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
                    ScreenshotCapture {
                        screenshot_status,
                        screenshot_data_url,
                    }
                }
            }

            div {
                style: "margin-top:10px;display:flex;flex-direction:column;row-gap:10px;",
                span { "{range_text()}" }
                span { "{screenshot_status()}" }
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
                if let Some(data_url) = screenshot_data_url() {
                    img {
                        src: data_url,
                        style: "max-width:280px;border:1px solid #94a3b8;",
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

#[derive(Clone, Props)]
struct ScreenshotCaptureProps {
    screenshot_status: Signal<String>,
    screenshot_data_url: Signal<Option<String>>,
}

impl PartialEq for ScreenshotCaptureProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[allow(non_snake_case)]
fn ScreenshotCapture(props: ScreenshotCaptureProps) -> Element {
    let chart = use_chart();
    let mut captured = use_signal(|| false);

    {
        let chart = chart.clone();
        let mut screenshot_status = props.screenshot_status;
        let mut screenshot_data_url = props.screenshot_data_url;
        use_effect(move || {
            if captured() {
                return;
            }

            captured.set(true);

            let delayed_chart = chart.clone();
            let callback = Closure::<dyn FnMut()>::new(move || match delayed_chart.take_screenshot_data_url() {
                Ok(data_url) => {
                    screenshot_status.set(format!("Screenshot: captured {} chars", data_url.len()));
                    screenshot_data_url.set(Some(data_url));
                }
                Err(err) => {
                    err.with_prefix("Failed to capture screenshot").log();
                    screenshot_status.set(String::from("Screenshot: capture failed"));
                }
            });

            match window() {
                Some(window) => {
                    if let Err(err) = window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 800)
                    {
                        charts::JsError::from(err)
                            .with_prefix("Failed to schedule screenshot capture")
                            .log();
                        screenshot_status.set(String::from("Screenshot: scheduling failed"));
                    }
                }

                None => {
                    charts::JsError::new_from_str("window is not available")
                        .with_prefix("Failed to schedule screenshot capture")
                        .log();
                    screenshot_status.set(String::from("Screenshot: window unavailable"));
                }
            }

            callback.forget();
        });
    }

    rsx! {}
}
