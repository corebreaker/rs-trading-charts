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
    pub(super) fn applyTimeScaleOptions(this: &TradingChart, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn applyPriceScaleOptions(
        this: &TradingChart,
        priceScaleId: String,
        paneIndex: JsValue,
        options: JsValue,
    ) -> Result<(), JsValue>;

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
    pub(super) fn logicalToCoordinate(this: &TradingChart, logical: f64) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn coordinateToLogical(this: &TradingChart, coordinate: f64) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn timeToCoordinate(this: &TradingChart, time: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn coordinateToTime(this: &TradingChart, coordinate: f64) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn getPriceScaleVisibleRange(
        this: &TradingChart,
        priceScaleId: String,
        paneIndex: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setPriceScaleVisibleRange(
        this: &TradingChart,
        priceScaleId: String,
        paneIndex: JsValue,
        range: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setPriceScaleAutoScale(
        this: &TradingChart,
        priceScaleId: String,
        paneIndex: JsValue,
        on: bool,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn getPriceScaleWidth(
        this: &TradingChart,
        priceScaleId: String,
        paneIndex: JsValue,
    ) -> Result<f64, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn getPaneCount(this: &TradingChart) -> Result<u32, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn getPaneSize(this: &TradingChart, paneIndex: u32) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setPaneHeight(this: &TradingChart, paneIndex: u32, height: f64) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn getPaneStretchFactor(this: &TradingChart, paneIndex: u32) -> Result<f64, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setPaneStretchFactor(this: &TradingChart, paneIndex: u32, stretchFactor: f64) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn movePane(this: &TradingChart, paneIndex: u32, targetIndex: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn removePane(this: &TradingChart, paneIndex: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn swapPanes(this: &TradingChart, first: u32, second: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn resize(this: &TradingChart, width: f64, height: f64) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn takeScreenshot(this: &TradingChart) -> Result<String, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setLegendOptions(this: &TradingChart, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn removeLegend(this: &TradingChart) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn addTextWatermark(this: &TradingChart, paneIndex: u32, options: JsValue) -> Result<String, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn updateTextWatermark(
        this: &TradingChart,
        watermarkId: String,
        options: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn addImageWatermark(
        this: &TradingChart,
        paneIndex: u32,
        imageUrl: String,
        options: JsValue,
    ) -> Result<String, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn updateImageWatermark(
        this: &TradingChart,
        watermarkId: String,
        options: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn removeWatermark(this: &TradingChart, watermarkId: String) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn addSeries(this: &TradingChart, series: JsValue) -> Result<String, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn updateSeriesOptions(this: &TradingChart, seriesId: String, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn applySeriesPriceScaleOptions(
        this: &TradingChart,
        seriesId: String,
        options: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn getSeriesPriceScaleVisibleRange(this: &TradingChart, seriesId: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setSeriesPriceScaleVisibleRange(
        this: &TradingChart,
        seriesId: String,
        range: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setSeriesPriceScaleAutoScale(this: &TradingChart, seriesId: String, on: bool) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn getSeriesPriceScaleWidth(this: &TradingChart, seriesId: String) -> Result<f64, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn moveSeriesToPane(this: &TradingChart, seriesId: String, paneIndex: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn getSeriesPaneIndex(this: &TradingChart, seriesId: String) -> Result<u32, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn getSeriesOrder(this: &TradingChart, seriesId: String) -> Result<u32, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setSeriesOrder(this: &TradingChart, seriesId: String, order: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn priceToCoordinate(this: &TradingChart, seriesId: String, price: f64) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn coordinateToPrice(this: &TradingChart, seriesId: String, coordinate: f64)
        -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn setCrosshairPosition(
        this: &TradingChart,
        seriesId: String,
        price: f64,
        horizontalPosition: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub(super) fn clearCrosshairPosition(this: &TradingChart) -> Result<(), JsValue>;

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
