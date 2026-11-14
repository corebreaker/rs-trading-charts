use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct VerticalGradientColor {
    #[serde(rename = "topColor")]
    top_color: String,

    #[serde(rename = "bottomColor")]
    bottom_color: String,
}

impl VerticalGradientColor {
    pub fn new(top_color: impl Into<String>, bottom_color: impl Into<String>) -> Self {
        Self {
            top_color:    top_color.into(),
            bottom_color: bottom_color.into(),
        }
    }

    pub fn top_color(&self) -> &str {
        &self.top_color
    }

    pub fn bottom_color(&self) -> &str {
        &self.bottom_color
    }
}
