use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasmbind_macro::wasmbind_dump_js_file_as_inline;
use web_sys::HtmlDivElement;

#[wasmbind_dump_js_file_as_inline(path = "${outDir}/bindings/target/module.mjs")]
extern "C" {
    pub(super) type TradingChart;

    #[wasm_bindgen(constructor)]
    pub(super) fn new() -> TradingChart;

    #[wasm_bindgen(method)]
    pub(super) fn destroy(this: &TradingChart);

    #[wasm_bindgen(method, catch)]
    pub(super) fn applyChartOptions(this: &TradingChart, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn bindChart(this: &TradingChart, node: HtmlDivElement, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn refitContent(this: &TradingChart) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn addPanel(this: &TradingChart) -> Result<(), JsValue>;

    #[wasm_bindgen(method)]
    pub(super) fn removePanel(this: &TradingChart);

    #[wasm_bindgen(method, catch)]
    pub(super) fn addSeries(this: &TradingChart, series: JsValue) -> Result<String, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn updateSeriesOptions(this: &TradingChart, seriesId: String, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn removeSeries(this: &TradingChart, seriesId: String) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn updateData(this: &TradingChart, seriesId: String, data: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setMarker(this: &TradingChart, seriesId: String, marker: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setMarkers(this: &TradingChart, seriesId: String, markers: JsValue) -> Result<(), JsValue>;
}
