use charts::{
    data::{
        options::{
            background::Background,
            layout::{LayoutPanesOptions, LayoutOptions},
            ChartOptions,
        },
        UTCTimestamp,
        Candlestick,
        Marker,
    },
    series::candlesticks::CandleStickSeries,
    chart::Chart,
    panel::ChartPanel,
    JsError,
};

use leptos::{
    tachys::html::attribute::global::{StyleAttribute, OnAttribute},
    reactive::{signal::RwSignal, traits::{Update, With}, wrappers::read::Signal},
    html::ElementChild,
    IntoView,
    component,
    view,
};

use wasm_bindgen::prelude::wasm_bindgen;
use std::str::FromStr;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

struct Data {
    data_up: Vec<Candlestick>,
    data_down: Vec<Candlestick>,
    markers: Vec<Marker>,
    counter: usize,
}

impl Data {
    fn new() -> Self {
        let data_up = make_dataset();
        let data_down = make_dataset();
        let markers = make_markers(&data_up, 0);
        let counter = 0;

        Self {
            data_up,
            data_down,
            markers,
            counter,
        }
    }
    
    fn inc(&mut self) {
        self.counter = (self.counter + 1) % 16;
        self.markers = make_markers(&self.data_down, self.counter);
        self.data_down = make_dataset();
    }
}

fn make_data() -> Result<Vec<Candlestick>, JsError> {
    let mut data = vec![
        Candlestick::new(UTCTimestamp::from_str("2024-04-01")?, 10., 10.63, 9.49, 9.55),
        Candlestick::new(UTCTimestamp::from_str("2024-04-02")?, 9.55, 10.30, 9.42, 9.94),
        Candlestick::new(UTCTimestamp::from_str("2024-04-03")?, 9.94, 10.17, 9.92, 9.78),
        Candlestick::new(UTCTimestamp::from_str("2024-04-04")?, 9.78, 10.59, 9.18, 9.51),
        Candlestick::new(UTCTimestamp::from_str("2024-04-05")?, 9.51, 10.46, 9.10, 10.17),
        Candlestick::new(UTCTimestamp::from_str("2024-04-06")?, 10.17, 10.96, 10.16, 10.47),
        Candlestick::new(UTCTimestamp::from_str("2024-04-07")?, 10.47, 11.39, 10.40, 10.81),
        Candlestick::new(UTCTimestamp::from_str("2024-04-08")?, 10.81, 11.60, 10.30, 10.75),
        Candlestick::new(UTCTimestamp::from_str("2024-04-09")?, 10.75, 11.60, 10.49, 10.93),
        Candlestick::new(UTCTimestamp::from_str("2024-04-10")?, 10.93, 11.53, 10.76, 10.96),
        Candlestick::new(UTCTimestamp::from_str("2024-04-11")?, 10.96, 11.00, 10.50, 10.55),
    ];

    let mut open = 10.55;
    let mut i = 12usize;
    for m in 4usize..7 {
        let s = i;
        i = 1;

        for d in s..=30 {
            let high = open + random() * (12.5 - open);
            let low = 8.5 + random() * (open - 8.5);
            let close = low + random() * (high - low);

            data.push(Candlestick::new(
                UTCTimestamp::from_str(&format!("2024-{m:02}-{d:02}"))?,
                open,
                high,
                low,
                close,
            ));
            open = close;
        }
    }

    Ok(data)
}

fn make_markers(data: &Vec<Candlestick>, delta: usize) -> Vec<Marker> {
    let mut markers = vec![];

    markers.push(Marker::buy(data[8 + delta].time()).with_text(String::from("B0")));
    markers.push(Marker::buy(data[11 + delta].time()).with_text(String::from("Yes, buy here")));
    markers.push(Marker::sell(data[16 + delta].time()).with_text(String::from("Sell here, sure ?")));
    markers.push(Marker::buy(data[19 + delta].time()).with_text(String::from("B1")));
    markers.push(Marker::sell(data[23 + delta].time()).with_text(String::from("S1")));
    markers.push(Marker::sell(data[26 + delta].time()).with_text(String::from("S2")));
    markers.push(Marker::sell(data[32 + delta].time()).with_text(String::from("S3")));
    markers.push(Marker::buy(data[35 + delta].time()).with_text(String::from("B2")));
    markers.push(Marker::buy(data[40 + delta].time()).with_text(String::from("B3")));
    markers.push(Marker::buy(data[42 + delta].time()).with_text(String::from("B4")));
    markers.push(Marker::sell(data[47 + delta].time()).with_text(String::from("S4")));
    markers.push(Marker::buy(data[48 + delta].time()).with_text(String::from("B5")));
    markers.push(Marker::sell(data[53 + delta].time()).with_text(String::from("S5")));
    markers.push(Marker::buy(data[60 + delta].time()).with_text(String::from("B6")));
    markers.push(Marker::buy(data[65 + delta].time()).with_text(String::from("B7")));
    markers.push(Marker::buy(data[68 + delta].time()).with_text(String::from("B7")));
    markers.push(Marker::sell(data[70 + delta].time()).with_text(String::from("S6")));
    markers.push(Marker::sell(data[72 + delta].time()).with_text(String::from("S7")));

    markers
}

fn make_dataset() -> Vec<Candlestick> {
    make_data().unwrap_or_else(|err| {
        err.log();

        vec![]
    })
}

#[component]
pub fn App() -> impl IntoView {
    let layout = LayoutOptions::new()
        .with_text_color(String::from("black"))
        .with_background(Background::new_solid_color(String::from("white")))
        .with_panes(LayoutPanesOptions::new());

    let options = RwSignal::new(ChartOptions::new().with_layout(layout).with_auto_size(true));
    let data = RwSignal::new(Data::new());

    view! {
        <div style="margin-top:10px;padding:10px">
            <div style="border:1px dashed black;height:768px">
                <Chart options=options.read_only() style="width:100%;height:100%">
                    <ChartPanel>
                        <CandleStickSeries
                            data=Signal::derive(move || data.with(|d| d.data_up.clone()))
                            markers=Signal::derive(move || data.with(|d| d.markers.clone()))
                        />
                    </ChartPanel>
                    <ChartPanel>
                        <CandleStickSeries
                            data=Signal::derive(move || data.with(|d| d.data_down.clone()))
                            markers=Signal::derive(move || data.with(|d| d.markers.clone()))
                        />
                    </ChartPanel>
                </Chart>
            </div>
        
            <div style="margin-top:10px">
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
        </div>
    }
}
