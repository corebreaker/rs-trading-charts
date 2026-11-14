use super::UTCTimestamp;
use serde::{Deserialize, Serialize};

/// Data item of the histogram series.
///
/// An item without value is a *whitespace* item: it occupies a slot on the time scale without being drawn.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistogramData {
    time:  UTCTimestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    color: Option<String>,
}

impl HistogramData {
    /// Create an item with a value.
    pub fn new(time: UTCTimestamp, value: f64) -> Self {
        Self {
            time,
            value: Some(value),
            color: None,
        }
    }

    /// Create a whitespace item, which has no value.
    pub fn whitespace(time: UTCTimestamp) -> Self {
        Self {
            time,
            value: None,
            color: None,
        }
    }

    /// Color of the column of this item, overriding the series color.
    pub fn with_color(self, color: impl Into<String>) -> Self {
        Self {
            color: Some(color.into()),
            ..self
        }
    }

    pub fn time(&self) -> UTCTimestamp {
        self.time
    }

    /// Value of the item, `None` for a whitespace item.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn is_whitespace(&self) -> bool {
        self.value.is_none()
    }

    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }
}
