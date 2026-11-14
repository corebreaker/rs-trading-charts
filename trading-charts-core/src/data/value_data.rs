use super::UTCTimestamp;
use serde::{Deserialize, Serialize};

/// Data item of the single-value series: line, area and baseline series.
///
/// An item without value is a *whitespace* item: it occupies a slot on the time scale without being drawn, which
/// makes gaps in the series. The colors override the series options for this item only; each one is only used by the
/// series types noted in its builder documentation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueData {
    time:               UTCTimestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value:              Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    color:              Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    line_color:         Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    top_color:          Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bottom_color:       Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    top_line_color:     Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    top_fill_color1:    Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    top_fill_color2:    Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bottom_line_color:  Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bottom_fill_color1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bottom_fill_color2: Option<String>,
}

impl ValueData {
    /// Create an item with a value.
    pub fn new(time: UTCTimestamp, value: f64) -> Self {
        Self {
            value: Some(value),
            ..Self::whitespace(time)
        }
    }

    /// Create a whitespace item, which has no value.
    pub fn whitespace(time: UTCTimestamp) -> Self {
        Self {
            time,
            value: None,
            color: None,
            line_color: None,
            top_color: None,
            bottom_color: None,
            top_line_color: None,
            top_fill_color1: None,
            top_fill_color2: None,
            bottom_line_color: None,
            bottom_fill_color1: None,
            bottom_fill_color2: None,
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

    /// Color of the line at this item (line series).
    pub fn with_color(self, color: impl Into<String>) -> Self {
        Self {
            color: Some(color.into()),
            ..self
        }
    }

    /// Color of the line at this item (area series).
    pub fn with_line_color(self, line_color: impl Into<String>) -> Self {
        Self {
            line_color: Some(line_color.into()),
            ..self
        }
    }

    /// Top color of the area fill at this item (area series).
    pub fn with_top_color(self, top_color: impl Into<String>) -> Self {
        Self {
            top_color: Some(top_color.into()),
            ..self
        }
    }

    /// Bottom color of the area fill at this item (area series).
    pub fn with_bottom_color(self, bottom_color: impl Into<String>) -> Self {
        Self {
            bottom_color: Some(bottom_color.into()),
            ..self
        }
    }

    /// Color of the line above the base value at this item (baseline series).
    pub fn with_top_line_color(self, top_line_color: impl Into<String>) -> Self {
        Self {
            top_line_color: Some(top_line_color.into()),
            ..self
        }
    }

    /// First color of the fill above the base value at this item (baseline series).
    pub fn with_top_fill_color1(self, top_fill_color1: impl Into<String>) -> Self {
        Self {
            top_fill_color1: Some(top_fill_color1.into()),
            ..self
        }
    }

    /// Second color of the fill above the base value at this item (baseline series).
    pub fn with_top_fill_color2(self, top_fill_color2: impl Into<String>) -> Self {
        Self {
            top_fill_color2: Some(top_fill_color2.into()),
            ..self
        }
    }

    /// Color of the line below the base value at this item (baseline series).
    pub fn with_bottom_line_color(self, bottom_line_color: impl Into<String>) -> Self {
        Self {
            bottom_line_color: Some(bottom_line_color.into()),
            ..self
        }
    }

    /// First color of the fill below the base value at this item (baseline series).
    pub fn with_bottom_fill_color1(self, bottom_fill_color1: impl Into<String>) -> Self {
        Self {
            bottom_fill_color1: Some(bottom_fill_color1.into()),
            ..self
        }
    }

    /// Second color of the fill below the base value at this item (baseline series).
    pub fn with_bottom_fill_color2(self, bottom_fill_color2: impl Into<String>) -> Self {
        Self {
            bottom_fill_color2: Some(bottom_fill_color2.into()),
            ..self
        }
    }

    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    pub fn line_color(&self) -> Option<&str> {
        self.line_color.as_deref()
    }

    pub fn top_color(&self) -> Option<&str> {
        self.top_color.as_deref()
    }

    pub fn bottom_color(&self) -> Option<&str> {
        self.bottom_color.as_deref()
    }

    pub fn top_line_color(&self) -> Option<&str> {
        self.top_line_color.as_deref()
    }

    pub fn top_fill_color1(&self) -> Option<&str> {
        self.top_fill_color1.as_deref()
    }

    pub fn top_fill_color2(&self) -> Option<&str> {
        self.top_fill_color2.as_deref()
    }

    pub fn bottom_line_color(&self) -> Option<&str> {
        self.bottom_line_color.as_deref()
    }

    pub fn bottom_fill_color1(&self) -> Option<&str> {
        self.bottom_fill_color1.as_deref()
    }

    pub fn bottom_fill_color2(&self) -> Option<&str> {
        self.bottom_fill_color2.as_deref()
    }
}
