use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SolidColor {
    color: String,
}

impl SolidColor {
    pub fn new(color: String) -> Self {
        Self {
            color,
        }
    }

    pub fn color(&self) -> &str {
        &self.color
    }
}
