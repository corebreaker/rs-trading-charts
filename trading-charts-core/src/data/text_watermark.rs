use {crate::data::options::HorzAlign, crate::data::options::VertAlign};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextWatermarkLineOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    color:       Option<String>,
    text:        String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    font_size:   Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    line_height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    font_family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    font_style:  Option<String>,
}

impl TextWatermarkLineOptions {
    /// Create options with the required values, the other options keeping their default value.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text:        text.into(),
            color:       None,
            font_size:   None,
            line_height: None,
            font_family: None,
            font_style:  None,
        }
    }

    /// Watermark color.
    ///
    /// Default: `"rgba(0, 0, 0, 0.5)"`.
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

    /// Text of the watermark. Word wrapping is not supported.
    pub fn with_text(self, text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..self
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    /// Font size in pixels.
    ///
    /// Default: `48`.
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

    /// Line height in pixels.
    ///
    /// Default: `1.2 * fontSize`.
    pub fn with_line_height(self, line_height: impl Into<f64>) -> Self {
        Self {
            line_height: Some(line_height.into()),
            ..self
        }
    }

    pub fn line_height(&self) -> Option<f64> {
        self.line_height
    }

    /// Set or reset (with `None`) the `lineHeight` option.
    pub fn set_line_height(&mut self, line_height: Option<f64>) {
        self.line_height = line_height;
    }

    /// Font family.
    ///
    /// Default: `-apple-system, BlinkMacSystemFont, 'Trebuchet MS', Roboto, Ubuntu, sans-serif`.
    pub fn with_font_family(self, font_family: impl Into<String>) -> Self {
        Self {
            font_family: Some(font_family.into()),
            ..self
        }
    }

    pub fn font_family(&self) -> Option<&str> {
        self.font_family.as_deref()
    }

    /// Set or reset (with `None`) the `fontFamily` option.
    pub fn set_font_family(&mut self, font_family: Option<String>) {
        self.font_family = font_family;
    }

    /// Font style.
    ///
    /// Default: `""`.
    pub fn with_font_style(self, font_style: impl Into<String>) -> Self {
        Self {
            font_style: Some(font_style.into()),
            ..self
        }
    }

    pub fn font_style(&self) -> Option<&str> {
        self.font_style.as_deref()
    }

    /// Set or reset (with `None`) the `fontStyle` option.
    pub fn set_font_style(&mut self, font_style: Option<String>) {
        self.font_style = font_style;
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextWatermarkOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    visible:    Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    horz_align: Option<HorzAlign>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    vert_align: Option<VertAlign>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lines:      Option<Vec<TextWatermarkLineOptions>>,
}

impl TextWatermarkOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Display the watermark.
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

    /// Horizontal alignment inside the chart area.
    ///
    /// Default: `"center"`.
    pub fn with_horz_align(self, horz_align: HorzAlign) -> Self {
        Self {
            horz_align: Some(horz_align),
            ..self
        }
    }

    pub fn horz_align(&self) -> Option<HorzAlign> {
        self.horz_align
    }

    /// Set or reset (with `None`) the `horzAlign` option.
    pub fn set_horz_align(&mut self, horz_align: Option<HorzAlign>) {
        self.horz_align = horz_align;
    }

    /// Vertical alignment inside the chart area.
    ///
    /// Default: `"center"`.
    pub fn with_vert_align(self, vert_align: VertAlign) -> Self {
        Self {
            vert_align: Some(vert_align),
            ..self
        }
    }

    pub fn vert_align(&self) -> Option<VertAlign> {
        self.vert_align
    }

    /// Set or reset (with `None`) the `vertAlign` option.
    pub fn set_vert_align(&mut self, vert_align: Option<VertAlign>) {
        self.vert_align = vert_align;
    }

    /// Text to be displayed within the watermark. Each item in the array is treated as new line.
    ///
    /// Default: `[]`.
    pub fn with_lines(self, lines: impl IntoIterator<Item = TextWatermarkLineOptions>) -> Self {
        Self {
            lines: Some(lines.into_iter().collect()),
            ..self
        }
    }

    pub fn lines(&self) -> Option<&[TextWatermarkLineOptions]> {
        self.lines.as_deref()
    }

    /// Set or reset (with `None`) the `lines` option.
    pub fn set_lines(&mut self, lines: Option<Vec<TextWatermarkLineOptions>>) {
        self.lines = lines;
    }
}
