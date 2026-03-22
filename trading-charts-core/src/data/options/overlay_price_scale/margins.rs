use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct OverlayPriceScaleMargins {
    #[serde(rename = "top", default = "defaults::top")]
    top: f64,

    #[serde(rename = "bottom", default = "defaults::bottom")]
    bottom: f64,
}

impl OverlayPriceScaleMargins {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_params(top: f64, bottom: f64) -> Self {
        Self {
            top,
            bottom,
        }
    }

    pub fn with_top(self, top: f64) -> Self {
        Self {
            top,
            ..self
        }
    }

    pub fn with_bottom(self, bottom: f64) -> Self {
        Self {
            bottom,
            ..self
        }
    }

    pub fn top(&self) -> f64 {
        self.top
    }

    pub fn set_top(&mut self, top: f64) {
        self.top = top;
    }

    pub fn bottom(&self) -> f64 {
        self.bottom
    }

    pub fn set_bottom(&mut self, bottom: f64) {
        self.bottom = bottom;
    }
}

impl Default for OverlayPriceScaleMargins {
    fn default() -> Self {
        Self {
            top:    defaults::top(),
            bottom: defaults::bottom(),
        }
    }
}

mod defaults {
    pub(super) fn top() -> f64 {
        0.2
    }

    pub(super) fn bottom() -> f64 {
        0.1
    }
}
