use super::background_options::{SolidColor, VerticalGradientColor};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(tag = "type")]
pub enum Background {
    #[default]
    #[serde(rename = "none")]
    None,

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

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}
