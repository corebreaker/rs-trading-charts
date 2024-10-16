use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::HtmlDivElement;

#[wasm_bindgen(module = "/bindings/module.mjs")]
extern "C" {
    pub(crate) type TradingChart;

    #[wasm_bindgen(constructor)]
    pub(crate) fn new() -> TradingChart;

    #[wasm_bindgen(method)]
    pub(crate) fn destroy(this: &TradingChart);

    #[wasm_bindgen(method, catch)]
    pub(crate) fn bindChart(this: &TradingChart, node: HtmlDivElement, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn addSeries(this: &TradingChart, series: JsValue) -> Result<String, JsValue>;

    #[wasm_bindgen(method)]
    pub(crate) fn removeSeries(this: &TradingChart, seriesId: String) -> bool;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn updateOptions(this: &TradingChart, seriesId: String, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn updateData(this: &TradingChart, seriesId: String, data: Vec<JsValue>) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setMarker(this: &TradingChart, seriesId: String, marker: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setMarkers(this: &TradingChart, seriesId: String, markers: Vec<JsValue>) -> Result<(), JsValue>;
}
