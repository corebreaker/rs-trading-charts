use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasmbind_macro::wasmbind_dump_js_file_as_inline;
use web_sys::HtmlDivElement;

#[wasmbind_dump_js_file_as_inline(path = "${outDir}/bindings/target/module.mjs")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub(crate) fn registerCustomFunction(kind: &str, name: &str, function: &JsValue) -> Result<(), JsValue>;

    pub(crate) fn unregisterCustomFunction(kind: &str, name: &str) -> bool;

    #[wasm_bindgen(catch)]
    pub(crate) fn loadWebFont(family: &str, source: &str, descriptors: JsValue) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch)]
    pub(crate) fn registerMarkerStyle(id: &str, style: JsValue) -> Result<(), JsValue>;

    pub(crate) fn unregisterMarkerStyle(id: &str) -> bool;

    pub(crate) type TradingChart;

    #[wasm_bindgen(constructor)]
    pub(crate) fn new() -> TradingChart;

    #[wasm_bindgen(method)]
    pub(crate) fn destroy(this: &TradingChart);

    #[wasm_bindgen(method, catch)]
    pub(crate) fn applyChartOptions(this: &TradingChart, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn applyTimeScaleOptions(this: &TradingChart, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn applyPriceScaleOptions(
        this: &TradingChart,
        priceScaleId: &str,
        paneIndex: JsValue,
        options: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn bindChart(this: &TradingChart, node: HtmlDivElement, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setTimeZone(this: &TradingChart, timeZone: Option<String>) -> Result<(), JsValue>;

    #[wasm_bindgen(method)]
    pub(crate) fn getTimeZone(this: &TradingChart) -> Option<String>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn refitContent(this: &TradingChart) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn getVisibleRange(this: &TradingChart) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setVisibleRange(this: &TradingChart, range: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn getVisibleLogicalRange(this: &TradingChart) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setVisibleLogicalRange(this: &TradingChart, range: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn logicalToCoordinate(this: &TradingChart, logical: f64) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn coordinateToLogical(this: &TradingChart, coordinate: f64) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn timeToCoordinate(this: &TradingChart, time: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn coordinateToTime(this: &TradingChart, coordinate: f64) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn getPriceScaleVisibleRange(
        this: &TradingChart,
        priceScaleId: &str,
        paneIndex: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setPriceScaleVisibleRange(
        this: &TradingChart,
        priceScaleId: &str,
        paneIndex: JsValue,
        range: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setPriceScaleAutoScale(
        this: &TradingChart,
        priceScaleId: &str,
        paneIndex: JsValue,
        on: bool,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn getPriceScaleWidth(
        this: &TradingChart,
        priceScaleId: &str,
        paneIndex: JsValue,
    ) -> Result<f64, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn getPaneCount(this: &TradingChart) -> Result<u32, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn getPaneSize(this: &TradingChart, paneIndex: u32) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setPaneHeight(this: &TradingChart, paneIndex: u32, height: f64) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn getPaneStretchFactor(this: &TradingChart, paneIndex: u32) -> Result<f64, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setPaneStretchFactor(this: &TradingChart, paneIndex: u32, stretchFactor: f64) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn movePane(this: &TradingChart, paneIndex: u32, targetIndex: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn removePane(this: &TradingChart, paneIndex: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn swapPanes(this: &TradingChart, first: u32, second: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn resize(this: &TradingChart, width: f64, height: f64) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn takeScreenshot(this: &TradingChart) -> Result<String, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setLegendOptions(this: &TradingChart, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn removeLegend(this: &TradingChart) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn addTextWatermark(this: &TradingChart, paneIndex: u32, options: JsValue) -> Result<String, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn updateTextWatermark(this: &TradingChart, watermarkId: &str, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn addImageWatermark(
        this: &TradingChart,
        paneIndex: u32,
        imageUrl: &str,
        options: JsValue,
    ) -> Result<String, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn updateImageWatermark(this: &TradingChart, watermarkId: &str, options: JsValue)
        -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn removeWatermark(this: &TradingChart, watermarkId: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn addSeries(this: &TradingChart, series: JsValue) -> Result<String, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn updateSeriesOptions(this: &TradingChart, seriesId: &str, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn applySeriesPriceScaleOptions(
        this: &TradingChart,
        seriesId: &str,
        options: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn getSeriesPriceScaleVisibleRange(this: &TradingChart, seriesId: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setSeriesPriceScaleVisibleRange(
        this: &TradingChart,
        seriesId: &str,
        range: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setSeriesPriceScaleAutoScale(this: &TradingChart, seriesId: &str, on: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn getSeriesPriceScaleWidth(this: &TradingChart, seriesId: &str) -> Result<f64, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn moveSeriesToPane(this: &TradingChart, seriesId: &str, paneIndex: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn getSeriesPaneIndex(this: &TradingChart, seriesId: &str) -> Result<u32, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn getSeriesOrder(this: &TradingChart, seriesId: &str) -> Result<u32, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setSeriesOrder(this: &TradingChart, seriesId: &str, order: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn priceToCoordinate(this: &TradingChart, seriesId: &str, price: f64) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn coordinateToPrice(this: &TradingChart, seriesId: &str, coordinate: f64) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setCrosshairPosition(
        this: &TradingChart,
        seriesId: &str,
        price: f64,
        horizontalPosition: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn clearCrosshairPosition(this: &TradingChart) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn removeSeries(this: &TradingChart, seriesId: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn updateData(this: &TradingChart, seriesId: &str, data: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn updateDataPoint(this: &TradingChart, seriesId: &str, data: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setMarker(this: &TradingChart, seriesId: &str, marker: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setMarkers(this: &TradingChart, seriesId: &str, markers: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn applySeriesMarkersOptions(
        this: &TradingChart,
        seriesId: &str,
        options: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(crate) fn setPriceLines(this: &TradingChart, seriesId: &str, priceLines: JsValue) -> Result<(), JsValue>;
}
