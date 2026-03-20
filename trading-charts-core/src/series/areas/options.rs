use crate::data::options::{LineStyle, LineWidth};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AreaSeriesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<LineStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_width: Option<LineWidth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_filled_area: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crosshair_marker_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_value_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_line_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_scale_id: Option<String>,
}

impl AreaSeriesOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(self, title: String) -> Self {
        Self {
            title: Some(title),
            ..self
        }
    }

    pub fn with_visible(self, visible: bool) -> Self {
        Self {
            visible: Some(visible),
            ..self
        }
    }

    pub fn with_top_color(self, top_color: String) -> Self {
        Self {
            top_color: Some(top_color),
            ..self
        }
    }

    pub fn with_bottom_color(self, bottom_color: String) -> Self {
        Self {
            bottom_color: Some(bottom_color),
            ..self
        }
    }

    pub fn with_line_color(self, line_color: String) -> Self {
        Self {
            line_color: Some(line_color),
            ..self
        }
    }

    pub fn with_line_style(self, line_style: LineStyle) -> Self {
        Self {
            line_style: Some(line_style),
            ..self
        }
    }

    pub fn with_line_width(self, line_width: LineWidth) -> Self {
        Self {
            line_width: Some(line_width),
            ..self
        }
    }

    pub fn with_invert_filled_area(self, invert_filled_area: bool) -> Self {
        Self {
            invert_filled_area: Some(invert_filled_area),
            ..self
        }
    }

    pub fn with_crosshair_marker_visible(self, crosshair_marker_visible: bool) -> Self {
        Self {
            crosshair_marker_visible: Some(crosshair_marker_visible),
            ..self
        }
    }

    pub fn with_last_value_visible(self, last_value_visible: bool) -> Self {
        Self {
            last_value_visible: Some(last_value_visible),
            ..self
        }
    }

    pub fn with_price_line_visible(self, price_line_visible: bool) -> Self {
        Self {
            price_line_visible: Some(price_line_visible),
            ..self
        }
    }

    pub fn with_price_scale_id(self, price_scale_id: String) -> Self {
        Self {
            price_scale_id: Some(price_scale_id),
            ..self
        }
    }
}
