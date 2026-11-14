use super::UTCTimestamp;
use serde::{Deserialize, Serialize};

/// Data item of the OHLC series: candlestick and bar series.
///
/// An item without prices is a *whitespace* item: it occupies a slot on the time scale without being drawn, which
/// makes gaps in the series. The colors override the series options for this item only.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candlestick {
    time:         UTCTimestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    open:         Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    high:         Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    low:          Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    close:        Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    color:        Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    border_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    wick_color:   Option<String>,
}

impl Candlestick {
    /// Create an item with its open, high, low and close prices.
    pub fn new(time: UTCTimestamp, open: f64, high: f64, low: f64, close: f64) -> Self {
        Self {
            open: Some(open),
            high: Some(high),
            low: Some(low),
            close: Some(close),
            ..Self::whitespace(time)
        }
    }

    /// Create a whitespace item, which has no prices.
    pub fn whitespace(time: UTCTimestamp) -> Self {
        Self {
            time,
            open: None,
            high: None,
            low: None,
            close: None,
            color: None,
            border_color: None,
            wick_color: None,
        }
    }

    pub fn time(&self) -> UTCTimestamp {
        self.time
    }

    pub fn open(&self) -> Option<f64> {
        self.open
    }

    pub fn high(&self) -> Option<f64> {
        self.high
    }

    pub fn low(&self) -> Option<f64> {
        self.low
    }

    pub fn close(&self) -> Option<f64> {
        self.close
    }

    pub fn is_whitespace(&self) -> bool {
        self.open.is_none() || self.high.is_none() || self.low.is_none() || self.close.is_none()
    }

    /// Return `true` when the close price is greater than or equal to the open price.
    pub fn is_up(&self) -> bool {
        matches!((self.open, self.close), (Some(open), Some(close)) if close >= open)
    }

    /// Color of the candle body (candlestick series) or of the bar (bar series).
    pub fn with_color(self, color: impl Into<String>) -> Self {
        Self {
            color: Some(color.into()),
            ..self
        }
    }

    /// Color of the candle border (candlestick series).
    pub fn with_border_color(self, border_color: impl Into<String>) -> Self {
        Self {
            border_color: Some(border_color.into()),
            ..self
        }
    }

    /// Color of the candle wick (candlestick series).
    pub fn with_wick_color(self, wick_color: impl Into<String>) -> Self {
        Self {
            wick_color: Some(wick_color.into()),
            ..self
        }
    }

    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    pub fn border_color(&self) -> Option<&str> {
        self.border_color.as_deref()
    }

    pub fn wick_color(&self) -> Option<&str> {
        self.wick_color.as_deref()
    }
}
