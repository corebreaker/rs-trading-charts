use super::dataset::Dataset;
use charts::{
    chart::{Chart, use_chart},
    data::{
        LegendOptions, PriceLineOptions,
        options::{
            background::Background,
            cross_hair::{CrossHairOptions, CrosshairLineOptions},
            layout::{LayoutOptions, LayoutPanesOptions},
            ChartOptions, LastPriceAnimationMode, LineType, LineWidth, PriceFormatOptions, PriceScaleOptions,
            TimeScaleOptions,
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
    let price_scale_text = use_signal(|| String::from("Price scale: pending"));
    let pane_text = use_signal(|| String::from("Panes: pending"));
    let coordinate_text = use_signal(|| String::from("Coordinates: pending"));
    let watermark_text = use_signal(|| String::from("Watermarks: pending"));
    let direct_series_text = use_signal(|| String::from("Direct series: idle"));
    let screenshot_status = use_signal(|| String::from("Screenshot: idle"));
    let screenshot_data_url = use_signal(|| None::<String>);
    let mut price_scale_zoom_request = use_signal(|| 0_u64);
    let mut price_scale_auto_request = use_signal(|| 0_u64);
    let mut pane_resize_request = use_signal(|| 0_u64);
    let mut pane_swap_request = use_signal(|| 0_u64);
    let mut coordinate_probe_request = use_signal(|| 0_u64);
    let mut watermark_update_request = use_signal(|| 0_u64);
    let mut direct_series_add_request = use_signal(|| 0_u64);
    let mut direct_series_remove_request = use_signal(|| 0_u64);
    let chart_options = use_signal(|| {
        ChartOptions::new()
            .with_time_scale(TimeScaleOptions::new().with_time_visible(true))
            .with_cross_hair(
                CrossHairOptions::new().with_vert_line(
                    CrosshairLineOptions::new()
                        .with_color(String::from("#0f172a"))
                        .with_label_background_color(String::from("#0f172a")),
                ),
            )
            .with_layout(
                LayoutOptions::new()
                    .with_background(Background::new_solid_color(String::from("white")))
                    .with_panes(LayoutPanesOptions::new())
                    .with_text_color(String::from("black")),
            )
            .with_auto_size(true)
    });
    let legend_options = LegendOptions::new()
        .with_text_color("#0f172a")
        .with_background_color("rgba(255, 255, 255, 0.82)")
        .with_font_size(12.0)
        .with_top(10.0)
        .with_left(10.0);
    let line_options = LineSeriesOptions::new()
        .with_title(String::from("Close"))
        .with_color(String::from("#1d4ed8"))
        .with_line_type(LineType::Curved)
        .with_point_markers_visible(true)
        .with_point_markers_radius(3.0)
        .with_last_price_animation(LastPriceAnimationMode::Continuous)
        .with_price_format(PriceFormatOptions::price().with_precision(3).with_min_move(0.001))
        .with_price_line_visible(false);
    let area_options = AreaSeriesOptions::new()
        .with_title(String::from("Mid"))
        .with_top_color(String::from("rgba(14, 165, 233, 0.28)"))
        .with_bottom_color(String::from("rgba(14, 165, 233, 0.02)"))
        .with_line_color(String::from("rgba(2, 132, 199, 0.95)"))
        .with_relative_gradient(true)
        .with_line_type(LineType::Curved)
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
        .with_price_format(PriceFormatOptions::volume())
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
                    legend: Some(legend_options.clone()),
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
                    PriceScaleProbe {
                        price_scale_text,
                        zoom_request: price_scale_zoom_request,
                        auto_request: price_scale_auto_request,
                    }
                    PaneProbe {
                        pane_text,
                        resize_request: pane_resize_request,
                        swap_request: pane_swap_request,
                    }
                    CoordinateProbe {
                        coordinate_text,
                        recent_range: data.read().recent_range(),
                        probe_request: coordinate_probe_request,
                    }
                    WatermarkProbe {
                        watermark_text,
                        update_request: watermark_update_request,
                    }
                    DirectSeriesProbe {
                        direct_series_text,
                        add_request: direct_series_add_request,
                        remove_request: direct_series_remove_request,
                        data: data.read().line_up(),
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
                span { "{price_scale_text()}" }
                span { "{pane_text()}" }
                span { "{coordinate_text()}" }
                span { "{watermark_text()}" }
                span { "{direct_series_text()}" }
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
                    button {
                        onclick: move |_| price_scale_zoom_request.set(price_scale_zoom_request() + 1),
                        "Zoom price scale"
                    }
                    button {
                        onclick: move |_| price_scale_auto_request.set(price_scale_auto_request() + 1),
                        "Auto price scale"
                    }
                    button {
                        onclick: move |_| pane_resize_request.set(pane_resize_request() + 1),
                        "Resize lower pane"
                    }
                    button {
                        onclick: move |_| pane_swap_request.set(pane_swap_request() + 1),
                        "Swap panes"
                    }
                    button {
                        onclick: move |_| coordinate_probe_request.set(coordinate_probe_request() + 1),
                        "Probe coordinates"
                    }
                    button {
                        onclick: move |_| watermark_update_request.set(watermark_update_request() + 1),
                        "Update watermarks"
                    }
                    button {
                        onclick: move |_| direct_series_add_request.set(direct_series_add_request() + 1),
                        "Add direct line"
                    }
                    button {
                        onclick: move |_| direct_series_remove_request.set(direct_series_remove_request() + 1),
                        "Remove direct line"
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

#[derive(Clone, Props)]
struct PriceScaleProbeProps {
    price_scale_text: Signal<String>,
    zoom_request: Signal<u64>,
    auto_request: Signal<u64>,
}

impl PartialEq for PriceScaleProbeProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[allow(non_snake_case)]
fn PriceScaleProbe(props: PriceScaleProbeProps) -> Element {
    let chart = use_chart();
    let mut initial_reported = use_signal(|| false);
    let mut last_zoom_request = use_signal(|| 0_u64);
    let mut last_auto_request = use_signal(|| 0_u64);

    {
        let chart = chart.clone();
        let price_scale_text = props.price_scale_text;
        use_effect(move || {
            if initial_reported() {
                return;
            }

            initial_reported.set(true);
            schedule_price_scale_report(chart.clone(), price_scale_text, String::from("Price scale"), 150);
        });
    }

    {
        let chart = chart.clone();
        let price_scale_text = props.price_scale_text;
        use_effect(move || {
            let zoom_request = (props.zoom_request)();
            if zoom_request == 0 || zoom_request == last_zoom_request() {
                return;
            }

            last_zoom_request.set(zoom_request);
            schedule_price_scale_zoom(chart.clone(), price_scale_text, 150);
        });
    }

    {
        let chart = chart.clone();
        let price_scale_text = props.price_scale_text;
        use_effect(move || {
            let auto_request = (props.auto_request)();
            if auto_request == 0 || auto_request == last_auto_request() {
                return;
            }

            last_auto_request.set(auto_request);
            schedule_price_scale_auto(chart.clone(), price_scale_text, 150);
        });
    }

    rsx! {}
}

#[derive(Clone, Props)]
struct PaneProbeProps {
    pane_text: Signal<String>,
    resize_request: Signal<u64>,
    swap_request: Signal<u64>,
}

impl PartialEq for PaneProbeProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[allow(non_snake_case)]
fn PaneProbe(props: PaneProbeProps) -> Element {
    let chart = use_chart();
    let mut initial_reported = use_signal(|| false);
    let mut last_resize_request = use_signal(|| 0_u64);
    let mut last_swap_request = use_signal(|| 0_u64);

    {
        let chart = chart.clone();
        let pane_text = props.pane_text;
        use_effect(move || {
            if initial_reported() {
                return;
            }

            initial_reported.set(true);
            schedule_pane_report(chart.clone(), pane_text, 150);
        });
    }

    {
        let chart = chart.clone();
        let pane_text = props.pane_text;
        use_effect(move || {
            let resize_request = (props.resize_request)();
            if resize_request == 0 || resize_request == last_resize_request() {
                return;
            }

            last_resize_request.set(resize_request);
            schedule_pane_resize(chart.clone(), pane_text, 150);
        });
    }

    {
        let chart = chart.clone();
        let pane_text = props.pane_text;
        use_effect(move || {
            let swap_request = (props.swap_request)();
            if swap_request == 0 || swap_request == last_swap_request() {
                return;
            }

            last_swap_request.set(swap_request);
            schedule_pane_swap(chart.clone(), pane_text, 150);
        });
    }

    rsx! {}
}

#[derive(Clone, Props)]
struct CoordinateProbeProps {
    coordinate_text: Signal<String>,
    recent_range: Option<charts::data::TimeRange>,
    probe_request: Signal<u64>,
}

impl PartialEq for CoordinateProbeProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[allow(non_snake_case)]
fn CoordinateProbe(props: CoordinateProbeProps) -> Element {
    let chart = use_chart();
    let mut initial_reported = use_signal(|| false);
    let mut last_probe_request = use_signal(|| 0_u64);

    {
        let chart = chart.clone();
        let coordinate_text = props.coordinate_text;
        let recent_range = props.recent_range;
        use_effect(move || {
            if initial_reported() {
                return;
            }

            initial_reported.set(true);
            schedule_coordinate_probe(chart.clone(), recent_range, coordinate_text, 150);
        });
    }

    {
        let chart = chart.clone();
        let coordinate_text = props.coordinate_text;
        let recent_range = props.recent_range;
        use_effect(move || {
            let probe_request = (props.probe_request)();
            if probe_request == 0 || probe_request == last_probe_request() {
                return;
            }

            last_probe_request.set(probe_request);
            schedule_coordinate_probe(chart.clone(), recent_range, coordinate_text, 150);
        });
    }

    rsx! {}
}

#[derive(Clone, Props)]
struct WatermarkProbeProps {
    watermark_text: Signal<String>,
    update_request: Signal<u64>,
}

impl PartialEq for WatermarkProbeProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[allow(non_snake_case)]
fn WatermarkProbe(props: WatermarkProbeProps) -> Element {
    let chart = use_chart();
    let mut initialized = use_signal(|| false);
    let mut last_update_request = use_signal(|| 0_u64);
    let text_watermark_id = use_signal(|| None::<String>);
    let image_watermark_id = use_signal(|| None::<String>);

    {
        let chart = chart.clone();
        let watermark_text = props.watermark_text;
        use_effect(move || {
            if initialized() {
                return;
            }

            initialized.set(true);
            schedule_watermark_init(
                chart.clone(),
                watermark_text,
                text_watermark_id,
                image_watermark_id,
                150,
            );
        });
    }

    {
        let chart = chart.clone();
        let watermark_text = props.watermark_text;
        use_effect(move || {
            let update_request = (props.update_request)();
            if update_request == 0 || update_request == last_update_request() {
                return;
            }

            last_update_request.set(update_request);
            schedule_watermark_update(
                chart.clone(),
                watermark_text,
                text_watermark_id(),
                image_watermark_id(),
                150,
            );
        });
    }

    rsx! {}
}

#[derive(Clone, Props)]
struct DirectSeriesProbeProps {
    direct_series_text: Signal<String>,
    add_request: Signal<u64>,
    remove_request: Signal<u64>,
    data: Vec<charts::data::ValueData>,
}

impl PartialEq for DirectSeriesProbeProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[allow(non_snake_case)]
fn DirectSeriesProbe(props: DirectSeriesProbeProps) -> Element {
    let chart = use_chart();
    let mut series_id = use_signal(|| None::<String>);
    let mut last_add_request = use_signal(|| 0_u64);
    let mut last_remove_request = use_signal(|| 0_u64);

    {
        let chart = chart.clone();
        let mut direct_series_text = props.direct_series_text;
        let data = props.data.clone();
        use_effect(move || {
            let add_request = (props.add_request)();
            if add_request == 0 || add_request == last_add_request() {
                return;
            }

            last_add_request.set(add_request);

            if let Some(series_id) = series_id() {
                if let Err(err) = chart.update_data(series_id.clone(), &data) {
                    err.with_prefix("Failed to refresh direct line series").log();
                    direct_series_text.set(String::from("Direct series: refresh failed"));
                    return;
                }

                direct_series_text.set(format!("Direct series: refreshed {}", series_id));
                return;
            }

            let options = LineSeriesOptions::new()
                .with_title(String::from("Direct MA"))
                .with_color(String::from("#f97316"))
                .with_line_width(LineWidth::W2)
                .with_line_type(LineType::Curved)
                .with_point_markers_visible(false)
                .with_price_line_visible(false);

            match chart.add_line_series(data.clone(), Some(options), Some(0)) {
                Ok(new_series_id) => {
                    series_id.set(Some(new_series_id.clone()));
                    direct_series_text.set(format!("Direct series: added {}", new_series_id));
                }
                Err(err) => {
                    err.with_prefix("Failed to add direct line series").log();
                    direct_series_text.set(String::from("Direct series: add failed"));
                }
            }
        });
    }

    {
        let chart = chart.clone();
        let mut direct_series_text = props.direct_series_text;
        use_effect(move || {
            let remove_request = (props.remove_request)();
            if remove_request == 0 || remove_request == last_remove_request() {
                return;
            }

            last_remove_request.set(remove_request);

            let Some(current_series_id) = series_id() else {
                direct_series_text.set(String::from("Direct series: nothing to remove"));
                return;
            };

            match chart.remove_series(current_series_id.clone()) {
                Ok(()) => {
                    series_id.set(None);
                    direct_series_text.set(format!("Direct series: removed {}", current_series_id));
                }
                Err(err) => {
                    err.with_prefix("Failed to remove direct line series").log();
                    direct_series_text.set(String::from("Direct series: remove failed"));
                }
            }
        });
    }

    {
        let chart = chart.clone();
        let mut direct_series_text = props.direct_series_text;
        let data = props.data.clone();
        use_effect(move || {
            let Some(current_series_id) = series_id() else {
                return;
            };

            if let Err(err) = chart.update_data(current_series_id.clone(), &data) {
                err.with_prefix("Failed to update direct line series data").log();
                direct_series_text.set(String::from("Direct series: data sync failed"));
                return;
            }

            direct_series_text.set(format!("Direct series: synced {}", current_series_id));
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

fn schedule_price_scale_zoom(chart: charts::ChartHandle, mut price_scale_text: Signal<String>, delay_ms: i32) {
    let callback = Closure::<dyn FnMut()>::new(move || {
        let options = PriceScaleOptions::new()
            .with_auto_scale(false)
            .with_minimum_width(96.0)
            .with_ensure_edge_tick_marks_visible(true);
        let range = charts::data::PriceRange::new(8.5, 12.5);

        if let Err(err) = chart.apply_price_scale_options("right", Some(0), &options) {
            err.with_prefix("Failed to apply price scale options").log();
            price_scale_text.set(String::from("Price scale: options failed"));
            return;
        }

        if let Err(err) = chart.set_price_scale_visible_range("right", Some(0), &range) {
            err.with_prefix("Failed to set price scale visible range").log();
            price_scale_text.set(String::from("Price scale: range set failed"));
            return;
        }

        update_price_scale_text(&chart, &mut price_scale_text, "Price scale: zoomed");
    });

    schedule_timeout(callback, delay_ms, &mut price_scale_text, "price scale zoom");
}

fn schedule_price_scale_auto(chart: charts::ChartHandle, mut price_scale_text: Signal<String>, delay_ms: i32) {
    let callback = Closure::<dyn FnMut()>::new(move || {
        if let Err(err) = chart.set_price_scale_auto_scale("right", Some(0), true) {
            err.with_prefix("Failed to enable price scale autoscale").log();
            price_scale_text.set(String::from("Price scale: autoscale failed"));
            return;
        }

        update_price_scale_text(&chart, &mut price_scale_text, "Price scale: auto");
    });

    schedule_timeout(callback, delay_ms, &mut price_scale_text, "price scale autoscale");
}

fn schedule_price_scale_report(
    chart: charts::ChartHandle,
    mut price_scale_text: Signal<String>,
    label: String,
    delay_ms: i32,
) {
    let callback = Closure::<dyn FnMut()>::new(move || {
        update_price_scale_text(&chart, &mut price_scale_text, &label);
    });

    schedule_timeout(callback, delay_ms, &mut price_scale_text, "price scale probe");
}

fn schedule_pane_resize(chart: charts::ChartHandle, mut pane_text: Signal<String>, delay_ms: i32) {
    let callback = Closure::<dyn FnMut()>::new(move || {
        let count = match chart.get_pane_count() {
            Ok(count) => count,
            Err(err) => {
                err.with_prefix("Failed to read pane count").log();
                pane_text.set(String::from("Panes: count read failed"));
                return;
            }
        };

        if count < 2 {
            pane_text.set(format!("Panes: resize skipped ({count} pane)"));
            return;
        }

        if let Err(err) = chart.set_pane_stretch_factor(1, 2.0) {
            err.with_prefix("Failed to set pane stretch factor").log();
            pane_text.set(String::from("Panes: resize failed"));
            return;
        }

        update_pane_text(&chart, &mut pane_text, "Panes: resized");
    });

    schedule_timeout(callback, delay_ms, &mut pane_text, "pane resize");
}

fn schedule_pane_swap(chart: charts::ChartHandle, mut pane_text: Signal<String>, delay_ms: i32) {
    let callback = Closure::<dyn FnMut()>::new(move || {
        let count = match chart.get_pane_count() {
            Ok(count) => count,
            Err(err) => {
                err.with_prefix("Failed to read pane count").log();
                pane_text.set(String::from("Panes: count read failed"));
                return;
            }
        };

        if count < 2 {
            pane_text.set(format!("Panes: swap skipped ({count} pane)"));
            return;
        }

        if let Err(err) = chart.swap_panes(0, 1) {
            err.with_prefix("Failed to swap panes").log();
            pane_text.set(String::from("Panes: swap failed"));
            return;
        }

        update_pane_text(&chart, &mut pane_text, "Panes: swapped");
    });

    schedule_timeout(callback, delay_ms, &mut pane_text, "pane swap");
}

fn schedule_pane_report(chart: charts::ChartHandle, mut pane_text: Signal<String>, delay_ms: i32) {
    let callback = Closure::<dyn FnMut()>::new(move || {
        update_pane_text(&chart, &mut pane_text, "Panes");
    });

    schedule_timeout(callback, delay_ms, &mut pane_text, "pane probe");
}

fn schedule_coordinate_probe(
    chart: charts::ChartHandle,
    recent_range: Option<charts::data::TimeRange>,
    mut coordinate_text: Signal<String>,
    delay_ms: i32,
) {
    let callback = Closure::<dyn FnMut()>::new(move || {
        let Some(range) = recent_range else {
            coordinate_text.set(String::from("Coordinates: no visible range"));
            return;
        };

        let time_x = match chart.time_to_coordinate(range.from()) {
            Ok(Some(coordinate)) => coordinate,
            Ok(None) => {
                coordinate_text.set(String::from("Coordinates: time not mapped"));
                return;
            }
            Err(err) => {
                err.with_prefix("Failed to convert time to coordinate").log();
                coordinate_text.set(String::from("Coordinates: time probe failed"));
                return;
            }
        };

        let time_roundtrip = match chart.coordinate_to_time(time_x) {
            Ok(Some(time)) => time,
            Ok(None) => {
                coordinate_text.set(String::from("Coordinates: time roundtrip missing"));
                return;
            }
            Err(err) => {
                err.with_prefix("Failed to convert coordinate to time").log();
                coordinate_text.set(String::from("Coordinates: time roundtrip failed"));
                return;
            }
        };

        let logical_range = match chart.get_visible_logical_range() {
            Ok(Some(range)) => range,
            Ok(None) => {
                coordinate_text.set(String::from("Coordinates: logical range unavailable"));
                return;
            }
            Err(err) => {
                err.with_prefix("Failed to read visible logical range").log();
                coordinate_text.set(String::from("Coordinates: logical range failed"));
                return;
            }
        };

        let logical_mid = (logical_range.from() + logical_range.to()) / 2.0;
        let logical_x = match chart.logical_to_coordinate(logical_mid) {
            Ok(Some(coordinate)) => coordinate,
            Ok(None) => {
                coordinate_text.set(String::from("Coordinates: logical not mapped"));
                return;
            }
            Err(err) => {
                err.with_prefix("Failed to convert logical to coordinate").log();
                coordinate_text.set(String::from("Coordinates: logical probe failed"));
                return;
            }
        };

        let logical_roundtrip = match chart.coordinate_to_logical(logical_x) {
            Ok(Some(logical)) => logical,
            Ok(None) => {
                coordinate_text.set(String::from("Coordinates: logical roundtrip missing"));
                return;
            }
            Err(err) => {
                err.with_prefix("Failed to convert coordinate to logical").log();
                coordinate_text.set(String::from("Coordinates: logical roundtrip failed"));
                return;
            }
        };

        coordinate_text.set(format!(
            "Coordinates: time {:.1} -> {} | logical {:.2} -> {:.1} -> {:.2}",
            time_x, time_roundtrip, logical_mid, logical_x, logical_roundtrip,
        ));
    });

    schedule_timeout(callback, delay_ms, &mut coordinate_text, "coordinate probe");
}

fn schedule_watermark_init(
    chart: charts::ChartHandle,
    mut watermark_text: Signal<String>,
    mut text_watermark_id: Signal<Option<String>>,
    mut image_watermark_id: Signal<Option<String>>,
    delay_ms: i32,
) {
    let callback = Closure::<dyn FnMut()>::new(move || {
        if text_watermark_id().is_some() || image_watermark_id().is_some() {
            watermark_text.set(String::from("Watermarks: already initialized"));
            return;
        }

        let text_options = charts::data::TextWatermarkOptions::new()
            .with_horz_align("center")
            .with_vert_align("center")
            .with_lines(vec![
                charts::data::TextWatermarkLineOptions::new("rs-trading-charts")
                    .with_color("rgba(29, 78, 216, 0.20)")
                    .with_font_size(42.0)
                    .with_font_style("bold"),
                charts::data::TextWatermarkLineOptions::new("Dioxus + Lightweight Charts")
                    .with_color("rgba(2, 132, 199, 0.30)")
                    .with_font_size(20.0)
                    .with_font_family("monospace"),
            ]);
        let image_options = charts::data::ImageWatermarkOptions::new()
            .with_max_width(120.0)
            .with_max_height(120.0)
            .with_padding(16.0)
            .with_alpha(0.55);

        let text_id = match chart.add_text_watermark(0, &text_options) {
            Ok(id) => id,
            Err(err) => {
                err.with_prefix("Failed to add text watermark").log();
                watermark_text.set(String::from("Watermarks: text add failed"));
                return;
            }
        };

        let image_id = match chart.add_image_watermark(1, watermark_svg_data_url(), &image_options) {
            Ok(id) => id,
            Err(err) => {
                err.with_prefix("Failed to add image watermark").log();
                watermark_text.set(String::from("Watermarks: image add failed"));
                return;
            }
        };

        text_watermark_id.set(Some(text_id));
        image_watermark_id.set(Some(image_id));
        watermark_text.set(String::from("Watermarks: initialized"));
    });

    schedule_timeout(callback, delay_ms, &mut watermark_text, "watermark init");
}

fn schedule_watermark_update(
    chart: charts::ChartHandle,
    mut watermark_text: Signal<String>,
    text_watermark_id: Option<String>,
    image_watermark_id: Option<String>,
    delay_ms: i32,
) {
    let callback = Closure::<dyn FnMut()>::new(move || {
        let Some(text_id) = text_watermark_id.clone() else {
            watermark_text.set(String::from("Watermarks: text watermark missing"));
            return;
        };
        let Some(image_id) = image_watermark_id.clone() else {
            watermark_text.set(String::from("Watermarks: image watermark missing"));
            return;
        };

        let text_options = charts::data::TextWatermarkOptions::new()
            .with_horz_align("left")
            .with_vert_align("top")
            .with_lines(vec![
                charts::data::TextWatermarkLineOptions::new("Watermarks updated")
                    .with_color("rgba(15, 23, 42, 0.22)")
                    .with_font_size(28.0)
                    .with_font_style("italic"),
                charts::data::TextWatermarkLineOptions::new("pane primitives")
                    .with_color("rgba(220, 38, 38, 0.22)")
                    .with_font_size(18.0),
            ]);
        let image_options = charts::data::ImageWatermarkOptions::new()
            .with_max_width(96.0)
            .with_max_height(96.0)
            .with_padding(8.0)
            .with_alpha(0.85);

        if let Err(err) = chart.update_text_watermark(text_id, &text_options) {
            err.with_prefix("Failed to update text watermark").log();
            watermark_text.set(String::from("Watermarks: text update failed"));
            return;
        }

        if let Err(err) = chart.update_image_watermark(image_id, &image_options) {
            err.with_prefix("Failed to update image watermark").log();
            watermark_text.set(String::from("Watermarks: image update failed"));
            return;
        }

        watermark_text.set(String::from("Watermarks: updated"));
    });

    schedule_timeout(callback, delay_ms, &mut watermark_text, "watermark update");
}

fn update_price_scale_text(chart: &charts::ChartHandle, price_scale_text: &mut Signal<String>, prefix: &str) {
    match (
        chart.get_price_scale_visible_range("right", Some(0)),
        chart.get_price_scale_width("right", Some(0)),
    ) {
        (Ok(Some(range)), Ok(width)) => {
            price_scale_text.set(format!(
                "{prefix}: {} -> {} (width {:.1})",
                range.from(),
                range.to(),
                width,
            ));
        }
        (Ok(None), Ok(width)) => {
            price_scale_text.set(format!("{prefix}: none (width {:.1})", width));
        }
        (Err(err), _) => {
            err.with_prefix("Failed to read price scale visible range").log();
            price_scale_text.set(String::from("Price scale: range read failed"));
        }
        (_, Err(err)) => {
            err.with_prefix("Failed to read price scale width").log();
            price_scale_text.set(String::from("Price scale: width read failed"));
        }
    }
}

fn update_pane_text(chart: &charts::ChartHandle, pane_text: &mut Signal<String>, prefix: &str) {
    let count = match chart.get_pane_count() {
        Ok(count) => count,
        Err(err) => {
            err.with_prefix("Failed to read pane count").log();
            pane_text.set(String::from("Panes: count read failed"));
            return;
        }
    };

    let panes = (0..count)
        .map(|panel| {
            let size = chart.get_pane_size(panel);
            let stretch = chart.get_pane_stretch_factor(panel);

            match (size, stretch) {
                (Ok(size), Ok(stretch)) => {
                    format!("#{panel}: {:.0}x{:.0} sf {:.2}", size.width(), size.height(), stretch,)
                }
                (Err(err), _) => {
                    err.with_prefix(&format!("Failed to read pane {panel} size")).log();
                    format!("#{panel}: size error")
                }
                (_, Err(err)) => {
                    err.with_prefix(&format!("Failed to read pane {panel} stretch")).log();
                    format!("#{panel}: stretch error")
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" | ");

    pane_text.set(format!("{prefix}: {count} pane(s) [{panes}]"));
}

fn watermark_svg_data_url() -> String {
    String::from(
        "data:image/svg+xml;utf8,\
<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 120 120'>\
<rect width='120' height='120' rx='18' fill='%230f172a' fill-opacity='0.08'/>\
<circle cx='60' cy='60' r='34' fill='%231d4ed8' fill-opacity='0.18'/>\
<path d='M36 66 L54 44 L68 58 L84 38' fill='none' stroke='%230f172a' stroke-width='8' stroke-linecap='round' stroke-linejoin='round'/>\
</svg>",
    )
}

fn schedule_timeout(callback: Closure<dyn FnMut()>, delay_ms: i32, status_text: &mut Signal<String>, context: &str) {
    match window() {
        Some(window) => {
            if let Err(err) = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), delay_ms)
            {
                charts::JsError::from(err)
                    .with_prefix(&format!("Failed to schedule {context}"))
                    .log();
                status_text.set(format!("Scheduling failed ({context})"));
            }
        }
        None => {
            charts::JsError::new_from_str("window is not available")
                .with_prefix(&format!("Failed to schedule {context}"))
                .log();
            status_text.set(format!("Window unavailable ({context})"));
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
