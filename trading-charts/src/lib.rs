use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlDivElement;
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

#[wasm_bindgen(module = "/bindings/module.mjs")]
extern "C" {
    type TradingChart;

    #[wasm_bindgen(constructor)]
    fn new() -> TradingChart;

    #[wasm_bindgen(method)]
    fn bindChart(this: &TradingChart, node: HtmlDivElement);

    #[wasm_bindgen(method)]
    fn show(this: &TradingChart) -> String;
}

#[component]
pub fn Chart() -> impl IntoView {
    let chart = TradingChart::new();
    let value = chart.show();
    let node_ref = NodeRef::<Div>::new();
    let _ = Effect::new(move || {
        if let Some(node) = node_ref.get() {
            chart.bindChart(node);
        }
    });

    view! {
        <p>Value: <span style="margin-left:10px">{value}</span></p>
        <div style="margin-top:10px;padding:10px">
            <div style="border:1px dashed black;width:1024px;height:500px">
                <div style="width:100%;height:100%" node_ref={node_ref}/>
            </div>
        </div>
    }
}
