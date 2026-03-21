use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct TextWatermarkLineOptions {
    color:       String,
    text:        String,
    #[serde(rename = "fontSize")]
    font_size:   f64,
    #[serde(rename = "lineHeight", skip_serializing_if = "Option::is_none")]
    line_height: Option<f64>,
    #[serde(rename = "fontFamily")]
    font_family: String,
    #[serde(rename = "fontStyle")]
    font_style:  String,
}

impl TextWatermarkLineOptions {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }

    pub fn with_color(self, color: impl Into<String>) -> Self {
        Self {
            color: color.into(),
            ..self
        }
    }

    pub fn with_font_size(self, font_size: f64) -> Self {
        Self {
            font_size,
            ..self
        }
    }

    pub fn with_line_height(self, line_height: f64) -> Self {
        Self {
            line_height: Some(line_height),
            ..self
        }
    }

    pub fn with_font_family(self, font_family: impl Into<String>) -> Self {
        Self {
            font_family: font_family.into(),
            ..self
        }
    }

    pub fn with_font_style(self, font_style: impl Into<String>) -> Self {
        Self {
            font_style: font_style.into(),
            ..self
        }
    }
}

impl Default for TextWatermarkLineOptions {
    fn default() -> Self {
        Self {
            color:       String::from("rgba(0, 0, 0, 0.5)"),
            text:        String::new(),
            font_size:   48.0,
            line_height: None,
            font_family: String::from("-apple-system, BlinkMacSystemFont, 'Trebuchet MS', Roboto, Ubuntu, sans-serif"),
            font_style:  String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct TextWatermarkOptions {
    visible:    bool,
    #[serde(rename = "horzAlign")]
    horz_align: String,
    #[serde(rename = "vertAlign")]
    vert_align: String,
    lines:      Vec<TextWatermarkLineOptions>,
}

impl TextWatermarkOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_visible(self, visible: bool) -> Self {
        Self {
            visible,
            ..self
        }
    }

    pub fn with_horz_align(self, horz_align: impl Into<String>) -> Self {
        Self {
            horz_align: horz_align.into(),
            ..self
        }
    }

    pub fn with_vert_align(self, vert_align: impl Into<String>) -> Self {
        Self {
            vert_align: vert_align.into(),
            ..self
        }
    }

    pub fn with_lines(self, lines: Vec<TextWatermarkLineOptions>) -> Self {
        Self {
            lines,
            ..self
        }
    }
}

impl Default for TextWatermarkOptions {
    fn default() -> Self {
        Self {
            visible:    true,
            horz_align: String::from("center"),
            vert_align: String::from("center"),
            lines:      Vec::new(),
        }
    }
}
