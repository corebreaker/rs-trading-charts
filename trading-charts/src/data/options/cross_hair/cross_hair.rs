use super::{CrosshairMode, CrosshairLineOptions};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CrossHairOptions {
    #[serde(rename = "mode", default)]
    mode: CrosshairMode,

    #[serde(rename = "vertLine", default)]
    vert_line: CrosshairLineOptions,

    #[serde(rename = "horzLine", default)]
    horz_line: CrosshairLineOptions,
}

impl CrossHairOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_params(
        mode: CrosshairMode,
        vert_line: CrosshairLineOptions,
        horz_line: CrosshairLineOptions,
    ) -> Self {
        Self {
            mode,
            vert_line,
            horz_line,
        }
    }

    pub fn with_mode(self, mode: CrosshairMode) -> Self {
        Self {
            mode,
            ..self
        }
    }

    pub fn with_vert_line(self, vert_line: CrosshairLineOptions) -> Self {
        Self {
            vert_line,
            ..self
        }
    }

    pub fn with_horz_line(self, horz_line: CrosshairLineOptions) -> Self {
        Self {
            horz_line,
            ..self
        }
    }

    pub fn mode(&self) -> &CrosshairMode {
        &self.mode
    }

    pub fn set_mode(&mut self, mode: CrosshairMode) {
        self.mode = mode;
    }

    pub fn vert_line(&self) -> &CrosshairLineOptions {
        &self.vert_line
    }

    pub fn vert_line_mut(&mut self) -> &mut CrosshairLineOptions {
        &mut self.vert_line
    }

    pub fn set_vert_line(&mut self, vert_line: CrosshairLineOptions) {
        self.vert_line = vert_line;
    }

    pub fn horz_line(&self) -> &CrosshairLineOptions {
        &self.horz_line
    }

    pub fn horz_line_mut(&mut self) -> &mut CrosshairLineOptions {
        &mut self.horz_line
    }

    pub fn set_horz_line(&mut self, horz_line: CrosshairLineOptions) {
        self.horz_line = horz_line;
    }
}
