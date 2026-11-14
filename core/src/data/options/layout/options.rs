use {
    crate::data::format::FontFamily,
    crate::data::options::background::Background,
    crate::data::options::layout::ColorSpace,
    crate::data::options::layout::LayoutPanesOptions,
};
use serde::{Deserialize, Serialize};

/// Represents layout options
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    background:       Option<Background>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    text_color:       Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    font_size:        Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    font_family:      Option<FontFamily>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    panes:            Option<LayoutPanesOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    attribution_logo: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    color_space:      Option<ColorSpace>,
}

impl LayoutOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Chart and scales background color.
    ///
    /// Default: `{ type: ColorType.Solid, color: '#FFFFFF' }`.
    pub fn with_background(self, background: impl Into<Background>) -> Self {
        Self {
            background: Some(background.into()),
            ..self
        }
    }

    pub fn background(&self) -> Option<&Background> {
        self.background.as_ref()
    }

    /// Set or reset (with `None`) the `background` option.
    pub fn set_background(&mut self, background: Option<Background>) {
        self.background = background;
    }

    /// Color of text on the scales.
    ///
    /// Default: `"#191919"`.
    pub fn with_text_color(self, text_color: impl Into<String>) -> Self {
        Self {
            text_color: Some(text_color.into()),
            ..self
        }
    }

    pub fn text_color(&self) -> Option<&str> {
        self.text_color.as_deref()
    }

    /// Set or reset (with `None`) the `textColor` option.
    pub fn set_text_color(&mut self, text_color: Option<String>) {
        self.text_color = text_color;
    }

    /// Font size of text on scales in pixels.
    ///
    /// Default: `12`.
    pub fn with_font_size(self, font_size: impl Into<f64>) -> Self {
        Self {
            font_size: Some(font_size.into()),
            ..self
        }
    }

    pub fn font_size(&self) -> Option<f64> {
        self.font_size
    }

    /// Set or reset (with `None`) the `fontSize` option.
    pub fn set_font_size(&mut self, font_size: Option<f64>) {
        self.font_size = font_size;
    }

    /// Font family of text on the scales.
    ///
    /// Default: `-apple-system, BlinkMacSystemFont, 'Trebuchet MS', Roboto, Ubuntu, sans-serif`.
    pub fn with_font_family(self, font_family: impl Into<FontFamily>) -> Self {
        Self {
            font_family: Some(font_family.into()),
            ..self
        }
    }

    pub fn font_family(&self) -> Option<&FontFamily> {
        self.font_family.as_ref()
    }

    /// Set or reset (with `None`) the `fontFamily` option.
    pub fn set_font_family(&mut self, font_family: Option<FontFamily>) {
        self.font_family = font_family;
    }

    /// Panes options.
    pub fn with_panes(self, panes: LayoutPanesOptions) -> Self {
        Self {
            panes: Some(panes),
            ..self
        }
    }

    pub fn panes(&self) -> Option<&LayoutPanesOptions> {
        self.panes.as_ref()
    }

    /// Set or reset (with `None`) the `panes` option.
    pub fn set_panes(&mut self, panes: Option<LayoutPanesOptions>) {
        self.panes = panes;
    }

    /// Display the TradingView attribution logo on the main chart pane.
    ///
    /// The licence for library specifies that you add the "attribution notice" from the NOTICE file to your code and a
    /// link to https://www.tradingview.com/ to the page of your website or mobile application that is available to your
    /// users. Using this attribution logo is sufficient for meeting this linking requirement. However,
    /// if you already fulfill this requirement then you can disable this attribution logo.
    ///
    /// Default: `true`.
    pub fn with_attribution_logo(self, attribution_logo: bool) -> Self {
        Self {
            attribution_logo: Some(attribution_logo),
            ..self
        }
    }

    pub fn attribution_logo(&self) -> Option<bool> {
        self.attribution_logo
    }

    /// Set or reset (with `None`) the `attributionLogo` option.
    pub fn set_attribution_logo(&mut self, attribution_logo: Option<bool>) {
        self.attribution_logo = attribution_logo;
    }

    /// Specifies the color space of the rendering context for the internal canvas elements.
    ///
    /// Note: this option should only be specified during the chart creation and not changed at a later stage by using
    /// `applyOptions`.
    pub fn with_color_space(self, color_space: ColorSpace) -> Self {
        Self {
            color_space: Some(color_space),
            ..self
        }
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        self.color_space
    }

    /// Set or reset (with `None`) the `colorSpace` option.
    pub fn set_color_space(&mut self, color_space: Option<ColorSpace>) {
        self.color_space = color_space;
    }
}
