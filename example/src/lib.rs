use charts::{
    data::{
        options::{background::Background, ChartOptions, LayoutOptions},
        Candlestick,
        Marker,
    },
    series::candlesticks::CandleStickSeries,
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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[component]
pub fn App() -> impl IntoView {
    let layout = LayoutOptions::new()
        .with_text_color(String::from("black"))
        .with_background(Background::new_solid_color(String::from("white")));

    let options = RwSignal::new(ChartOptions::new().with_layout(layout));

    let buy_marker = Marker::buy(String::from("2024-02-12")).with_text(String::from("Yes, buy here"));
    let sell_marker = Marker::sell(String::from("2024-02-17")).with_text(String::from("Sell here, sure ?"));
    let markers = RwSignal::new(vec![buy_marker, sell_marker]);

    let mut data = vec![
        Candlestick::new(String::from("2024-02-01"), 10., 10.63, 9.49, 9.55),
        Candlestick::new(String::from("2024-02-02"), 9.55, 10.30, 9.42, 9.94),
        Candlestick::new(String::from("2024-02-03"), 9.94, 10.17, 9.92, 9.78),
        Candlestick::new(String::from("2024-02-04"), 9.78, 10.59, 9.18, 9.51),
        Candlestick::new(String::from("2024-02-05"), 9.51, 10.46, 9.10, 10.17),
        Candlestick::new(String::from("2024-02-06"), 10.17, 10.96, 10.16, 10.47),
        Candlestick::new(String::from("2024-02-07"), 10.47, 11.39, 10.40, 10.81),
        Candlestick::new(String::from("2024-02-08"), 10.81, 11.60, 10.30, 10.75),
        Candlestick::new(String::from("2024-02-09"), 10.75, 11.60, 10.49, 10.93),
        Candlestick::new(String::from("2024-02-10"), 10.93, 11.53, 10.76, 10.96),
        Candlestick::new(String::from("2024-02-11"), 10.96, 11.00, 10.50, 10.55),
    ];

    let mut open = 10.55;
    for i in 12..=30 {
        let high = open + random() * (12.5 - open);
        let low = 8.5 + random() * (open - 8.5);
        let close = low + random() * (high - low);

        data.push(Candlestick::new(format!("2024-02-{:02}", i), open, high, low, close));
        open = close;
    }

    let data = RwSignal::new(data);

    view! {
        <div style="margin-top:10px;padding:10px">
            <div style="border:1px dashed black;width:1024px;height:500px">
                <Chart options=options.read_only() style="width:100%;height:100%">
                    <CandleStickSeries data=data.read_only() markers=markers.read_only() />
                </Chart>
            </div>
        </div>
    }
}
