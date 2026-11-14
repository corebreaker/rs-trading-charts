use serde::{Deserialize, Serialize};

/// Represents panes customizations.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutPanesOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    enable_resize:         Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    separator_color:       Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    separator_hover_color: Option<String>,
}

impl LayoutPanesOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable panes resizing
    ///
    /// Default: `true`.
    pub fn with_enable_resize(self, enable_resize: bool) -> Self {
        Self {
            enable_resize: Some(enable_resize),
            ..self
        }
    }

    pub fn enable_resize(&self) -> Option<bool> {
        self.enable_resize
    }

    /// Set or reset (with `None`) the `enableResize` option.
    pub fn set_enable_resize(&mut self, enable_resize: Option<bool>) {
        self.enable_resize = enable_resize;
    }

    /// Color of pane separator
    ///
    /// Default: `#2B2B43`.
    pub fn with_separator_color(self, separator_color: impl Into<String>) -> Self {
        Self {
            separator_color: Some(separator_color.into()),
            ..self
        }
    }

    pub fn separator_color(&self) -> Option<&str> {
        self.separator_color.as_deref()
    }

    /// Set or reset (with `None`) the `separatorColor` option.
    pub fn set_separator_color(&mut self, separator_color: Option<String>) {
        self.separator_color = separator_color;
    }

    /// Color of pane separator background applied on hover
    ///
    /// Default: `rgba(178, 181, 189, 0.2)`.
    pub fn with_separator_hover_color(self, separator_hover_color: impl Into<String>) -> Self {
        Self {
            separator_hover_color: Some(separator_hover_color.into()),
            ..self
        }
    }

    pub fn separator_hover_color(&self) -> Option<&str> {
        self.separator_hover_color.as_deref()
    }

    /// Set or reset (with `None`) the `separatorHoverColor` option.
    pub fn set_separator_hover_color(&mut self, separator_hover_color: Option<String>) {
        self.separator_hover_color = separator_hover_color;
    }
}
