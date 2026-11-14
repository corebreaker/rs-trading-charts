use super::{SolidColor, VerticalGradientColor};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum Background {
    #[serde(rename = "solid")]
    SolidColor(SolidColor),

    #[serde(rename = "gradient")]
    VerticalGradientColor(VerticalGradientColor),
}

impl Background {
    pub fn new_solid_color(color: impl Into<String>) -> Self {
        Self::SolidColor(SolidColor::new(color))
    }

    pub fn new_vertical_gradient_color(top_color: impl Into<String>, bottom_color: impl Into<String>) -> Self {
        Self::VerticalGradientColor(VerticalGradientColor::new(top_color, bottom_color))
    }
}

impl Default for Background {
    fn default() -> Self {
        Self::SolidColor(SolidColor::default())
    }
}

impl From<SolidColor> for Background {
    fn from(color: SolidColor) -> Self {
        Self::SolidColor(color)
    }
}

impl From<VerticalGradientColor> for Background {
    fn from(gradient: VerticalGradientColor) -> Self {
        Self::VerticalGradientColor(gradient)
    }
}
