use serde::{Deserialize, Serialize};

/// Settings of a pane, applied with [`ChartHandle::configure_pane`](crate::ChartHandle::configure_pane).
///
/// The settings which are not set are left unchanged.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaneOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    stretch_factor: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    height:         Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    preserve_empty: Option<bool>,
}

impl PaneOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Relative height of the pane: the height of the chart is shared between the panes in proportion to their
    /// stretch factors. Default: `1`.
    pub fn with_stretch_factor(self, stretch_factor: f64) -> Self {
        Self {
            stretch_factor: Some(stretch_factor),
            ..self
        }
    }

    /// Height of the pane in pixels, which changes its stretch factor accordingly.
    pub fn with_height(self, height: f64) -> Self {
        Self {
            height: Some(height),
            ..self
        }
    }

    /// Keep the pane when it has no series anymore. Default: `false`, an empty pane is removed.
    pub fn with_preserve_empty(self, preserve_empty: bool) -> Self {
        Self {
            preserve_empty: Some(preserve_empty),
            ..self
        }
    }

    pub fn stretch_factor(&self) -> Option<f64> {
        self.stretch_factor
    }

    pub fn height(&self) -> Option<f64> {
        self.height
    }

    pub fn preserve_empty(&self) -> Option<bool> {
        self.preserve_empty
    }

    /// Set or reset (with `None`) the `stretchFactor` setting.
    pub fn set_stretch_factor(&mut self, stretch_factor: Option<f64>) {
        self.stretch_factor = stretch_factor;
    }

    /// Set or reset (with `None`) the `height` setting.
    pub fn set_height(&mut self, height: Option<f64>) {
        self.height = height;
    }

    /// Set or reset (with `None`) the `preserveEmpty` setting.
    pub fn set_preserve_empty(&mut self, preserve_empty: Option<bool>) {
        self.preserve_empty = preserve_empty;
    }
}
