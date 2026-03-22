use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ImageWatermarkOptions {
    #[serde(rename = "maxWidth", skip_serializing_if = "Option::is_none")]
    max_width:  Option<f64>,
    #[serde(rename = "maxHeight", skip_serializing_if = "Option::is_none")]
    max_height: Option<f64>,
    padding:    f64,
    alpha:      f64,
}

impl ImageWatermarkOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_width(self, max_width: f64) -> Self {
        Self {
            max_width: Some(max_width),
            ..self
        }
    }

    pub fn with_max_height(self, max_height: f64) -> Self {
        Self {
            max_height: Some(max_height),
            ..self
        }
    }

    pub fn with_padding(self, padding: f64) -> Self {
        Self {
            padding,
            ..self
        }
    }

    pub fn with_alpha(self, alpha: f64) -> Self {
        Self {
            alpha,
            ..self
        }
    }
}

impl Default for ImageWatermarkOptions {
    fn default() -> Self {
        Self {
            max_width:  None,
            max_height: None,
            padding:    0.0,
            alpha:      1.0,
        }
    }
}
