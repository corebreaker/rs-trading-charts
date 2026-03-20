use super::js::TradingChart as JsChart;
use crate::{
    data::{
        series::Series,
        options::{ChartOptions, PriceScaleOptions, TimeScaleOptions},
        Candlestick, HistogramData, ImageWatermarkOptions, LegendOptions, LogicalRange, Marker, PaneSize,
        PriceLineOptions, PriceRange, TextWatermarkOptions, TimeRange, UTCTimestamp, ValueData,
    },
    series::{
        areas::AreaSeriesOptions, bars::BarSeriesOptions, baselines::BaselineSeriesOptions,
        candlesticks::CandlestickOptions, histograms::HistogramSeriesOptions, lines::LineSeriesOptions,
    },
    JsError,
};

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;
use web_sys::HtmlDivElement;
use std::sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    Arc, Mutex,
};

pub type PanelId = u32;

#[derive(Clone)]
pub struct ChartHandle {
    options: Arc<JsValue>,
    chart: Arc<Mutex<JsChart>>,
    bound: Arc<AtomicBool>,
    next_panel: Arc<AtomicU32>,
}

impl ChartHandle {
    pub fn new(options: Option<&ChartOptions>) -> Result<Self, JsError> {
        let options = Arc::new(match options.map(to_value) {
            None => JsValue::NULL,
            Some(Ok(options)) => options,
            Some(Err(err)) => {
                return Err(JsError::from(err));
            }
        });

        Ok(Self {
            options,
            chart: Arc::new(Mutex::new(JsChart::new())),
            bound: Arc::new(AtomicBool::new(false)),
            next_panel: Arc::new(AtomicU32::new(0)),
        })
    }

    pub fn apply_chart_options(&self, options: &ChartOptions) -> Result<(), JsError> {
        if !self.bound.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?
            .applyChartOptions(to_value(options)?)?;

        Ok(())
    }

    pub fn apply_time_scale_options(&self, options: &TimeScaleOptions) -> Result<(), JsError> {
        if !self.bound.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?
            .applyTimeScaleOptions(to_value(options)?)?;

        Ok(())
    }

    pub fn bind(&self, node: HtmlDivElement) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        let options = self.options.as_ref();

