use charts::{
    data::{
        options::{background::Background, ChartOptions, LayoutOptions},
        UTCTimestamp,
        Candlestick,
        Marker,
    },
    series::candlesticks::CandleStickSeries,
    JsError,
    Chart,
};

use leptos::{
    tachys::html::attribute::global::StyleAttribute,
    reactive::signal::RwSignal,
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
    for i in 12..=30 {
        let high = open + random() * (12.5 - open);
        let low = 8.5 + random() * (open - 8.5);
        let close = low + random() * (high - low);

        data.push(Candlestick::new(
            UTCTimestamp::from_str(&format!("2024-04-{:02}", i))?,
            open,
            high,
            low,
            close,
        ));
        open = close;
    }

    Ok(data)
}

fn make_markers(data: &Vec<Candlestick>) -> Vec<Marker> {
    let mut markers = vec![];

    if data.len() > 16 {
        markers.push(Marker::buy(data[11].time()).with_text(String::from("Yes, buy here")));
        markers.push(Marker::sell(data[16].time()).with_text(String::from("Sell here, sure ?")));
    }

    markers
}

#[component]
pub fn App() -> impl IntoView {
    let layout = LayoutOptions::new()
        .with_text_color(String::from("black"))
        .with_background(Background::new_solid_color(String::from("white")));

    let options = RwSignal::new(ChartOptions::new().with_layout(layout).with_auto_size(true));

    let data = make_data().unwrap_or_else(|err| {
        err.log();

        vec![]
    });

    let markers = RwSignal::new(make_markers(&data));
    let data = RwSignal::new(data);

    view! {
        <div style="margin-top:10px;padding:10px">
            <div style="border:1px dashed black;height:500px">
                <Chart options=options.read_only().into() style="width:100%;height:100%">
                    <CandleStickSeries data=data.read_only().into() markers=markers.read_only().into() />
                </Chart>
            </div>
        </div>
    }
}
