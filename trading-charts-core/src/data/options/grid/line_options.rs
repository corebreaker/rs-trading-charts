use crate::data::options::LineStyle;
use serde::{Deserialize, Serialize};

/// Grid line options.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridLineOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    color:   Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    style:   Option<LineStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
}

impl GridLineOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Line color.
    ///
    /// Default: `"#D6DCDE"`.
    pub fn with_color(self, color: impl Into<String>) -> Self {
        Self {
            color: Some(color.into()),
            ..self
        }
    }

    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    /// Set or reset (with `None`) the `color` option.
    pub fn set_color(&mut self, color: Option<String>) {
        self.color = color;
    }

    /// Line style.
    ///
    /// Default: `LineStyle.Solid`.
    pub fn with_style(self, style: LineStyle) -> Self {
        Self {
            style: Some(style),
            ..self
        }
    }

    pub fn style(&self) -> Option<LineStyle> {
        self.style
    }

    /// Set or reset (with `None`) the `style` option.
    pub fn set_style(&mut self, style: Option<LineStyle>) {
        self.style = style;
    }

    /// Display the lines.
    ///
    /// Default: `true`.
    pub fn with_visible(self, visible: bool) -> Self {
        Self {
            visible: Some(visible),
            ..self
        }
    }

    pub fn visible(&self) -> Option<bool> {
        self.visible
    }

    /// Set or reset (with `None`) the `visible` option.
    pub fn set_visible(&mut self, visible: Option<bool>) {
        self.visible = visible;
    }
}
