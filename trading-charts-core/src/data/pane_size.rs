use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct PaneSize {
    width:  f64,
    height: f64,
}

impl PaneSize {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }
}
