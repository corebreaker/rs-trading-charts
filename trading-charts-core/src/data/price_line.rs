use crate::data::options::{LineStyle, LineWidth};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceLineOptions {
    price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_width: Option<LineWidth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<LineStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_label_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_label_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_label_text_color: Option<String>,
}

impl PriceLineOptions {
    pub fn new(price: f64) -> Self {
        Self {
            price,
            id: None,
            color: None,
            line_width: None,
            line_style: None,
            line_visible: None,
            axis_label_visible: None,
            title: None,
            axis_label_color: None,
            axis_label_text_color: None,
        }
    }

    pub fn with_id(self, id: String) -> Self {
        Self { id: Some(id), ..self }
    }

    pub fn with_color(self, color: String) -> Self {
        Self {
            color: Some(color),
            ..self
        }
    }

    pub fn with_line_width(self, line_width: LineWidth) -> Self {
        Self {
            line_width: Some(line_width),
            ..self
        }
    }

    pub fn with_line_style(self, line_style: LineStyle) -> Self {
        Self {
            line_style: Some(line_style),
            ..self
        }
    }

    pub fn with_line_visible(self, line_visible: bool) -> Self {
        Self {
            line_visible: Some(line_visible),
            ..self
        }
    }

    pub fn with_axis_label_visible(self, axis_label_visible: bool) -> Self {
        Self {
            axis_label_visible: Some(axis_label_visible),
            ..self
        }
    }

    pub fn with_title(self, title: String) -> Self {
        Self {
            title: Some(title),
            ..self
        }
    }

    pub fn with_axis_label_color(self, axis_label_color: String) -> Self {
        Self {
            axis_label_color: Some(axis_label_color),
            ..self
        }
    }

    pub fn with_axis_label_text_color(self, axis_label_text_color: String) -> Self {
        Self {
            axis_label_text_color: Some(axis_label_text_color),
            ..self
        }
    }
}
