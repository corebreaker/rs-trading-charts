use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LayoutPanesOptions {
    #[serde(rename = "enableResize", default = "defaults::enable_resize")]
    pub enable_resize: bool,

    #[serde(rename = "separatorColor", default = "defaults::separator_color")]
    pub separator_color: String,

    #[serde(rename = "separatorHoverColor", default = "defaults::separator_hover_color")]
    pub separator_hover_color: String,
}

impl LayoutPanesOptions {
    pub fn with_enable_resize(self, enable_resize: bool) -> Self {
        Self {
            enable_resize,
            ..self
        }
    }

    pub fn with_separator_color(self, separator_color: String) -> Self {
        Self {
            separator_color,
            ..self
        }
    }

    pub fn with_separator_hover_color(self, separator_hover_color: String) -> Self {
        Self {
            separator_hover_color,
            ..self
        }
    }

    pub fn enable_resize(&self) -> bool {
        self.enable_resize
    }

    pub fn set_enable_resize(&mut self, enable_resize: bool) {
        self.enable_resize = enable_resize;
    }

    pub fn separator_color(&self) -> &str {
        &self.separator_color
    }

    pub fn set_separator_color(&mut self, separator_color: String) {
        self.separator_color = separator_color;
    }

    pub fn separator_hover_color(&self) -> &str {
        &self.separator_hover_color
    }

    pub fn set_separator_hover_color(&mut self, separator_hover_color: String) {
        self.separator_hover_color = separator_hover_color;
    }
}

impl Default for LayoutPanesOptions {
    fn default() -> Self {
        Self {
            enable_resize:         defaults::enable_resize(),
            separator_color:       defaults::separator_color(),
            separator_hover_color: defaults::separator_hover_color(),
        }
    }
}

mod defaults {
    pub(super) fn enable_resize() -> bool {
        true
    }

    pub(super) fn separator_color() -> String {
        String::from("#2B2B43")
    }

    pub(super) fn separator_hover_color() -> String {
        String::from("rgba(178, 181, 189, 0.2)")
    }
}
