use super::Background;
use num_traits::Zero;
use serde::{Deserialize, Serialize};

fn default_attribution_logo() -> bool {
    true
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct LayoutOptions {
    #[serde(skip_serializing_if = "Background::is_none")]
    pub background: Background,

    #[serde(rename = "textColor", skip_serializing_if = "String::is_empty")]
    pub text_color: String,

    #[serde(rename = "fontSize", skip_serializing_if = "Zero::is_zero")]
    pub font_size: usize,

    #[serde(rename = "fontFamily", skip_serializing_if = "String::is_empty")]
    pub font_family: String,

    #[serde(rename = "attributionLogo", default = "default_attribution_logo")]
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

    pub fn text_color(&self) -> &str {
        &self.text_color
    }

    pub fn font_size(&self) -> usize {
        self.font_size
    }

    pub fn font_family(&self) -> &str {
        &self.font_family
    }

    pub fn attribution_logo(&self) -> bool {
        self.attribution_logo
    }
}
