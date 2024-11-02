use super::js::TradingChart as JsChart;
use crate::{data::{series::Series, options::ChartOptions, Candlestick, Marker}, JsError};
use serde_wasm_bindgen::to_value;
use serde::Serialize;
use wasm_bindgen::JsValue;
use web_sys::HtmlDivElement;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
    Mutex,
};

#[derive(Clone)]
pub struct TradingChartBinding {
    options: Arc<JsValue>,
    chart:   Arc<Mutex<JsChart>>,
    bound:   Arc<AtomicBool>,
}

impl TradingChartBinding {
    pub(crate) fn new(options: Option<&ChartOptions>) -> Result<Self, JsError> {
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
        })
    }

    pub(crate) fn apply_chart_options(&self, options: &ChartOptions) -> Result<(), JsError> {
        if !self.bound.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.chart.lock().unwrap().applyCharOptions(to_value(options)?)?;

        Ok(())
    }

    pub(crate) fn bind_chart(&self, node: HtmlDivElement) -> Result<(), JsError> {
        let chart = self.chart.lock().unwrap();
        let options = self.options.as_ref();

        self.bound.store(true, Ordering::SeqCst);
        Ok(chart.bindChart(node, options.clone())?)
    }

    pub(crate) fn add_series<Dat, Opt>(&self, series: &mut Series<Dat, Opt>) -> Result<(), JsError>
    where
        Dat: Serialize + Clone,
        Opt: Serialize + Clone, {
        let chart = self.chart.lock().unwrap();
        let id = chart.addSeries(series.to_value()?)?;

        series.set_id(id);
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn update_series_options(&self, series_id: String, options: &impl Serialize) -> Result<(), JsError> {
        let chart = self.chart.lock().unwrap();

        Ok(chart.updateSeriesOptions(series_id, to_value(options)?)?)
    }

    #[allow(dead_code)]
    pub(crate) fn update_data(&self, series_id: String, data: &Vec<Candlestick>) -> Result<(), JsError> {
        let chart = self.chart.lock().unwrap();

        Ok(chart.updateData(series_id, to_value(data)?)?)
    }

    #[allow(dead_code)]
    pub(crate) fn set_marker(&self, series_id: String, marker: &Marker) -> Result<(), JsError> {
        let chart = self.chart.lock().unwrap();

        Ok(chart.setMarker(series_id, to_value(marker)?)?)
    }

    #[allow(dead_code)]
    pub(crate) fn set_markers(&self, series_id: String, markers: &Vec<Marker>) -> Result<(), JsError> {
        let chart = self.chart.lock().unwrap();

        Ok(chart.setMarkers(series_id, to_value(markers)?)?)
    }
}

impl Drop for TradingChartBinding {
    fn drop(&mut self) {
        self.chart.lock().unwrap().destroy();
    }
}

unsafe impl Send for TradingChartBinding {}
unsafe impl Sync for TradingChartBinding {}
