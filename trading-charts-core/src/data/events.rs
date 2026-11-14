use super::UTCTimestamp;
use crate::PanelId;
use serde::{Deserialize, Serialize};

/// Position in pixels, relative to the top-left corner of the chart.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Data item of a series at the time of a mouse event.
///
/// It gathers the fields of all the data item types: `value` is set for the single-value and histogram series,
/// `open`, `high`, `low` and `close` for the OHLC series.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDataItem {
    pub time:  UTCTimestamp,
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default)]
    pub open:  Option<f64>,
    #[serde(default)]
    pub high:  Option<f64>,
    #[serde(default)]
    pub low:   Option<f64>,
    #[serde(default)]
    pub close: Option<f64>,
    #[serde(default)]
    pub color: Option<String>,
}

impl EventDataItem {
    /// The value of a single-value item, or the close price of an OHLC item.
    pub fn price(&self) -> Option<f64> {
        self.value.or(self.close)
    }

    /// Change between the open and close prices of an OHLC item, in percent.
    pub fn change_percent(&self) -> Option<f64> {
        match (self.open, self.close) {
            (Some(open), Some(close)) if open != 0.0 => Some((close - open) / open * 100.0),
            _ => None,
        }
    }
}

/// Data of a series at the time of a mouse event.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesEventData {
    /// Identifier of the series, as given by [`SeriesHandle::id`](crate::SeriesHandle::id).
    pub series_id:   String,
    /// Type of the series: `line`, `area`, `baseline`, `histogram`, `bar` or `candlestick`.
    pub series_type: String,
    /// Title of the series, from its options.
    #[serde(default)]
    pub title:       Option<String>,
    /// Pane of the series.
    pub pane_index:  PanelId,
    /// Data item of the series at the time of the event.
    pub data:        EventDataItem,
    /// Vertical coordinate of the price of the item, in pixels from the top of the chart.
    #[serde(default)]
    pub coordinate:  Option<f64>,
}

/// Parameters of the crosshair move, click and double click events.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MouseEventParams {
    /// Time of the bar under the cursor, `None` when the cursor is outside of the data.
    #[serde(default)]
    pub time:              Option<UTCTimestamp>,
    /// Logical index of the bar under the cursor.
    #[serde(default)]
    pub logical:           Option<f64>,
    /// Position of the cursor relatively to the top-left corner of the chart, `None` when the cursor left the
    /// chart.
    #[serde(default)]
    pub point:             Option<Point>,
    /// Pane under the cursor.
    #[serde(default)]
    pub pane_index:        Option<PanelId>,
    /// Identifier of the series under the cursor.
    #[serde(default)]
    pub hovered_series_id: Option<String>,
    /// Identifier of the object under the cursor, such as a marker or a price line.
    #[serde(default)]
    pub hovered_object_id: Option<String>,
    /// Data of the series at the time of the bar under the cursor.
    #[serde(default)]
    pub series_data:       Vec<SeriesEventData>,
    /// Width of the chart, in pixels.
    pub chart_width:       f64,
    /// Height of the chart, in pixels.
    pub chart_height:      f64,
}

impl MouseEventParams {
    /// Return `true` when the cursor is over the data of the chart.
    pub fn is_over_data(&self) -> bool {
        self.time.is_some() && self.point.is_some()
    }

    /// Data of a series, by series identifier.
    pub fn series(&self, series_id: &str) -> Option<&SeriesEventData> {
        self.series_data.iter().find(|data| data.series_id == series_id)
    }

    /// Data of the series under the cursor, or of the first series when no series is under the cursor.
    pub fn main_series(&self) -> Option<&SeriesEventData> {
        self.hovered_series_id
            .as_deref()
            .and_then(|id| self.series(id))
            .or_else(|| self.series_data.first())
    }
}

/// Size of a chart, in pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Size {
    pub width:  f64,
    pub height: f64,
}
