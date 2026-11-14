use {crate::data::options::crosshair::CrosshairLineOptions, crate::data::options::crosshair::CrosshairMode};
use serde::{Deserialize, Serialize};

/// Structure describing crosshair options
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrosshairOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    mode:                                 Option<CrosshairMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    vert_line:                            Option<CrosshairLineOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    horz_line:                            Option<CrosshairLineOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    do_not_snap_to_hidden_series_indices: Option<bool>,
}

impl CrosshairOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Crosshair mode
    ///
    /// Default: `CrosshairMode.Magnet`.
    pub fn with_mode(self, mode: CrosshairMode) -> Self {
        Self {
            mode: Some(mode),
            ..self
        }
    }

    pub fn mode(&self) -> Option<CrosshairMode> {
        self.mode
    }

    /// Set or reset (with `None`) the `mode` option.
    pub fn set_mode(&mut self, mode: Option<CrosshairMode>) {
        self.mode = mode;
    }

    /// Vertical line options.
    pub fn with_vert_line(self, vert_line: CrosshairLineOptions) -> Self {
        Self {
            vert_line: Some(vert_line),
            ..self
        }
    }

    pub fn vert_line(&self) -> Option<&CrosshairLineOptions> {
        self.vert_line.as_ref()
    }

    /// Set or reset (with `None`) the `vertLine` option.
    pub fn set_vert_line(&mut self, vert_line: Option<CrosshairLineOptions>) {
        self.vert_line = vert_line;
    }

    /// Horizontal line options.
    pub fn with_horz_line(self, horz_line: CrosshairLineOptions) -> Self {
        Self {
            horz_line: Some(horz_line),
            ..self
        }
    }

    pub fn horz_line(&self) -> Option<&CrosshairLineOptions> {
        self.horz_line.as_ref()
    }

    /// Set or reset (with `None`) the `horzLine` option.
    pub fn set_horz_line(&mut self, horz_line: Option<CrosshairLineOptions>) {
        self.horz_line = horz_line;
    }

    /// If set to `true`, the crosshair will not snap to the data points of hidden series.
    ///
    /// Default: `false`.
    pub fn with_do_not_snap_to_hidden_series_indices(self, do_not_snap_to_hidden_series_indices: bool) -> Self {
        Self {
            do_not_snap_to_hidden_series_indices: Some(do_not_snap_to_hidden_series_indices),
            ..self
        }
    }

    pub fn do_not_snap_to_hidden_series_indices(&self) -> Option<bool> {
        self.do_not_snap_to_hidden_series_indices
    }

    /// Set or reset (with `None`) the `doNotSnapToHiddenSeriesIndices` option.
    pub fn set_do_not_snap_to_hidden_series_indices(&mut self, do_not_snap_to_hidden_series_indices: Option<bool>) {
        self.do_not_snap_to_hidden_series_indices = do_not_snap_to_hidden_series_indices;
    }
}