        self.bound.store(true, Ordering::SeqCst);
        Ok(chart.bindChart(node, options.clone())?)
    }

    pub fn refit_content(&self) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.refitContent()?)
    }

    pub fn get_visible_range(&self) -> Result<Option<TimeRange>, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        let value = chart.getVisibleRange()?;
        if value.is_null() || value.is_undefined() {
            return Ok(None);
        }

        Ok(Some(from_value(value)?))
    }

    pub fn set_visible_range(&self, range: &TimeRange) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.setVisibleRange(to_value(range)?)?)
    }

    pub fn get_visible_logical_range(&self) -> Result<Option<LogicalRange>, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        let value = chart.getVisibleLogicalRange()?;
        if value.is_null() || value.is_undefined() {
            return Ok(None);
        }

        Ok(Some(from_value(value)?))
    }

    pub fn set_visible_logical_range(&self, range: &LogicalRange) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.setVisibleLogicalRange(to_value(range)?)?)
    }

    pub fn logical_to_coordinate(&self, logical: f64) -> Result<Option<f64>, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        let value = chart.logicalToCoordinate(logical)?;
        if value.is_null() || value.is_undefined() {
            return Ok(None);
        }

        Ok(Some(from_value(value)?))
    }

    pub fn coordinate_to_logical(&self, coordinate: f64) -> Result<Option<f64>, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        let value = chart.coordinateToLogical(coordinate)?;
        if value.is_null() || value.is_undefined() {
            return Ok(None);
        }

        Ok(Some(from_value(value)?))
    }

    pub fn time_to_coordinate(&self, time: UTCTimestamp) -> Result<Option<f64>, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        let value = chart.timeToCoordinate(to_value(&time)?)?;
        if value.is_null() || value.is_undefined() {
            return Ok(None);
        }

        Ok(Some(from_value(value)?))
    }

    pub fn coordinate_to_time(&self, coordinate: f64) -> Result<Option<UTCTimestamp>, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        let value = chart.coordinateToTime(coordinate)?;
        if value.is_null() || value.is_undefined() {
            return Ok(None);
        }

        Ok(Some(from_value(value)?))
    }

    pub fn apply_price_scale_options(
        &self,
        price_scale_id: impl Into<String>,
        panel: Option<PanelId>,
        options: &PriceScaleOptions,
    ) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.applyPriceScaleOptions(
            price_scale_id.into(),
            panel
                .map(|panel| JsValue::from_f64(panel as f64))
                .unwrap_or(JsValue::UNDEFINED),
            to_value(options)?,
        )?)
    }

    pub fn get_price_scale_visible_range(
        &self,
        price_scale_id: impl Into<String>,
        panel: Option<PanelId>,
    ) -> Result<Option<PriceRange>, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        let value = chart.getPriceScaleVisibleRange(
            price_scale_id.into(),
            panel
                .map(|panel| JsValue::from_f64(panel as f64))
                .unwrap_or(JsValue::UNDEFINED),
        )?;
        if value.is_null() || value.is_undefined() {
            return Ok(None);
        }

        Ok(Some(from_value(value)?))
    }

    pub fn set_price_scale_visible_range(
        &self,
        price_scale_id: impl Into<String>,
        panel: Option<PanelId>,
        range: &PriceRange,
    ) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.setPriceScaleVisibleRange(
            price_scale_id.into(),
            panel
                .map(|panel| JsValue::from_f64(panel as f64))
                .unwrap_or(JsValue::UNDEFINED),
            to_value(range)?,
        )?)
    }

    pub fn set_price_scale_auto_scale(
        &self,
        price_scale_id: impl Into<String>,
        panel: Option<PanelId>,
        on: bool,
    ) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.setPriceScaleAutoScale(
            price_scale_id.into(),
            panel
                .map(|panel| JsValue::from_f64(panel as f64))
                .unwrap_or(JsValue::UNDEFINED),
            on,
        )?)
    }

    pub fn get_price_scale_width(
        &self,
        price_scale_id: impl Into<String>,
        panel: Option<PanelId>,
    ) -> Result<f64, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.getPriceScaleWidth(
            price_scale_id.into(),
            panel
                .map(|panel| JsValue::from_f64(panel as f64))
                .unwrap_or(JsValue::UNDEFINED),
        )?)
    }

    pub fn get_pane_count(&self) -> Result<u32, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.getPaneCount()?)
    }

    pub fn get_pane_size(&self, panel: PanelId) -> Result<PaneSize, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(from_value(chart.getPaneSize(panel)?)?)
    }

    pub fn set_pane_height(&self, panel: PanelId, height: f64) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.setPaneHeight(panel, height)?)
    }

    pub fn get_pane_stretch_factor(&self, panel: PanelId) -> Result<f64, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.getPaneStretchFactor(panel)?)
    }

    pub fn set_pane_stretch_factor(&self, panel: PanelId, stretch_factor: f64) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.setPaneStretchFactor(panel, stretch_factor)?)
    }

    pub fn move_pane(&self, panel: PanelId, target: PanelId) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.movePane(panel, target)?)
    }

    pub fn remove_pane(&self, panel: PanelId) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.removePane(panel)?)
    }

    pub fn swap_panes(&self, first: PanelId, second: PanelId) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.swapPanes(first, second)?)
    }

    pub fn resize(&self, width: f64, height: f64) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.resize(width, height)?)
    }

    pub fn take_screenshot_data_url(&self) -> Result<String, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.takeScreenshot()?)
    }

    pub fn apply_legend_options(&self, options: &LegendOptions) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.setLegendOptions(to_value(options)?)?)
    }

    pub fn remove_legend(&self) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.removeLegend()?)
    }

    pub fn add_text_watermark(&self, panel: PanelId, options: &TextWatermarkOptions) -> Result<String, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.addTextWatermark(panel, to_value(options)?)?)
    }

    pub fn update_text_watermark(&self, watermark_id: String, options: &TextWatermarkOptions) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.updateTextWatermark(watermark_id, to_value(options)?)?)
    }

    pub fn add_image_watermark(
        &self,
        panel: PanelId,
        image_url: impl Into<String>,
        options: &ImageWatermarkOptions,
    ) -> Result<String, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.addImageWatermark(panel, image_url.into(), to_value(options)?)?)
    }

    pub fn update_image_watermark(&self, watermark_id: String, options: &ImageWatermarkOptions) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.updateImageWatermark(watermark_id, to_value(options)?)?)
    }

    pub fn remove_watermark(&self, watermark_id: String) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.removeWatermark(watermark_id)?)
    }

    pub fn allocate_panel(&self) -> PanelId {
        self.next_panel.fetch_add(1, Ordering::SeqCst)
    }

    pub fn add_series<Dat, Opt>(&self, series: &mut Series<Dat, Opt>) -> Result<(), JsError>
    where
        Dat: Serialize + Clone,
        Opt: Serialize + Clone,
    {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        let id = chart.addSeries(series.to_value()?)?;

        series.set_id(id);
        Ok(())
    }

    pub fn add_line_series(
        &self,
        data: Vec<ValueData>,
        options: Option<LineSeriesOptions>,
        panel: Option<PanelId>,
    ) -> Result<String, JsError> {
        self.add_typed_series("line", data, options, panel)
    }

    pub fn add_area_series(
        &self,
        data: Vec<ValueData>,
        options: Option<AreaSeriesOptions>,
        panel: Option<PanelId>,
    ) -> Result<String, JsError> {
        self.add_typed_series("area", data, options, panel)
    }

    pub fn add_baseline_series(
        &self,
        data: Vec<ValueData>,
        options: Option<BaselineSeriesOptions>,
        panel: Option<PanelId>,
    ) -> Result<String, JsError> {
        self.add_typed_series("baseline", data, options, panel)
    }

    pub fn add_histogram_series(
        &self,
        data: Vec<HistogramData>,
        options: Option<HistogramSeriesOptions>,
        panel: Option<PanelId>,
    ) -> Result<String, JsError> {
        self.add_typed_series("histogram", data, options, panel)
    }

    pub fn add_candlestick_series(
        &self,
        data: Vec<Candlestick>,
        options: Option<CandlestickOptions>,
        panel: Option<PanelId>,
    ) -> Result<String, JsError> {
        self.add_typed_series("candlestick", data, options, panel)
    }

    pub fn add_bar_series(
        &self,
        data: Vec<Candlestick>,
        options: Option<BarSeriesOptions>,
        panel: Option<PanelId>,
    ) -> Result<String, JsError> {
        self.add_typed_series("bar", data, options, panel)
    }

    pub fn update_series_options(&self, series_id: String, options: &impl Serialize) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.updateSeriesOptions(series_id, to_value(options)?)?)
    }

    pub fn apply_series_price_scale_options(
        &self,
        series_id: String,
        options: &PriceScaleOptions,
    ) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.applySeriesPriceScaleOptions(series_id, to_value(options)?)?)
    }

    pub fn get_series_price_scale_visible_range(&self, series_id: String) -> Result<Option<PriceRange>, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        let value = chart.getSeriesPriceScaleVisibleRange(series_id)?;
        if value.is_null() || value.is_undefined() {
            return Ok(None);
        }

        Ok(Some(from_value(value)?))
    }

    pub fn set_series_price_scale_visible_range(&self, series_id: String, range: &PriceRange) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.setSeriesPriceScaleVisibleRange(series_id, to_value(range)?)?)
    }

    pub fn set_series_price_scale_auto_scale(&self, series_id: String, on: bool) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.setSeriesPriceScaleAutoScale(series_id, on)?)
    }

    pub fn get_series_price_scale_width(&self, series_id: String) -> Result<f64, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.getSeriesPriceScaleWidth(series_id)?)
    }

    pub fn move_series_to_panel(&self, series_id: String, panel: PanelId) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.moveSeriesToPane(series_id, panel)?)
    }

    pub fn get_series_panel(&self, series_id: String) -> Result<PanelId, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.getSeriesPaneIndex(series_id)?)
    }

    pub fn get_series_order(&self, series_id: String) -> Result<u32, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.getSeriesOrder(series_id)?)
    }

    pub fn set_series_order(&self, series_id: String, order: u32) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.setSeriesOrder(series_id, order)?)
    }

    pub fn price_to_coordinate(&self, series_id: String, price: f64) -> Result<Option<f64>, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        let value = chart.priceToCoordinate(series_id, price)?;
        if value.is_null() || value.is_undefined() {
            return Ok(None);
        }

        Ok(Some(from_value(value)?))
    }

    pub fn coordinate_to_price(&self, series_id: String, coordinate: f64) -> Result<Option<f64>, JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        let value = chart.coordinateToPrice(series_id, coordinate)?;
        if value.is_null() || value.is_undefined() {
            return Ok(None);
        }

        Ok(Some(from_value(value)?))
    }

    pub fn set_crosshair_position(
        &self,
        series_id: String,
        price: f64,
        horizontal_position: UTCTimestamp,
    ) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.setCrosshairPosition(series_id, price, to_value(&horizontal_position)?)?)
    }

    pub fn clear_crosshair_position(&self) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.clearCrosshairPosition()?)
    }

    pub fn update_data<Dat>(&self, series_id: String, data: &Vec<Dat>) -> Result<(), JsError>
    where
        Dat: Serialize + Clone,
    {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.updateData(series_id, to_value(data)?)?)
    }

    pub fn update_data_point<Dat>(&self, series_id: String, data: &Dat) -> Result<(), JsError>
    where
        Dat: Serialize + Clone,
    {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.updateDataPoint(series_id, to_value(data)?)?)
    }

    pub fn set_marker(&self, series_id: String, marker: &Marker) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.setMarker(series_id, to_value(marker)?)?)
    }

    pub fn set_markers(&self, series_id: String, markers: &Vec<Marker>) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.setMarkers(series_id, to_value(markers)?)?)
    }

    pub fn set_price_lines(&self, series_id: String, price_lines: &Vec<PriceLineOptions>) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.setPriceLines(series_id, to_value(price_lines)?)?)
    }

    pub fn remove_series(&self, series_id: String) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.removeSeries(series_id)?)
    }
}

impl Drop for ChartHandle {
    fn drop(&mut self) {
        if Arc::strong_count(&self.chart) != 1 {
            return;
        }

        match self.chart.lock() {
            Ok(chart) => {
                chart.destroy();
            }

            Err(err) => {
                JsError::new_from_str(&err.to_string()).log();
            }
        }
    }
}

impl ChartHandle {
    fn add_typed_series<Dat, Opt>(
        &self,
        series_type: &str,
        data: Vec<Dat>,
        options: Option<Opt>,
        panel: Option<PanelId>,
    ) -> Result<String, JsError>
    where
        Dat: Serialize + Clone,
        Opt: Serialize + Clone,
    {
        let mut series: Series<Dat, Opt> = Series::new(series_type);
        if let Some(panel) = panel {
            series.set_panel(panel);
        }
        if let Some(options) = options {
            series.set_options(options);
        }
        series.set_data(data);
        self.add_series(&mut series)?;

        series
            .id()
            .cloned()
            .ok_or_else(|| JsError::new_from_str("Series was added without an id"))
    }
}

unsafe impl Send for ChartHandle {}
unsafe impl Sync for ChartHandle {}
