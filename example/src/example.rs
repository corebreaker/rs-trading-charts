use super::dataset::Dataset;
use charts::{
    data::options::{
        background::Background,
        layout::{LayoutOptions, LayoutPanesOptions},
        ChartOptions, TimeScaleOptions,
    },
    chart::Chart,
    panel::ChartPanel,
    series::{
        areas::{AreaSeries, AreaSeriesOptions},
        bars::{BarSeries, BarSeriesOptions},
        candlesticks::CandleStickSeries,
        histograms::{HistogramSeries, HistogramSeriesOptions},
        lines::{LineSeries, LineSeriesOptions},
    },
    REFIT_EVENT_KIND,
};

use emitix::{leptos::LeptosEventChannels, EventManager};
use leptos::{
    tachys::html::attribute::global::{StyleAttribute, OnAttribute},
    reactive::{
        signal::{RwSignal, signal},
        traits::{Update, With},
        wrappers::read::Signal,
    },
    html::ElementChild,
    IntoView, component, view,
};

use log::error;

#[component]
pub fn App() -> impl IntoView {
    let refit_event = LeptosEventChannels::default();
    let refit_emitter = refit_event.new_emitter(REFIT_EVENT_KIND);
    let (chart_options, _) = signal(
        ChartOptions::new()
            .with_time_scale(TimeScaleOptions::new().with_time_visible(true))
            .with_layout(
                LayoutOptions::new()
                    .with_background(Background::new_solid_color(String::from("white")))
                    .with_panes(LayoutPanesOptions::new())
                    .with_text_color(String::from("black")),
            )
            .with_auto_size(true),
    );
    let (line_options, _) = signal(
        LineSeriesOptions::new()
            .with_title(String::from("Close"))
            .with_color(String::from("#1d4ed8"))
            .with_price_line_visible(false),
    );
    let (area_options, _) = signal(
        AreaSeriesOptions::new()
            .with_title(String::from("Mid"))
            .with_top_color(String::from("rgba(14, 165, 233, 0.28)"))
            .with_bottom_color(String::from("rgba(14, 165, 233, 0.02)"))
            .with_line_color(String::from("rgba(2, 132, 199, 0.95)"))
            .with_price_line_visible(false),
    );
    let (bar_options, _) = signal(
        BarSeriesOptions::new()
            .with_title(String::from("Bar"))
            .with_open_visible(true)
            .with_price_line_visible(false),
    );
    let (histogram_options, _) = signal(
        HistogramSeriesOptions::new()
            .with_title(String::from("Delta"))
            .with_base(0.0)
            .with_price_line_visible(false),
    );

    let data = RwSignal::new(Dataset::new());

    view! {
        <div style="margin-top:10px;padding:10px">
            <div style="border:1px dashed black;height:768px">
                <Chart options=chart_options style="width:100%;height:100%" refit=refit_event.clone()>
                    <ChartPanel>
                        <CandleStickSeries
                            data=Signal::derive(move || data.with(|d| d.data_up().clone()))
                            markers=Signal::derive(move || data.with(|d| d.markers().clone()))
                        />
                        <LineSeries
                            options=line_options
                            data=Signal::derive(move || data.with(|d| d.line_up()))
                            markers=Signal::derive(Vec::new)
                        />
                        <AreaSeries
                            options=area_options
                            data=Signal::derive(move || data.with(|d| d.area_up()))
                            markers=Signal::derive(Vec::new)
                        />
                    </ChartPanel>
                    <ChartPanel>
                        <BarSeries
                            options=bar_options
                            data=Signal::derive(move || data.with(|d| d.data_down().clone()))
                            markers=Signal::derive(move || data.with(|d| d.markers().clone()))
                        />
                        <HistogramSeries
                            options=histogram_options
                            data=Signal::derive(move || data.with(|d| d.histogram_down()))
                            markers=Signal::derive(Vec::new)
                        />
                    </ChartPanel>
                </Chart>
            </div>

            <div style="margin-top:10px; display: flex; flex-direction: row; column-gap: 10px;">
                <div>
                    <button
                        on:click=move |_| {
                            data.update(|d| {
                                d.inc();
                            });
                        }
                    >
                        "Change markers"
                    </button>
                </div>
                <div>
                    <button
                        on:click=move |_| {
                            data.maybe_update(|d| {
                                match Dataset::load("data1") {
                                    Ok(new_data) => {
                                        *d = new_data;
                                        true
                                    }
                                    Err(err) => {
                                        error!("Failed to load dataset 1: {err}");
                                        false
                                    }
                                }
                            });
                        }
                    >
                        "Load dataset 1"
                    </button>
                </div>
                <div>
                    <button
                        on:click=move |_| {
                            data.maybe_update(|d| {
                                match Dataset::load("data2") {
                                    Ok(new_data) => {
                                        *d = new_data;
                                        true
                                    }
                                    Err(err) => {
                                        error!("Failed to load dataset 2: {err}");
                                        false
                                    }
                                }
                            });
                        }
                    >
                        "Load dataset 2"
                    </button>
                </div>
                <div>
                    <button
                        on:click=move |_| {
                            if let Err(err) = refit_emitter.emit(()) {
                                error!("Failed to refit chart content: {err}");
                            }
                        }
                    >
                        "Refit Content"
                    </button>
                </div>
            </div>
        </div>
    }
}
