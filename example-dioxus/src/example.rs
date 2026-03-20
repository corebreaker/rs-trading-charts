use super::dataset::Dataset;
use charts::{
    chart::{Chart, use_chart},
    data::options::{
        background::Background,
        layout::{LayoutOptions, LayoutPanesOptions},
        ChartOptions, TimeScaleOptions,
    },
    panel::ChartPanel,
    series::candlesticks::CandleStickSeries,
};
use dioxus::prelude::*;
use log::error;

pub fn app() -> Element {
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
                    }
                    ChartPanel {
                        CandleStickSeries {
                            data: data.read().data_down().clone(),
                            markers: data.read().markers().clone(),
                        }
                    }
                    ChartActions {}
                }
            }

            div {
                style: "margin-top:10px;display:flex;flex-direction:row;column-gap:10px;",
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

#[allow(non_snake_case)]
fn ChartActions() -> Element {
    let chart = use_chart();

    rsx! {
        div {
            style: "margin-top:10px;display:flex;column-gap:10px;",
            button {
                onclick: move |_| {
                    if let Err(err) = chart.refit_content() {
                        err.with_prefix("Failed to refit chart content").log();
                    }
                },
                "Refit content"
            }
        }
    }
}
