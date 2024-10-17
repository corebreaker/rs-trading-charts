use super::data::ChartOptions;
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

use crate::bindings::TradingChartBinding;
use crate::data::{series::Series, Background, LayoutOptions, Marker};

fn new_options() -> ChartOptions {
    let layout = LayoutOptions::new()
        .with_text_color(String::from("black"))
        .with_background(Background::new_solid_color(String::from("white")));

    ChartOptions::new().with_layout(layout)
}

#[component]
pub fn Chart() -> impl IntoView {
    let options = new_options();
    let chart = TradingChartBinding::new(&options).unwrap();
    let node_ref = NodeRef::<Div>::new();
    let _ = Effect::new({
        move || {
            if let Some(node) = node_ref.get() {
                let series = Series::sample();
                let buyMarker = Marker::buy(String::from("2024-02-12")).with_text(String::from("Yes, buy here"));
                let sellMarker = Marker::sell(String::from("2024-02-17")).with_text(String::from("Sell here, sure ?"));

                chart.bind_chart(node).unwrap();

                let seriesId = chart.add_series(&series).unwrap();
                chart.set_marker(seriesId.clone(), &buyMarker).unwrap();
                chart.set_marker(seriesId.clone(), &sellMarker).unwrap();
            }
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
