use super::js::TradingChart as JsChart;
use crate::{
    data::{series::Series, options::ChartOptions, LogicalRange, Marker, PriceLineOptions, TimeRange},
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

    pub fn resize(&self, width: f64, height: f64) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.resize(width, height)?)
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

    pub fn update_series_options(&self, series_id: String, options: &impl Serialize) -> Result<(), JsError> {
        let chart = self
            .chart
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        Ok(chart.updateSeriesOptions(series_id, to_value(options)?)?)
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

unsafe impl Send for ChartHandle {}
unsafe impl Sync for ChartHandle {}
