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
    pub(super) fn getVisibleRange(this: &TradingChart) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setVisibleRange(this: &TradingChart, range: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn getVisibleLogicalRange(this: &TradingChart) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setVisibleLogicalRange(this: &TradingChart, range: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn resize(this: &TradingChart, width: f64, height: f64) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn addSeries(this: &TradingChart, series: JsValue) -> Result<String, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn updateSeriesOptions(this: &TradingChart, seriesId: String, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn removeSeries(this: &TradingChart, seriesId: String) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn updateData(this: &TradingChart, seriesId: String, data: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn updateDataPoint(this: &TradingChart, seriesId: String, data: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setMarker(this: &TradingChart, seriesId: String, marker: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setMarkers(this: &TradingChart, seriesId: String, markers: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setPriceLines(this: &TradingChart, seriesId: String, priceLines: JsValue) -> Result<(), JsValue>;
}
