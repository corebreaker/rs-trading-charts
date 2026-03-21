use super::GridLineOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct GridOptions {
    #[serde(rename = "vertLines", default)]
    vert_lines: GridLineOptions,

    #[serde(rename = "horzLines", default)]
    horz_lines: GridLineOptions,
}

impl GridOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_params(vert_lines: GridLineOptions, horz_lines: GridLineOptions) -> Self {
        Self {
            vert_lines,
            horz_lines,
        }
    }

    pub fn with_vert_lines(self, vert_lines: GridLineOptions) -> Self {
        Self {
            vert_lines,
            ..self
        }
    }

    pub fn with_horz_lines(self, horz_lines: GridLineOptions) -> Self {
        Self {
            horz_lines,
            ..self
        }
    }

    pub fn vert_lines(&self) -> &GridLineOptions {
        &self.vert_lines
    }

    pub fn vert_lines_mut(&mut self) -> &mut GridLineOptions {
        &mut self.vert_lines
    }

    pub fn set_vert_lines(&mut self, vert_lines: GridLineOptions) {
        self.vert_lines = vert_lines;
    }

    pub fn horz_lines(&self) -> &GridLineOptions {
        &self.horz_lines
    }

    pub fn horz_lines_mut(&mut self) -> &mut GridLineOptions {
        &mut self.horz_lines
    }

    pub fn set_horz_lines(&mut self, horz_lines: GridLineOptions) {
        self.horz_lines = horz_lines;
    }
}
