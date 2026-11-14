use super::{ChartHandle, PanelId};
use crate::{
    data::{options::PriceScaleOptions, Marker, PriceLineOptions, PriceRange, SeriesMarkersOptions},
    JsError,
};

use serde::Serialize;
use std::marker::PhantomData;

/// Handle on a series added to a chart.
///
/// `Dat` is the type of the data items and `Opt` the type of the options of the series. The handle is cheap to clone.
/// Dropping it does not remove the series: call [`SeriesHandle::remove`] to do so.
pub struct SeriesHandle<Dat, Opt> {
    chart:   ChartHandle,
    id:      String,
    _marker: PhantomData<fn() -> (Dat, Opt)>,
}

impl<Dat, Opt> Clone for SeriesHandle<Dat, Opt> {
    fn clone(&self) -> Self {
        Self {
            chart:   self.chart.clone(),
            id:      self.id.clone(),
            _marker: PhantomData,
        }
    }
}

impl<Dat, Opt> PartialEq for SeriesHandle<Dat, Opt> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.chart.same_chart(&other.chart)
    }
}

impl<Dat: Serialize, Opt: Serialize> SeriesHandle<Dat, Opt> {
    pub(crate) fn new(chart: ChartHandle, id: String) -> Self {
        Self {
            chart,
            id,
            _marker: PhantomData,
        }
    }

    /// Identifier of the series in the chart.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The chart the series belongs to.
    pub fn chart(&self) -> &ChartHandle {
        &self.chart
    }

    /// Remove the series from the chart.
    pub fn remove(&self) -> Result<(), JsError> {
        self.chart.remove_series(&self.id)
    }

    pub fn apply_options(&self, options: &Opt) -> Result<(), JsError> {
        self.chart.update_series_options(&self.id, options)
    }

    /// Replace all the data of the series.
    pub fn set_data(&self, data: &[Dat]) -> Result<(), JsError> {
        self.chart.update_data(&self.id, data)
    }

    /// Add or replace a single data item, which is efficient for the last item (real-time updates).
    pub fn update(&self, data: &Dat) -> Result<(), JsError> {
        self.chart.update_data_point(&self.id, data)
    }

    pub fn set_marker(&self, marker: &Marker) -> Result<(), JsError> {
        self.chart.set_marker(&self.id, marker)
    }

    pub fn set_markers(&self, markers: &[Marker]) -> Result<(), JsError> {
        self.chart.set_markers(&self.id, markers)
    }

    pub fn apply_markers_options(&self, options: &SeriesMarkersOptions) -> Result<(), JsError> {
        self.chart.apply_series_markers_options(&self.id, options)
    }

    pub fn set_price_lines(&self, price_lines: &[PriceLineOptions]) -> Result<(), JsError> {
        self.chart.set_price_lines(&self.id, price_lines)
    }

    pub fn apply_price_scale_options(&self, options: &PriceScaleOptions) -> Result<(), JsError> {
        self.chart.apply_series_price_scale_options(&self.id, options)
    }

    pub fn get_price_scale_visible_range(&self) -> Result<Option<PriceRange>, JsError> {
        self.chart.get_series_price_scale_visible_range(&self.id)
    }

    pub fn set_price_scale_visible_range(&self, range: &PriceRange) -> Result<(), JsError> {
        self.chart.set_series_price_scale_visible_range(&self.id, range)
    }

    pub fn set_price_scale_auto_scale(&self, on: bool) -> Result<(), JsError> {
        self.chart.set_series_price_scale_auto_scale(&self.id, on)
    }

    pub fn get_price_scale_width(&self) -> Result<f64, JsError> {
        self.chart.get_series_price_scale_width(&self.id)
    }

    pub fn move_to_panel(&self, panel: PanelId) -> Result<(), JsError> {
        self.chart.move_series_to_panel(&self.id, panel)
    }

    pub fn get_panel(&self) -> Result<PanelId, JsError> {
        self.chart.get_series_panel(&self.id)
    }

    pub fn get_order(&self) -> Result<u32, JsError> {
        self.chart.get_series_order(&self.id)
    }

    pub fn set_order(&self, order: u32) -> Result<(), JsError> {
        self.chart.set_series_order(&self.id, order)
    }

    pub fn price_to_coordinate(&self, price: f64) -> Result<Option<f64>, JsError> {
        self.chart.price_to_coordinate(&self.id, price)
    }

    pub fn coordinate_to_price(&self, coordinate: f64) -> Result<Option<f64>, JsError> {
        self.chart.coordinate_to_price(&self.id, coordinate)
    }
}
