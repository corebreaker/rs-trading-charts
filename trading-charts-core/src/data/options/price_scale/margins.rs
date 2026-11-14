use serde::{Deserialize, Serialize};

/// Defines margins of the price scale.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceScaleMargins {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    top:    Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bottom: Option<f64>,
}

impl PriceScaleMargins {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Top margin in percentages. Must be greater or equal to 0 and less than 1.
    pub fn with_top(self, top: impl Into<f64>) -> Self {
        Self {
            top: Some(top.into()),
            ..self
        }
    }

    pub fn top(&self) -> Option<f64> {
        self.top
    }

    /// Set or reset (with `None`) the `top` option.
    pub fn set_top(&mut self, top: Option<f64>) {
        self.top = top;
    }

    /// Bottom margin in percentages. Must be greater or equal to 0 and less than 1.
    pub fn with_bottom(self, bottom: impl Into<f64>) -> Self {
        Self {
            bottom: Some(bottom.into()),
            ..self
        }
    }

    pub fn bottom(&self) -> Option<f64> {
        self.bottom
    }

    /// Set or reset (with `None`) the `bottom` option.
    pub fn set_bottom(&mut self, bottom: Option<f64>) {
        self.bottom = bottom;
    }
}

impl From<(f64, f64)> for PriceScaleMargins {
    /// Create margins from the `(top, bottom)` pair.
    fn from((top, bottom): (f64, f64)) -> Self {
        Self::new().with_top(top).with_bottom(bottom)
    }
}
