use super::super::{LineStyle, LineWidth};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CrosshairLineOptions {
    #[serde(default = "defaults::color")]
    color: String,

    #[serde(default)]
    width: LineWidth,

    #[serde(default)]
    style: LineStyle,

    #[serde(default = "defaults::visible")]
    visible: bool,

    #[serde(default = "defaults::label_visible")]
    label_visible: bool,

    #[serde(default = "defaults::label_background_color")]
    label_background_color: String,
}

impl CrosshairLineOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_color(self, color: String) -> Self {
        Self {
            color,
            ..self
        }
    }

    pub fn with_width(self, width: LineWidth) -> Self {
        Self {
            width,
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

    pub fn with_label_visible(self, label_visible: bool) -> Self {
        Self {
            label_visible,
            ..self
        }
    }

    pub fn with_label_background_color(self, label_background_color: String) -> Self {
        Self {
            label_background_color,
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

    pub fn width(&self) -> &LineWidth {
        &self.width
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn width_mut(&mut self) -> &mut LineWidth {
        &mut self.width
    }

    pub fn set_width(&mut self, width: LineWidth) {
        self.width = width;
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

    pub fn label_visible(&self) -> bool {
        self.label_visible
    }

    pub fn set_label_visible(&mut self, label_visible: bool) {
        self.label_visible = label_visible;
    }

    pub fn label_background_color(&self) -> &str {
        &self.label_background_color
    }

    pub fn label_background_color_mut(&mut self) -> &mut String {
        &mut self.label_background_color
    }

    pub fn set_label_background_color(&mut self, label_background_color: String) {
        self.label_background_color = label_background_color;
    }
}

impl Default for CrosshairLineOptions {
    fn default() -> Self {
        Self {
            color:                  defaults::color(),
            width:                  LineWidth::default(),
            style:                  LineStyle::default(),
            visible:                defaults::visible(),
            label_visible:          defaults::label_visible(),
            label_background_color: defaults::label_background_color(),
        }
    }
}

mod defaults {
    pub(super) fn color() -> String {
        String::from("#758696")
    }

    pub(super) fn visible() -> bool {
        true
    }

    pub(super) fn label_visible() -> bool {
        true
    }

    pub(super) fn label_background_color() -> String {
        String::from("#4c525e")
    }
}
