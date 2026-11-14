use {crate::data::options::LineStyle, crate::data::options::LineWidth};
use serde::{Deserialize, Serialize};

/// Structure describing a crosshair line (vertical or horizontal)
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrosshairLineOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    color:                  Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    width:                  Option<LineWidth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    style:                  Option<LineStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    visible:                Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    label_visible:          Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    label_background_color: Option<String>,
}

impl CrosshairLineOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Crosshair line color.
    ///
    /// Default: `"#758696"`.
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

    /// Crosshair line width.
    ///
    /// Default: `1`.
    pub fn with_width(self, width: LineWidth) -> Self {
        Self {
            width: Some(width),
            ..self
        }
    }

    pub fn width(&self) -> Option<LineWidth> {
        self.width
    }

    /// Set or reset (with `None`) the `width` option.
    pub fn set_width(&mut self, width: Option<LineWidth>) {
        self.width = width;
    }

    /// Crosshair line style.
    ///
    /// Default: `LineStyle.LargeDashed`.
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

    /// Display the crosshair line.
    ///
    /// Note that disabling crosshair lines does not disable crosshair marker on Line and Area series.
    /// It can be disabled by using `crosshairMarkerVisible` option of a relevant series.
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

    /// Display the crosshair label on the relevant scale.
    ///
    /// Default: `true`.
    pub fn with_label_visible(self, label_visible: bool) -> Self {
        Self {
            label_visible: Some(label_visible),
            ..self
        }
    }

    pub fn label_visible(&self) -> Option<bool> {
        self.label_visible
    }

    /// Set or reset (with `None`) the `labelVisible` option.
    pub fn set_label_visible(&mut self, label_visible: Option<bool>) {
        self.label_visible = label_visible;
    }

    /// Crosshair label background color.
    ///
    /// Default: `"#4c525e"`.
    pub fn with_label_background_color(self, label_background_color: impl Into<String>) -> Self {
        Self {
            label_background_color: Some(label_background_color.into()),
            ..self
        }
    }

    pub fn label_background_color(&self) -> Option<&str> {
        self.label_background_color.as_deref()
    }

    /// Set or reset (with `None`) the `labelBackgroundColor` option.
    pub fn set_label_background_color(&mut self, label_background_color: Option<String>) {
        self.label_background_color = label_background_color;
    }
}
