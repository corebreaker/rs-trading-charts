use super::super::LineStyle;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct GridLineOptions {
    #[serde(default = "defaults::color")]
    color: String,

    #[serde(default)]
    style: LineStyle,

    #[serde(default = "defaults::visible")]
    visible: bool,
}

impl GridLineOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_params(color: String, style: LineStyle, visible: bool) -> Self {
        Self {
            color,
            style,
            visible,
        }
    }

    pub fn with_color(self, color: String) -> Self {
        Self {
            color,
            ..self
        }
    }

    pub fn with_style(self, style: LineStyle) -> Self {
        Self {
            style,
            ..self
        }
    }

    pub fn with_visible(self, visible: bool) -> Self {
        Self {
            visible,
            ..self
        }
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn color_mut(&mut self) -> &mut String {
        &mut self.color
    }

    pub fn set_color(&mut self, color: String) {
        self.color = color;
    }

    pub fn style(&self) -> &LineStyle {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut LineStyle {
        &mut self.style
    }

    pub fn set_style(&mut self, style: LineStyle) {
        self.style = style;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Default for GridLineOptions {
    fn default() -> Self {
        Self {
            color:   defaults::color(),
            style:   LineStyle::default(),
            visible: defaults::visible(),
        }
    }
}

mod defaults {
    pub(super) fn color() -> String {
        String::from("#D6DCDE")
    }

    pub(super) fn visible() -> bool {
        true
    }
}
