use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct VerticalGradientColor {
    #[serde(rename = "topColor")]
    top_color: String,

    #[serde(rename = "bottomColor")]
    bottom_color: String,
}

impl VerticalGradientColor {
    pub fn new(top_color: String, bottom_color: String) -> Self {
        Self {
            top_color,
            bottom_color,
        }
    }

    pub fn top_color(&self) -> &str {
        &self.top_color
    }

    pub fn bottom_color(&self) -> &str {
        &self.bottom_color
    }
}
