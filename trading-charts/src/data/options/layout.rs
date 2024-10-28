use super::background::Background;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LayoutOptions {
    #[serde(default)]
    pub background: Background,

    #[serde(rename = "textColor", default = "defaults::text_color")]
    pub text_color: String,

    #[serde(rename = "fontSize", default = "defaults::font_size")]
    pub font_size: usize,

    #[serde(rename = "fontFamily", default = "defaults::font_family")]
    pub font_family: String,

    #[serde(rename = "attributionLogo", default = "defaults::attribution_logo")]
    pub attribution_logo: bool,
}

impl LayoutOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_background(self, background: Background) -> Self {
        Self {
            background,
            ..self
        }
    }

    pub fn with_text_color(self, text_color: String) -> Self {
        Self {
            text_color,
            ..self
        }
    }

    pub fn with_font_size(self, font_size: usize) -> Self {
        Self {
            font_size,
            ..self
        }
    }

    pub fn with_font_family(self, font_family: String) -> Self {
        Self {
            font_family,
            ..self
        }
    }

    pub fn with_attribution_logo(self, attribution_logo: bool) -> Self {
        Self {
            attribution_logo,
            ..self
        }
    }

    pub fn background(&self) -> &Background {
        &self.background
    }

    pub fn set_background(&mut self, background: Background) {
        self.background = background;
    }

    pub fn text_color(&self) -> &str {
        &self.text_color
    }

    pub fn text_color_mut(&mut self) -> &mut String {
        &mut self.text_color
    }

    pub fn set_text_color(&mut self, text_color: String) {
        self.text_color = text_color;
    }

    pub fn font_size(&self) -> usize {
        self.font_size
    }

    pub fn set_font_size(&mut self, font_size: usize) {
        self.font_size = font_size;
    }

    pub fn font_family(&self) -> &str {
        &self.font_family
    }

    pub fn font_family_mut(&mut self) -> &mut String {
        &mut self.font_family
    }

    pub fn set_font_family(&mut self, font_family: String) {
        self.font_family = font_family;
    }

    pub fn attribution_logo(&self) -> bool {
        self.attribution_logo
    }

    pub fn set_attribution_logo(&mut self, attribution_logo: bool) {
        self.attribution_logo = attribution_logo;
    }
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            background:       Background::default(),
            text_color:       defaults::text_color(),
            font_size:        defaults::font_size(),
            font_family:      defaults::font_family(),
            attribution_logo: false,
        }
    }
}

mod defaults {
    pub(super) fn text_color() -> String {
        String::from("#191919")
    }

    pub(super) fn font_size() -> usize {
        12
    }

    pub(super) fn font_family() -> String {
        String::from("-apple-system, BlinkMacSystemFont, 'Trebuchet MS', Roboto, Ubuntu, sans-serif")
    }

    pub(super) fn attribution_logo() -> bool {
        true
    }
}
