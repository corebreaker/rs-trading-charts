use super::{js::TradingChart as JsChart, SeriesHandle, Subscription};
use crate::{
    data::{
        options::{ChartOptions, PriceScaleOptions, TimeScaleOptions},
        series::Series,
        Candlestick,
        HistogramData,
        ImageWatermarkOptions,
        LegendOptions,
        LogicalRange,
        Marker,
        MouseEventParams,
        PaneSize,
        SeriesMarkersOptions,
        Size,
        PriceLineOptions,
        PriceRange,
        TextWatermarkOptions,
        TimeRange,
        UTCTimestamp,
        ValueData,
    },
    series::{
        areas::AreaSeriesOptions,
        bars::BarSeriesOptions,
        baselines::BaselineSeriesOptions,
        candlesticks::CandlestickOptions,
        histograms::HistogramSeriesOptions,
        lines::LineSeriesOptions,
    },
    JsError,
};

use serde::{de::DeserializeOwned, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{closure::Closure, JsValue};
use web_sys::HtmlDivElement;
use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

/// Identifier of a pane (panel) of a chart, which is its index in the chart.
pub type PanelId = u32;

pub(super) struct ChartInner {
    pub(super) js: JsChart,
    options:       RefCell<JsValue>,
    bound:         Cell<bool>,
    next_panel:    Cell<PanelId>,
}

impl Drop for ChartInner {
    fn drop(&mut self) {
        self.js.destroy();
    }
}

/// Handle on a chart, shared by the framework adapters.
///
/// The handle is cheap to clone: all the clones drive the same JS chart, which is destroyed when the last clone is
/// dropped. Series, watermarks and the legend can be added before the chart is bound to a DOM node with
/// [`ChartHandle::bind`]; they are created on binding.
///
/// The JS objects are only reachable from the browser main thread. The handle implements `Send` and `Sync` so that
/// it can be stored in framework contexts, but it must not be used from another thread.
#[derive(Clone)]
pub struct ChartHandle {
    inner: Arc<ChartInner>,
}

// SAFETY: the WebAssembly target the handle is built for is single-threaded, the JS chart is only used from the
// thread which created it.
unsafe impl Send for ChartHandle {}
unsafe impl Sync for ChartHandle {}

pub(crate) fn optional_value<T: DeserializeOwned>(value: JsValue) -> Result<Option<T>, JsError> {
    if value.is_null() || value.is_undefined() {
        return Ok(None);
    }

    Ok(Some(from_value(value)?))
}

fn pane_index(panel: Option<PanelId>) -> JsValue {
    panel
        .map(|panel| JsValue::from_f64(panel as f64))
        .unwrap_or(JsValue::UNDEFINED)
}

impl ChartHandle {
    /// Create a chart which is not yet bound to the DOM.
    pub fn new(options: Option<&ChartOptions>) -> Result<Self, JsError> {
        let options = match options {
            None => JsValue::NULL,
            Some(options) => to_value(options)?,
        };

        Ok(Self {
            inner: Arc::new(ChartInner {
                js:         JsChart::new(),
                options:    RefCell::new(options),
                bound:      Cell::new(false),
                next_panel: Cell::new(0),
            }),
        })
    }

    pub(crate) fn js(&self) -> &JsChart {
        &self.inner.js
    }

    /// Return `true` when both handles drive the same chart.
    pub fn same_chart(&self, other: &ChartHandle) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }

    /// Return `true` when the chart is bound to a DOM node.
    pub fn is_bound(&self) -> bool {
        self.inner.bound.get()
    }

    /// Bind the chart to a DOM node, creating the JS chart and everything added to the handle so far.
    pub fn bind(&self, node: HtmlDivElement) -> Result<(), JsError> {
        let options = self.inner.options.borrow().clone();

        self.js().bindChart(node, options)?;
        self.inner.bound.set(true);

        Ok(())
    }

    // -----------------------------------------------------------------------------------------------------------------
    // Events
    // -----------------------------------------------------------------------------------------------------------------

    fn subscribe<T: DeserializeOwned + 'static>(
        &self,
        kind: &'static str,
        mut callback: impl FnMut(T) + 'static,
    ) -> Result<Subscription, JsError> {
        let closure = Closure::<dyn FnMut(JsValue)>::new(move |value: JsValue| match from_value::<T>(value) {
            Ok(event) => callback(event),
            Err(err) => JsError::from(err).with_prefix(format!("Invalid '{kind}' event")).log(),
        });

        let id = self.js().subscribe(kind, closure.as_ref())?;

        Ok(Subscription::new(Arc::downgrade(&self.inner), id, closure))
    }

    /// Call `callback` when the crosshair moves, including when it leaves the chart.
    pub fn on_crosshair_move(&self, callback: impl FnMut(MouseEventParams) + 'static) -> Result<Subscription, JsError> {
        self.subscribe("crosshairMove", callback)
    }

    /// Call `callback` when the chart is clicked.
    pub fn on_click(&self, callback: impl FnMut(MouseEventParams) + 'static) -> Result<Subscription, JsError> {
        self.subscribe("click", callback)
    }

    /// Call `callback` when the chart is double-clicked.
    pub fn on_dbl_click(&self, callback: impl FnMut(MouseEventParams) + 'static) -> Result<Subscription, JsError> {
        self.subscribe("dblClick", callback)
    }

    /// Call `callback` when the visible time range changes, with `None` when there is no data.
    pub fn on_visible_time_range_change(
        &self,
        callback: impl FnMut(Option<TimeRange>) + 'static,
    ) -> Result<Subscription, JsError> {
        self.subscribe("visibleTimeRangeChange", callback)
    }

    /// Call `callback` when the visible logical range changes, with `None` when there is no data.
    pub fn on_visible_logical_range_change(
        &self,
        callback: impl FnMut(Option<LogicalRange>) + 'static,
    ) -> Result<Subscription, JsError> {
        self.subscribe("visibleLogicalRangeChange", callback)
    }

    /// Call `callback` when the size of the time scale changes.
    pub fn on_size_change(&self, callback: impl FnMut(Size) + 'static) -> Result<Subscription, JsError> {
        self.subscribe("sizeChange", callback)
    }

    // -----------------------------------------------------------------------------------------------------------------
    // Chart and scales
    // -----------------------------------------------------------------------------------------------------------------

    /// Apply chart options.
    ///
    /// Before the chart is bound, the options replace the ones used for the binding.
    pub fn apply_chart_options(&self, options: &ChartOptions) -> Result<(), JsError> {
        let options = to_value(options)?;

        if self.is_bound() {
            self.js().applyChartOptions(options)?;
        } else {
            self.inner.options.replace(options);
        }

        Ok(())
    }

    pub fn apply_time_scale_options(&self, options: &TimeScaleOptions) -> Result<(), JsError> {
        if !self.is_bound() {
            return Ok(());
        }

        Ok(self.js().applyTimeScaleOptions(to_value(options)?)?)
    }

    /// Set the time zone in which the times are displayed, an IANA name such as `Europe/Paris` or `America/New_York`,
    /// or `None` to display the times in UTC, which is the default.
    ///
    /// The data and the times given to and returned by the handle stay real UTC times: the binding converts them to
    /// the wall-clock time of the time zone for the display, so that the time scale, its tick marks and the crosshair
    /// label follow the time zone, including its daylight saving time changes.
    ///
    /// Returns an error when the time zone is unknown.
    pub fn set_time_zone(&self, time_zone: Option<&str>) -> Result<(), JsError> {
        Ok(self.js().setTimeZone(time_zone.map(String::from))?)
    }

    /// The time zone of the chart, `None` when the times are displayed in UTC.
    pub fn time_zone(&self) -> Option<String> {
        self.js().getTimeZone()
    }

    /// Scale the time axis so that all the data fits in the chart.
    pub fn refit_content(&self) -> Result<(), JsError> {
        Ok(self.js().refitContent()?)
    }

    pub fn get_visible_range(&self) -> Result<Option<TimeRange>, JsError> {
        optional_value(self.js().getVisibleRange()?)
    }

    pub fn set_visible_range(&self, range: &TimeRange) -> Result<(), JsError> {
        Ok(self.js().setVisibleRange(to_value(range)?)?)
    }

    pub fn get_visible_logical_range(&self) -> Result<Option<LogicalRange>, JsError> {
        optional_value(self.js().getVisibleLogicalRange()?)
    }

    pub fn set_visible_logical_range(&self, range: &LogicalRange) -> Result<(), JsError> {
        Ok(self.js().setVisibleLogicalRange(to_value(range)?)?)
    }

    pub fn logical_to_coordinate(&self, logical: f64) -> Result<Option<f64>, JsError> {
        optional_value(self.js().logicalToCoordinate(logical)?)
    }

    pub fn coordinate_to_logical(&self, coordinate: f64) -> Result<Option<f64>, JsError> {
        optional_value(self.js().coordinateToLogical(coordinate)?)
    }

    pub fn time_to_coordinate(&self, time: UTCTimestamp) -> Result<Option<f64>, JsError> {
        optional_value(self.js().timeToCoordinate(to_value(&time)?)?)
    }

    pub fn coordinate_to_time(&self, coordinate: f64) -> Result<Option<UTCTimestamp>, JsError> {
        optional_value(self.js().coordinateToTime(coordinate)?)
    }

    pub fn apply_price_scale_options(
        &self,
        price_scale_id: &str,
        panel: Option<PanelId>,
        options: &PriceScaleOptions,
    ) -> Result<(), JsError> {
        Ok(self
            .js()
            .applyPriceScaleOptions(price_scale_id, pane_index(panel), to_value(options)?)?)
    }

    pub fn get_price_scale_visible_range(
        &self,
        price_scale_id: &str,
        panel: Option<PanelId>,
    ) -> Result<Option<PriceRange>, JsError> {
        optional_value(self.js().getPriceScaleVisibleRange(price_scale_id, pane_index(panel))?)
    }

    pub fn set_price_scale_visible_range(
        &self,
        price_scale_id: &str,
        panel: Option<PanelId>,
        range: &PriceRange,
    ) -> Result<(), JsError> {
        Ok(self
            .js()
            .setPriceScaleVisibleRange(price_scale_id, pane_index(panel), to_value(range)?)?)
    }

    pub fn set_price_scale_auto_scale(
        &self,
        price_scale_id: &str,
        panel: Option<PanelId>,
        on: bool,
    ) -> Result<(), JsError> {
        Ok(self
            .js()
            .setPriceScaleAutoScale(price_scale_id, pane_index(panel), on)?)
    }

    pub fn get_price_scale_width(&self, price_scale_id: &str, panel: Option<PanelId>) -> Result<f64, JsError> {
        Ok(self.js().getPriceScaleWidth(price_scale_id, pane_index(panel))?)
    }

    pub fn resize(&self, width: f64, height: f64) -> Result<(), JsError> {
        Ok(self.js().resize(width, height)?)
    }

    /// Take a screenshot of the chart, returned as a PNG data URL.
    pub fn take_screenshot_data_url(&self) -> Result<String, JsError> {
        Ok(self.js().takeScreenshot()?)
    }

    // -----------------------------------------------------------------------------------------------------------------
    // Panes
    // -----------------------------------------------------------------------------------------------------------------

    /// Allocate the identifier of a new pane.
    pub fn allocate_panel(&self) -> PanelId {
        let panel = self.inner.next_panel.get();

        self.inner.next_panel.set(panel + 1);
        panel
    }

    pub fn get_pane_count(&self) -> Result<u32, JsError> {
        Ok(self.js().getPaneCount()?)
    }

    pub fn get_pane_size(&self, panel: PanelId) -> Result<PaneSize, JsError> {
        Ok(from_value(self.js().getPaneSize(panel)?)?)
    }

    pub fn set_pane_height(&self, panel: PanelId, height: f64) -> Result<(), JsError> {
        Ok(self.js().setPaneHeight(panel, height)?)
    }

    pub fn get_pane_stretch_factor(&self, panel: PanelId) -> Result<f64, JsError> {
        Ok(self.js().getPaneStretchFactor(panel)?)
    }

    pub fn set_pane_stretch_factor(&self, panel: PanelId, stretch_factor: f64) -> Result<(), JsError> {
        Ok(self.js().setPaneStretchFactor(panel, stretch_factor)?)
    }

    pub fn move_pane(&self, panel: PanelId, target: PanelId) -> Result<(), JsError> {
        Ok(self.js().movePane(panel, target)?)
    }

    pub fn remove_pane(&self, panel: PanelId) -> Result<(), JsError> {
        Ok(self.js().removePane(panel)?)
    }

    pub fn swap_panes(&self, first: PanelId, second: PanelId) -> Result<(), JsError> {
        Ok(self.js().swapPanes(first, second)?)
    }

    // -----------------------------------------------------------------------------------------------------------------
    // Legend and watermarks
    // -----------------------------------------------------------------------------------------------------------------

    pub fn apply_legend_options(&self, options: &LegendOptions) -> Result<(), JsError> {
        Ok(self.js().setLegendOptions(to_value(options)?)?)
    }

    pub fn remove_legend(&self) -> Result<(), JsError> {
        Ok(self.js().removeLegend()?)
    }

    /// Add a text watermark on a pane, and return its identifier.
    pub fn add_text_watermark(&self, panel: PanelId, options: &TextWatermarkOptions) -> Result<String, JsError> {
        Ok(self.js().addTextWatermark(panel, to_value(options)?)?)
    }

    pub fn update_text_watermark(&self, watermark_id: &str, options: &TextWatermarkOptions) -> Result<(), JsError> {
        Ok(self.js().updateTextWatermark(watermark_id, to_value(options)?)?)
    }

    /// Add an image watermark on a pane, and return its identifier.
    pub fn add_image_watermark(
        &self,
        panel: PanelId,
        image_url: &str,
        options: &ImageWatermarkOptions,
    ) -> Result<String, JsError> {
        Ok(self.js().addImageWatermark(panel, image_url, to_value(options)?)?)
    }

    pub fn update_image_watermark(&self, watermark_id: &str, options: &ImageWatermarkOptions) -> Result<(), JsError> {
        Ok(self.js().updateImageWatermark(watermark_id, to_value(options)?)?)
    }

    pub fn remove_watermark(&self, watermark_id: &str) -> Result<(), JsError> {
        Ok(self.js().removeWatermark(watermark_id)?)
    }

    // -----------------------------------------------------------------------------------------------------------------
    // Series
    // -----------------------------------------------------------------------------------------------------------------

    /// Add a series described by `series`, and return a handle on it.
    pub fn add_series<Dat, Opt>(&self, series: &Series<Dat, Opt>) -> Result<SeriesHandle<Dat, Opt>, JsError>
    where
        Dat: Serialize + Clone,
        Opt: Serialize + Clone, {
        let id = self.js().addSeries(series.to_value()?)?;

        Ok(SeriesHandle::new(self.clone(), id))
    }

    pub fn add_line_series(
        &self,
        data: Vec<ValueData>,
        options: Option<LineSeriesOptions>,
        panel: Option<PanelId>,
    ) -> Result<SeriesHandle<ValueData, LineSeriesOptions>, JsError> {
        self.add_typed_series("line", data, options, panel)
    }

    pub fn add_area_series(
        &self,
        data: Vec<ValueData>,
        options: Option<AreaSeriesOptions>,
        panel: Option<PanelId>,
    ) -> Result<SeriesHandle<ValueData, AreaSeriesOptions>, JsError> {
        self.add_typed_series("area", data, options, panel)
    }

    pub fn add_baseline_series(
        &self,
        data: Vec<ValueData>,
        options: Option<BaselineSeriesOptions>,
        panel: Option<PanelId>,
    ) -> Result<SeriesHandle<ValueData, BaselineSeriesOptions>, JsError> {
        self.add_typed_series("baseline", data, options, panel)
    }

    pub fn add_histogram_series(
        &self,
        data: Vec<HistogramData>,
        options: Option<HistogramSeriesOptions>,
        panel: Option<PanelId>,
    ) -> Result<SeriesHandle<HistogramData, HistogramSeriesOptions>, JsError> {
        self.add_typed_series("histogram", data, options, panel)
    }

    pub fn add_candlestick_series(
        &self,
        data: Vec<Candlestick>,
        options: Option<CandlestickOptions>,
        panel: Option<PanelId>,
    ) -> Result<SeriesHandle<Candlestick, CandlestickOptions>, JsError> {
        self.add_typed_series("candlestick", data, options, panel)
    }

    pub fn add_bar_series(
        &self,
        data: Vec<Candlestick>,
        options: Option<BarSeriesOptions>,
        panel: Option<PanelId>,
    ) -> Result<SeriesHandle<Candlestick, BarSeriesOptions>, JsError> {
        self.add_typed_series("bar", data, options, panel)
    }

    fn add_typed_series<Dat, Opt>(
        &self,
        series_type: &str,
        data: Vec<Dat>,
        options: Option<Opt>,
        panel: Option<PanelId>,
    ) -> Result<SeriesHandle<Dat, Opt>, JsError>
    where
        Dat: Serialize + Clone,
        Opt: Serialize + Clone, {
        let mut series: Series<Dat, Opt> = Series::new(series_type);
        if let Some(panel) = panel {
            series.set_panel(panel);
        }
        if let Some(options) = options {
            series.set_options(options);
        }

        series.set_data(data);
        self.add_series(&series)
    }

    /// Remove a series from the chart. Removing an unknown series does nothing.
    pub fn remove_series(&self, series_id: &str) -> Result<(), JsError> {
        Ok(self.js().removeSeries(series_id)?)
    }

    pub fn update_series_options(&self, series_id: &str, options: &impl Serialize) -> Result<(), JsError> {
        Ok(self.js().updateSeriesOptions(series_id, to_value(options)?)?)
    }

    /// Replace all the data of a series.
    pub fn update_data<Dat: Serialize>(&self, series_id: &str, data: &[Dat]) -> Result<(), JsError> {
        Ok(self.js().updateData(series_id, to_value(data)?)?)
    }

    /// Add or replace a single data item of a series.
    pub fn update_data_point<Dat: Serialize>(&self, series_id: &str, data: &Dat) -> Result<(), JsError> {
        Ok(self.js().updateDataPoint(series_id, to_value(data)?)?)
    }

    /// Add, replace or remove a single marker of a series.
    ///
    /// A marker with an identifier replaces the marker with the same identifier; a marker without identifier
    /// replaces the markers without identifier at the same time. A [`Marker::remove`] removes them.
    pub fn set_marker(&self, series_id: &str, marker: &Marker) -> Result<(), JsError> {
        Ok(self.js().setMarker(series_id, to_value(marker)?)?)
    }

    /// Replace all the markers of a series. Several markers can share the same time.
    pub fn set_markers(&self, series_id: &str, markers: &[Marker]) -> Result<(), JsError> {
        Ok(self.js().setMarkers(series_id, to_value(markers)?)?)
    }

    pub fn apply_series_markers_options(&self, series_id: &str, options: &SeriesMarkersOptions) -> Result<(), JsError> {
        Ok(self.js().applySeriesMarkersOptions(series_id, to_value(options)?)?)
    }

    pub fn set_price_lines(&self, series_id: &str, price_lines: &[PriceLineOptions]) -> Result<(), JsError> {
        Ok(self.js().setPriceLines(series_id, to_value(price_lines)?)?)
    }

    pub fn apply_series_price_scale_options(
        &self,
        series_id: &str,
        options: &PriceScaleOptions,
    ) -> Result<(), JsError> {
        Ok(self.js().applySeriesPriceScaleOptions(series_id, to_value(options)?)?)
    }

    pub fn get_series_price_scale_visible_range(&self, series_id: &str) -> Result<Option<PriceRange>, JsError> {
        optional_value(self.js().getSeriesPriceScaleVisibleRange(series_id)?)
    }

    pub fn set_series_price_scale_visible_range(&self, series_id: &str, range: &PriceRange) -> Result<(), JsError> {
        Ok(self.js().setSeriesPriceScaleVisibleRange(series_id, to_value(range)?)?)
    }

    pub fn set_series_price_scale_auto_scale(&self, series_id: &str, on: bool) -> Result<(), JsError> {
        Ok(self.js().setSeriesPriceScaleAutoScale(series_id, on)?)
    }

    pub fn get_series_price_scale_width(&self, series_id: &str) -> Result<f64, JsError> {
        Ok(self.js().getSeriesPriceScaleWidth(series_id)?)
    }

    pub fn move_series_to_panel(&self, series_id: &str, panel: PanelId) -> Result<(), JsError> {
        Ok(self.js().moveSeriesToPane(series_id, panel)?)
    }

    pub fn get_series_panel(&self, series_id: &str) -> Result<PanelId, JsError> {
        Ok(self.js().getSeriesPaneIndex(series_id)?)
    }

    pub fn get_series_order(&self, series_id: &str) -> Result<u32, JsError> {
        Ok(self.js().getSeriesOrder(series_id)?)
    }

    pub fn set_series_order(&self, series_id: &str, order: u32) -> Result<(), JsError> {
        Ok(self.js().setSeriesOrder(series_id, order)?)
    }

    pub fn price_to_coordinate(&self, series_id: &str, price: f64) -> Result<Option<f64>, JsError> {
        optional_value(self.js().priceToCoordinate(series_id, price)?)
    }

    pub fn coordinate_to_price(&self, series_id: &str, coordinate: f64) -> Result<Option<f64>, JsError> {
        optional_value(self.js().coordinateToPrice(series_id, coordinate)?)
    }

    // -----------------------------------------------------------------------------------------------------------------
    // Crosshair
    // -----------------------------------------------------------------------------------------------------------------

    /// Move the crosshair to a price of a series at a given time.
    pub fn set_crosshair_position(
        &self,
        series_id: &str,
        price: f64,
        horizontal_position: UTCTimestamp,
    ) -> Result<(), JsError> {
        Ok(self
            .js()
            .setCrosshairPosition(series_id, price, to_value(&horizontal_position)?)?)
    }

    pub fn clear_crosshair_position(&self) -> Result<(), JsError> {
        Ok(self.js().clearCrosshairPosition()?)
    }
}
