use crate::data::options::grid::GridLineOptions;
use serde::{Deserialize, Serialize};

/// Structure describing grid options.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    vert_lines: Option<GridLineOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    horz_lines: Option<GridLineOptions>,
}

impl GridOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Vertical grid line options.
    pub fn with_vert_lines(self, vert_lines: GridLineOptions) -> Self {
        Self {
            vert_lines: Some(vert_lines),
            ..self
        }
    }

    pub fn vert_lines(&self) -> Option<&GridLineOptions> {
        self.vert_lines.as_ref()
    }

    /// Set or reset (with `None`) the `vertLines` option.
    pub fn set_vert_lines(&mut self, vert_lines: Option<GridLineOptions>) {
        self.vert_lines = vert_lines;
    }

    /// Horizontal grid line options.
    pub fn with_horz_lines(self, horz_lines: GridLineOptions) -> Self {
        Self {
            horz_lines: Some(horz_lines),
            ..self
        }
    }

    pub fn horz_lines(&self) -> Option<&GridLineOptions> {
        self.horz_lines.as_ref()
    }

    /// Set or reset (with `None`) the `horzLines` option.
    pub fn set_horz_lines(&mut self, horz_lines: Option<GridLineOptions>) {
        self.horz_lines = horz_lines;
    }
}
