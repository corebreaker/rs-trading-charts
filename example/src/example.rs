use super::dataset::Dataset;
use charts::{
    data::{
        events::EventManager,
        options::{
            background::Background,
            layout::{LayoutOptions, LayoutPanesOptions},
            TimeScaleOptions,
            ChartOptions,
        },
    },
    series::candlesticks::CandleStickSeries,
    panel::ChartPanel,
    chart::Chart,
};

use leptos::{
    tachys::html::attribute::global::{StyleAttribute, OnAttribute},
    reactive::{
        signal::{RwSignal, signal},
        traits::{Update, With},
        wrappers::read::Signal,
    },
    html::ElementChild,
    IntoView,
    component,
    view,
};

use log::error;

#[component]
pub fn App() -> impl IntoView {
    let refit_event = EventManager::<()>::new();
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
                    </ChartPanel>
                    <ChartPanel>
                        <CandleStickSeries
                            data=Signal::derive(move || data.with(|d| d.data_down().clone()))
                            markers=Signal::derive(move || data.with(|d| d.markers().clone()))
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
                            if let Err(err) = refit_event.emit(()) {
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
