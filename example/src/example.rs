use super::dataset::Dataset;
use charts::{
    chart::Chart,
    data::{
        LegendOptions,
        PriceLineOptions,
        options::{
            background::Background,
            cross_hair::{CrossHairOptions, CrosshairLineOptions},
            layout::{LayoutOptions, LayoutPanesOptions},
            ChartOptions,
            LastPriceAnimationMode,
            LineType,
            LineWidth,
            PriceFormatOptions,
            PriceScaleOptions,
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
use leptos::{
    context::use_context,
    html::ElementChild,
    reactive::{
        effect::Effect,
        signal::RwSignal,
        traits::{Get, Set, Update, With, WithUntracked},
        wrappers::read::Signal,
    },
    tachys::html::attribute::global::{OnAttribute, StyleAttribute},
    IntoView,
    component,
    view,
};
use log::error;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::window;

#[component]
pub fn App() -> impl IntoView {
    let chart_options = ChartOptions::new()
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
        .with_auto_size(true);
    let legend_options = Some(
        LegendOptions::new()
            .with_text_color("#0f172a")
            .with_background_color("rgba(255, 255, 255, 0.82)")
            .with_font_size(12.0)
            .with_top(10.0)
            .with_left(10.0),
    );
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

    let line_price_lines = Signal::derive(|| {
        vec![PriceLineOptions::new(10.5)
            .with_color(String::from("#1d4ed8"))
            .with_title(String::from("Close line"))]
    });
    let bar_price_lines = Signal::derive(|| {
        vec![PriceLineOptions::new(10.0)
            .with_color(String::from("#dc2626"))
            .with_title(String::from("Bar ref"))]
    });

    let data = RwSignal::new(Dataset::new());
    let range_text = RwSignal::new(String::from("Visible range: pending"));
    let price_scale_text = RwSignal::new(String::from("Price scale: pending"));
    let pane_text = RwSignal::new(String::from("Panes: pending"));
    let coordinate_text = RwSignal::new(String::from("Coordinates: pending"));
    let watermark_text = RwSignal::new(String::from("Watermarks: pending"));
    let direct_series_text = RwSignal::new(String::from("Direct series: idle"));
    let screenshot_status = RwSignal::new(String::from("Screenshot: idle"));
    let screenshot_data_url = RwSignal::new(None::<String>);
    let zoom_request = RwSignal::new(0_u64);
    let auto_request = RwSignal::new(0_u64);
    let resize_request = RwSignal::new(0_u64);
    let swap_request = RwSignal::new(0_u64);
    let coordinate_request = RwSignal::new(0_u64);
    let watermark_request = RwSignal::new(0_u64);
    let direct_add_request = RwSignal::new(0_u64);
    let direct_remove_request = RwSignal::new(0_u64);

    let recent_range = Signal::derive(move || data.with(|dataset| dataset.recent_range()));
    let direct_line_data = Signal::derive(move || data.with(|dataset| dataset.line_up()));

    view! {
        <div style="margin-top:10px;padding:10px">
            <h1>"Leptos trading charts example"</h1>
            <div style="border:1px dashed black;height:768px">
                <Chart options=chart_options.clone() legend=legend_options.clone() style="width:100%;height:100%">
                    <ChartPanel>
                        <CandleStickSeries
                            data=Signal::derive(move || data.with(|dataset| dataset.data_up().clone()))
                            markers=Signal::derive(move || data.with(|dataset| dataset.markers().clone()))
                        />
                        <LineSeries
                            options=line_options.clone()
                            data=Signal::derive(move || data.with(|dataset| dataset.line_up()))
                            markers=Signal::derive(Vec::new)
                            price_lines=line_price_lines
                        />
                        <AreaSeries
                            options=area_options.clone()
                            data=Signal::derive(move || data.with(|dataset| dataset.area_up()))
                            markers=Signal::derive(Vec::new)
                        />
                        <BaselineSeries
                            options=baseline_options.clone()
                            data=Signal::derive(move || data.with(|dataset| dataset.line_up()))
                            markers=Signal::derive(Vec::new)
                            price_lines=Signal::derive(Vec::new)
                        />
                    </ChartPanel>
                    <ChartPanel>
                        <BarSeries
                            options=bar_options.clone()
                            data=Signal::derive(move || data.with(|dataset| dataset.data_down().clone()))
                            markers=Signal::derive(move || data.with(|dataset| dataset.markers().clone()))
                            price_lines=bar_price_lines
                        />
                        <HistogramSeries
                            options=histogram_options.clone()
                            data=Signal::derive(move || data.with(|dataset| dataset.histogram_down()))
                            markers=Signal::derive(Vec::new)
                        />
                    </ChartPanel>
                    <ChartEffects
                        recent_range=recent_range
                        line_data=direct_line_data
                        range_text=range_text
                        price_scale_text=price_scale_text
                        pane_text=pane_text
                        coordinate_text=coordinate_text
                        watermark_text=watermark_text
                        direct_series_text=direct_series_text
                        screenshot_status=screenshot_status
                        screenshot_data_url=screenshot_data_url
                        zoom_request=zoom_request
                        auto_request=auto_request
                        resize_request=resize_request
                        swap_request=swap_request
                        coordinate_request=coordinate_request
                        watermark_request=watermark_request
                        direct_add_request=direct_add_request
                        direct_remove_request=direct_remove_request
                    />
                </Chart>
            </div>

            <div style="margin-top:10px;display:flex;flex-direction:column;row-gap:10px;">
                <span>{move || range_text.get()}</span>
                <span>{move || price_scale_text.get()}</span>
                <span>{move || pane_text.get()}</span>
                <span>{move || coordinate_text.get()}</span>
                <span>{move || watermark_text.get()}</span>
                <span>{move || direct_series_text.get()}</span>
                <span>{move || screenshot_status.get()}</span>
                <div style="display:flex;flex-direction:row;column-gap:10px;">
                    <button on:click=move |_| data.update(|dataset| dataset.inc())>
                        "Change markers"
                    </button>
                    <button on:click=move |_| {
                        data.update(|dataset| {
                            match Dataset::load("data1") {
                                Ok(new_data) => *dataset = new_data,
                                Err(err) => error!("Failed to load dataset 1: {err}"),
                            }
                        });
                    }>
                        "Load dataset 1"
                    </button>
                    <button on:click=move |_| {
                        data.update(|dataset| {
                            match Dataset::load("data2") {
                                Ok(new_data) => *dataset = new_data,
                                Err(err) => error!("Failed to load dataset 2: {err}"),
                            }
                        });
                    }>
                        "Load dataset 2"
                    </button>
                    <button on:click=move |_| zoom_request.update(|request| *request += 1)>
                        "Zoom price scale"
                    </button>
                    <button on:click=move |_| auto_request.update(|request| *request += 1)>
                        "Auto price scale"
                    </button>
                    <button on:click=move |_| resize_request.update(|request| *request += 1)>
                        "Resize lower pane"
                    </button>
                    <button on:click=move |_| swap_request.update(|request| *request += 1)>
                        "Swap panes"
                    </button>
                    <button on:click=move |_| coordinate_request.update(|request| *request += 1)>
                        "Probe coordinates"
                    </button>
                    <button on:click=move |_| watermark_request.update(|request| *request += 1)>
                        "Update watermarks"
                    </button>
                    <button on:click=move |_| direct_add_request.update(|request| *request += 1)>
                        "Add direct line"
                    </button>
                    <button on:click=move |_| direct_remove_request.update(|request| *request += 1)>
                        "Remove direct line"
                    </button>
                </div>
                <img
                    src=move || screenshot_data_url.get().unwrap_or_default()
                    style=move || {
                        if screenshot_data_url.get().is_some() {
                            String::from("max-width:280px;border:1px solid #94a3b8;")
                        } else {
                            String::from("display:none")
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component(transparent)]
fn ChartEffects(
    recent_range: Signal<Option<charts::data::TimeRange>>,
    line_data: Signal<Vec<charts::data::ValueData>>,
    range_text: RwSignal<String>,
    price_scale_text: RwSignal<String>,
    pane_text: RwSignal<String>,
    coordinate_text: RwSignal<String>,
    watermark_text: RwSignal<String>,
    direct_series_text: RwSignal<String>,
    screenshot_status: RwSignal<String>,
    screenshot_data_url: RwSignal<Option<String>>,
    zoom_request: RwSignal<u64>,
    auto_request: RwSignal<u64>,
    resize_request: RwSignal<u64>,
    swap_request: RwSignal<u64>,
    coordinate_request: RwSignal<u64>,
    watermark_request: RwSignal<u64>,
    direct_add_request: RwSignal<u64>,
    direct_remove_request: RwSignal<u64>,
) -> impl IntoView {
    let chart: Option<charts::ChartHandle> = use_context();
    let Some(chart) = chart else {
        return view! {};
    };

    let initialized = RwSignal::new(false);
    let last_applied_range = RwSignal::new(None::<charts::data::TimeRange>);
    let last_zoom_request = RwSignal::new(0_u64);
    let last_auto_request = RwSignal::new(0_u64);
    let last_resize_request = RwSignal::new(0_u64);
    let last_swap_request = RwSignal::new(0_u64);
    let last_coordinate_request = RwSignal::new(0_u64);
    let last_watermark_request = RwSignal::new(0_u64);
    let last_direct_add_request = RwSignal::new(0_u64);
    let last_direct_remove_request = RwSignal::new(0_u64);
    let text_watermark_id = RwSignal::new(None::<String>);
    let image_watermark_id = RwSignal::new(None::<String>);
    let direct_series_id = RwSignal::new(None::<String>);

    Effect::new({
        let chart = chart.clone();
        move || {
            let Some(range) = recent_range.get() else {
                range_text.set(String::from("Visible range: unavailable"));
                return;
            };

            if last_applied_range.get() == Some(range) {
                return;
            }

            last_applied_range.set(Some(range));
            schedule_visible_range_probe(chart.clone(), range, range_text, 6);
        }
    });

    Effect::new({
        let chart = chart.clone();
        move || {
            if initialized.get() {
                return;
            }

            initialized.set(true);
            schedule_price_scale_report(chart.clone(), price_scale_text, String::from("Price scale"), 150);
            schedule_pane_report(chart.clone(), pane_text, 150);
            schedule_coordinate_probe(chart.clone(), recent_range.get(), coordinate_text, 150);
            schedule_watermark_init(
                chart.clone(),
                watermark_text,
                text_watermark_id,
                image_watermark_id,
                150,
            );
            schedule_screenshot_capture(chart.clone(), screenshot_status, screenshot_data_url, 800);
        }
    });

    Effect::new({
        let chart = chart.clone();
        move || {
            let request = zoom_request.get();
            if request == 0 || request == last_zoom_request.get() {
                return;
            }

            last_zoom_request.set(request);
            schedule_price_scale_zoom(chart.clone(), price_scale_text, 150);
        }
    });

    Effect::new({
        let chart = chart.clone();
        move || {
            let request = auto_request.get();
            if request == 0 || request == last_auto_request.get() {
                return;
            }

            last_auto_request.set(request);
            schedule_price_scale_auto(chart.clone(), price_scale_text, 150);
        }
    });

    Effect::new({
        let chart = chart.clone();
        move || {
            let request = resize_request.get();
            if request == 0 || request == last_resize_request.get() {
                return;
            }

            last_resize_request.set(request);
            schedule_pane_resize(chart.clone(), pane_text, 150);
        }
    });

    Effect::new({
        let chart = chart.clone();
        move || {
            let request = swap_request.get();
            if request == 0 || request == last_swap_request.get() {
                return;
            }

            last_swap_request.set(request);
            schedule_pane_swap(chart.clone(), pane_text, 150);
        }
    });

    Effect::new({
        let chart = chart.clone();
        move || {
            let request = coordinate_request.get();
            if request == 0 || request == last_coordinate_request.get() {
                return;
            }

            last_coordinate_request.set(request);
            schedule_coordinate_probe(chart.clone(), recent_range.get(), coordinate_text, 150);
        }
    });

    Effect::new({
        let chart = chart.clone();
        move || {
            let request = watermark_request.get();
            if request == 0 || request == last_watermark_request.get() {
                return;
            }

            last_watermark_request.set(request);
            schedule_watermark_update(
                chart.clone(),
                watermark_text,
                text_watermark_id.get(),
                image_watermark_id.get(),
                150,
            );
        }
    });

    Effect::new({
        let chart = chart.clone();
        move || {
            let request = direct_add_request.get();
            if request == 0 || request == last_direct_add_request.get() {
                return;
            }

            last_direct_add_request.set(request);

            if let Some(series_id) = direct_series_id.get() {
                let data = line_data.get();
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

            match chart.add_line_series(line_data.get(), Some(options), Some(0)) {
                Ok(series_id) => {
                    direct_series_id.set(Some(series_id.clone()));
                    direct_series_text.set(format!("Direct series: added {}", series_id));
                }
                Err(err) => {
                    err.with_prefix("Failed to add direct line series").log();
                    direct_series_text.set(String::from("Direct series: add failed"));
                }
            }
        }
    });

    Effect::new({
        let chart = chart.clone();
        move || {
            let request = direct_remove_request.get();
            if request == 0 || request == last_direct_remove_request.get() {
                return;
            }

            last_direct_remove_request.set(request);

            let Some(series_id) = direct_series_id.get() else {
                direct_series_text.set(String::from("Direct series: nothing to remove"));
                return;
            };

            match chart.remove_series(series_id.clone()) {
                Ok(()) => {
                    direct_series_id.set(None);
                    direct_series_text.set(format!("Direct series: removed {}", series_id));
                }
                Err(err) => {
                    err.with_prefix("Failed to remove direct line series").log();
                    direct_series_text.set(String::from("Direct series: remove failed"));
                }
            }
        }
    });

    Effect::new({
        let chart = chart.clone();
        move || {
            let Some(series_id) = direct_series_id.get() else {
                return;
            };

            let data = line_data.get();
            if let Err(err) = chart.update_data(series_id.clone(), &data) {
                err.with_prefix("Failed to update direct line series data").log();
                direct_series_text.set(String::from("Direct series: data sync failed"));
                return;
            }

            direct_series_text.set(format!("Direct series: synced {}", series_id));
        }
    });

    view! {}
}

fn schedule_visible_range_probe(
    chart: charts::ChartHandle,
    range: charts::data::TimeRange,
    range_text: RwSignal<String>,
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

fn schedule_price_scale_zoom(chart: charts::ChartHandle, price_scale_text: RwSignal<String>, delay_ms: i32) {
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

        update_price_scale_text(&chart, price_scale_text, "Price scale: zoomed");
    });

    schedule_timeout(callback, delay_ms, price_scale_text, "price scale zoom");
}

fn schedule_price_scale_auto(chart: charts::ChartHandle, price_scale_text: RwSignal<String>, delay_ms: i32) {
    let callback = Closure::<dyn FnMut()>::new(move || {
        if let Err(err) = chart.set_price_scale_auto_scale("right", Some(0), true) {
            err.with_prefix("Failed to enable price scale autoscale").log();
            price_scale_text.set(String::from("Price scale: autoscale failed"));
            return;
        }

        update_price_scale_text(&chart, price_scale_text, "Price scale: auto");
    });

    schedule_timeout(callback, delay_ms, price_scale_text, "price scale autoscale");
}

fn schedule_price_scale_report(
    chart: charts::ChartHandle,
    price_scale_text: RwSignal<String>,
    label: String,
    delay_ms: i32,
) {
    let callback = Closure::<dyn FnMut()>::new(move || {
        update_price_scale_text(&chart, price_scale_text, &label);
    });

    schedule_timeout(callback, delay_ms, price_scale_text, "price scale probe");
}

fn schedule_pane_resize(chart: charts::ChartHandle, pane_text: RwSignal<String>, delay_ms: i32) {
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

        update_pane_text(&chart, pane_text, "Panes: resized");
    });

    schedule_timeout(callback, delay_ms, pane_text, "pane resize");
}

fn schedule_pane_swap(chart: charts::ChartHandle, pane_text: RwSignal<String>, delay_ms: i32) {
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

        update_pane_text(&chart, pane_text, "Panes: swapped");
    });

    schedule_timeout(callback, delay_ms, pane_text, "pane swap");
}

fn schedule_pane_report(chart: charts::ChartHandle, pane_text: RwSignal<String>, delay_ms: i32) {
    let callback = Closure::<dyn FnMut()>::new(move || {
        update_pane_text(&chart, pane_text, "Panes");
    });

    schedule_timeout(callback, delay_ms, pane_text, "pane probe");
}

fn schedule_coordinate_probe(
    chart: charts::ChartHandle,
    recent_range: Option<charts::data::TimeRange>,
    coordinate_text: RwSignal<String>,
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

    schedule_timeout(callback, delay_ms, coordinate_text, "coordinate probe");
}

fn schedule_watermark_init(
    chart: charts::ChartHandle,
    watermark_text: RwSignal<String>,
    text_watermark_id: RwSignal<Option<String>>,
    image_watermark_id: RwSignal<Option<String>>,
    delay_ms: i32,
) {
    let callback = Closure::<dyn FnMut()>::new(move || {
        if text_watermark_id.with_untracked(|id| id.is_some()) || image_watermark_id.with_untracked(|id| id.is_some()) {
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
                charts::data::TextWatermarkLineOptions::new("Leptos + Lightweight Charts")
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

    schedule_timeout(callback, delay_ms, watermark_text, "watermark init");
}

fn schedule_watermark_update(
    chart: charts::ChartHandle,
    watermark_text: RwSignal<String>,
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

    schedule_timeout(callback, delay_ms, watermark_text, "watermark update");
}

fn update_price_scale_text(chart: &charts::ChartHandle, price_scale_text: RwSignal<String>, prefix: &str) {
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

fn update_pane_text(chart: &charts::ChartHandle, pane_text: RwSignal<String>, prefix: &str) {
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
                    format!("#{panel}: {:.0}x{:.0} sf {:.2}", size.width(), size.height(), stretch)
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
    String::from(concat!(
        "data:image/svg+xml;utf8,",
        "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 120 120'>",
        "<rect width='120' height='120' rx='18' fill='%230f172a' fill-opacity='0.08'/>",
        "<circle cx='60' cy='60' r='34' fill='%231d4ed8' fill-opacity='0.18'/>",
        "<path d='M36 66 L54 44 L68 58 L84 38' fill='none' stroke='%230f172a' stroke-width='8' ",
        "stroke-linecap='round' stroke-linejoin='round'/>",
        "</svg>",
    ))
}

fn schedule_timeout(callback: Closure<dyn FnMut()>, delay_ms: i32, status_text: RwSignal<String>, context: &str) {
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

fn schedule_screenshot_capture(
    chart: charts::ChartHandle,
    screenshot_status: RwSignal<String>,
    screenshot_data_url: RwSignal<Option<String>>,
    delay_ms: i32,
) {
    let callback = Closure::<dyn FnMut()>::new(move || match chart.take_screenshot_data_url() {
        Ok(data_url) => {
            screenshot_status.set(format!("Screenshot: captured {} chars", data_url.len()));
            screenshot_data_url.set(Some(data_url));
        }
        Err(err) => {
            err.with_prefix("Failed to capture screenshot").log();
            screenshot_status.set(String::from("Screenshot: capture failed"));
        }
    });

    schedule_timeout(callback, delay_ms, screenshot_status, "screenshot capture");
}
