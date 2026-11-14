use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageWatermarkOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max_width:  Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max_height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    padding:    Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    alpha:      Option<f64>,
}

impl ImageWatermarkOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Maximum width for the image watermark.
    ///
    /// Default: `undefined`.
    pub fn with_max_width(self, max_width: impl Into<f64>) -> Self {
        Self {
            max_width: Some(max_width.into()),
            ..self
        }
    }

    pub fn max_width(&self) -> Option<f64> {
        self.max_width
    }

    /// Set or reset (with `None`) the `maxWidth` option.
    pub fn set_max_width(&mut self, max_width: Option<f64>) {
        self.max_width = max_width;
    }

    /// Maximum height for the image watermark.
    ///
    /// Default: `undefined`.
    pub fn with_max_height(self, max_height: impl Into<f64>) -> Self {
        Self {
            max_height: Some(max_height.into()),
            ..self
        }
    }

    pub fn max_height(&self) -> Option<f64> {
        self.max_height
    }

    /// Set or reset (with `None`) the `maxHeight` option.
    pub fn set_max_height(&mut self, max_height: Option<f64>) {
        self.max_height = max_height;
    }

    /// Padding to maintain around the image watermark relative to the chart pane edges.
    ///
    /// Default: `0`.
    pub fn with_padding(self, padding: impl Into<f64>) -> Self {
        Self {
            padding: Some(padding.into()),
            ..self
        }
    }

    pub fn padding(&self) -> Option<f64> {
        self.padding
    }

    /// Set or reset (with `None`) the `padding` option.
    pub fn set_padding(&mut self, padding: Option<f64>) {
        self.padding = padding;
    }

    /// The alpha (opacity) for the image watermark. Where `1` is fully opaque (visible) and `0` is fully transparent.
    ///
    /// Default: `1`.
    pub fn with_alpha(self, alpha: impl Into<f64>) -> Self {
        Self {
            alpha: Some(alpha.into()),
            ..self
        }
    }

    pub fn alpha(&self) -> Option<f64> {
        self.alpha
    }

    /// Set or reset (with `None`) the `alpha` option.
    pub fn set_alpha(&mut self, alpha: Option<f64>) {
        self.alpha = alpha;
    }
}
