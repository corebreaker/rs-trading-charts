use super::{js::TradingChart as JsChart, error::JsError};
use crate::data::{series::Series, ChartOptions, Candle, Marker};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use web_sys::HtmlDivElement;

pub struct TradingChartBinding {
    options: JsValue,
    chart: JsChart,
}

impl TradingChartBinding {
    pub(crate) fn new(options: &ChartOptions) -> Result<Self, JsError> {
        Ok(Self {
            options: to_value(options)?,
            chart: JsChart::new(),
        })
    }

    pub(crate) fn bind_chart(&self, node: HtmlDivElement) -> Result<(), JsError> {
        Ok(self.chart.bindChart(node, self.options.clone())?)
    }

    pub(crate) fn add_series(&self, series: &Series) -> Result<String, JsValue> {
        Ok(self.chart.addSeries(series.to_value()?)?)
    }

    pub(crate) fn update_options(&self, series_id: String, options: &ChartOptions) -> Result<(), JsValue> {
        Ok(self.chart.updateOptions(series_id, to_value(options)?)?)
    }

    pub(crate) fn update_data(&self, series_id: String, data: &Vec<Candle>) -> Result<(), JsValue> {
        Ok(self.chart.updateData(series_id, to_value(data)?)?)
    }

    pub(crate) fn set_marker(&self, series_id: String, marker: &Marker) -> Result<(), JsValue> {
        Ok(self.chart.setMarker(series_id, to_value(marker)?)?)
    }

    pub(crate) fn set_markers(&self, series_id: String, markers: &Vec<Marker>) -> Result<(), JsValue> {
        Ok(self.chart.setMarkers(series_id, to_value(markers)?)?)
    }
}

impl Drop for TradingChartBinding {
    fn drop(&mut self) {
        self.chart.destroy();
    }
}
