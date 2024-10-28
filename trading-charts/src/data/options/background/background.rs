use super::{SolidColor, VerticalGradientColor};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Background {
    #[serde(rename = "solid")]
    SolidColor(SolidColor),

    #[serde(rename = "gradient")]
    VerticalGradientColor(VerticalGradientColor),
}

impl Background {
    pub fn new_solid_color(color: String) -> Self {
        Self::SolidColor(SolidColor::new(color))
    }

    pub fn new_vertical_gradient_color(top_color: String, bottom_color: String) -> Self {
        Self::VerticalGradientColor(VerticalGradientColor::new(top_color, bottom_color))
    }
}

impl Default for Background {
    fn default() -> Self {
        Self::SolidColor(SolidColor::default())
    }
}
