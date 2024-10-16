mod bindings;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use leptos::{
    tachys::{
        html::{node_ref::NodeRefAttribute, attribute::global::StyleAttribute},
        reactive_graph::node_ref::NodeRef,
    },
    reactive::{effect::Effect, traits::Get},
    html::{ElementChild, Div},
    IntoView,
    component,
    view,
};

#[derive(Serialize, Deserialize)]
struct Candle {
    time:  u64,
    open:  f64,
    high:  f64,
    low:   f64,
    close: f64,
}

#[derive(Serialize, Deserialize)]
struct Background {
    #[serde(rename = "type")]
    r#type: String,
    color:  String,
}

#[derive(Serialize, Deserialize)]
struct Layout {
    #[serde(rename = "textColor")]
    text_color: String,
    background: Background,
}

#[derive(Serialize, Deserialize)]
struct Options {
    layout: Layout,
}

impl Options {
    fn new() -> Self {
        Self {
            layout: Layout {
                text_color: String::from("black"),
                background: Background {
                    r#type: String::from("solid"),
                    color:  String::from("white"),
                },
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Series {
    id:      Option<String>,
    #[serde(rename = "type")]
    r#type:  String,
    options: Option<Options>,
    data:    Vec<Candle>,
}

impl Series {
    //
    fn new() -> Self {
        Self {
            id:      None,
            r#type:  String::from("candlestick"),
            options: None,
            data:    vec![
                Candle {
                    open:  10.,
                    high:  10.63,
                    low:   9.49,
                    close: 9.55,
                    time:  1642427876,
                },
                Candle {
                    open:  9.55,
                    high:  10.30,
                    low:   9.42,
                    close: 9.94,
                    time:  1642514276,
                },
                Candle {
                    open:  9.94,
                    high:  10.17,
                    low:   9.92,
                    close: 9.78,
                    time:  1642600676,
                },
                Candle {
                    open:  9.78,
                    high:  10.59,
                    low:   9.18,
                    close: 9.51,
                    time:  1642687076,
                },
                Candle {
                    open:  9.51,
                    high:  10.46,
                    low:   9.10,
                    close: 10.17,
                    time:  1642773476,
                },
                Candle {
                    open:  10.17,
                    high:  10.96,
                    low:   10.16,
                    close: 10.47,
                    time:  1642859876,
                },
                Candle {
                    open:  10.47,
                    high:  11.39,
                    low:   10.40,
                    close: 10.81,
                    time:  1642946276,
                },
                Candle {
                    open:  10.81,
                    high:  11.60,
                    low:   10.30,
                    close: 10.75,
                    time:  1643032676,
                },
                Candle {
                    open:  10.75,
                    high:  11.60,
                    low:   10.49,
                    close: 10.93,
                    time:  1643119076,
                },
                Candle {
                    open:  10.93,
                    high:  11.53,
                    low:   10.76,
                    close: 10.96,
                    time:  1643205476,
                },
            ],
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Marker {
    time:   u64,
    text:   String,
    #[serde(rename = "type")]
    r#type: String,
}

impl Marker {
    fn buy() -> Self {
        Self {
            time:   1642773476,
            r#type: String::from("buy"),
            text:   String::from("Yes, buy here"),
        }
    }

    fn sell() -> Self {
        Self {
            time:   1643119076,
            r#type: String::from("sell"),
            text:   String::from("Sell here, sure ?"),
        }
    }
}

#[component]
pub fn Chart() -> impl IntoView {
    let chart = bindings::TradingChart::new();
    let node_ref = NodeRef::<Div>::new();
    let _ = Effect::new(move || {
        if let Some(node) = node_ref.get() {
            let series = Series::new();
            let options = Options::new();
            let buyMarker = Marker::buy();
            let sellMarker = Marker::sell();

            chart.bindChart(node, to_value(&options).unwrap()).unwrap();

            let seriesId = chart.addSeries(to_value(&series).unwrap()).unwrap();
            chart.setMarker(seriesId.clone(), to_value(&buyMarker).unwrap()).unwrap();
            chart.setMarker(seriesId.clone(), to_value(&sellMarker).unwrap()).unwrap();
        }
    });

    view! {
        <div style="margin-top:10px;padding:10px">
            <div style="border:1px dashed black;width:1024px;height:500px">
                <div style="width:100%;height:100%" node_ref={node_ref}/>
            </div>
        </div>
    }
}
