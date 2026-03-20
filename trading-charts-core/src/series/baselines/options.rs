use crate::data::options::{LastPriceAnimationMode, LineStyle, LineType, LineWidth, PriceFormatOptions, PriceLineSource};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BaselineBaseValue {
    #[serde(rename = "type")]
    kind: String,
    price: f64,
}

impl BaselineBaseValue {
    pub fn price(price: f64) -> Self {
        Self {
            kind: String::from("price"),
            price,
        }
    }
}

impl Default for BaselineBaseValue {
    fn default() -> Self {
        Self::price(0.0)
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BaselineSeriesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_value: Option<BaselineBaseValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_gradient: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_fill_color1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_fill_color2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_line_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom_fill_color1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom_fill_color2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom_line_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_width: Option<LineWidth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<LineStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_type: Option<LineType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_markers_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_markers_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crosshair_marker_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crosshair_marker_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crosshair_marker_border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crosshair_marker_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crosshair_marker_border_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_price_animation: Option<LastPriceAnimationMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_value_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_line_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_line_source: Option<PriceLineSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_line_width: Option<LineWidth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_line_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_line_style: Option<LineStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_format: Option<PriceFormatOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_scale_id: Option<String>,
}

impl BaselineSeriesOptions {
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

    pub fn with_base_value(self, base_value: BaselineBaseValue) -> Self {
        Self {
            base_value: Some(base_value),
            ..self
        }
    }

    pub fn with_relative_gradient(self, relative_gradient: bool) -> Self {
        Self {
            relative_gradient: Some(relative_gradient),
            ..self
        }
    }

    pub fn with_top_fill_color1(self, top_fill_color1: String) -> Self {
        Self {
            top_fill_color1: Some(top_fill_color1),
            ..self
        }
    }

    pub fn with_top_fill_color2(self, top_fill_color2: String) -> Self {
        Self {
            top_fill_color2: Some(top_fill_color2),
            ..self
        }
    }

    pub fn with_top_line_color(self, top_line_color: String) -> Self {
        Self {
            top_line_color: Some(top_line_color),
            ..self
        }
    }

    pub fn with_bottom_fill_color1(self, bottom_fill_color1: String) -> Self {
        Self {
            bottom_fill_color1: Some(bottom_fill_color1),
            ..self
        }
    }

    pub fn with_bottom_fill_color2(self, bottom_fill_color2: String) -> Self {
        Self {
            bottom_fill_color2: Some(bottom_fill_color2),
            ..self
        }
    }

    pub fn with_bottom_line_color(self, bottom_line_color: String) -> Self {
        Self {
            bottom_line_color: Some(bottom_line_color),
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

    pub fn with_line_type(self, line_type: LineType) -> Self {
        Self {
            line_type: Some(line_type),
            ..self
        }
    }

    pub fn with_line_visible(self, line_visible: bool) -> Self {
        Self {
            line_visible: Some(line_visible),
            ..self
        }
    }

    pub fn with_point_markers_visible(self, point_markers_visible: bool) -> Self {
        Self {
            point_markers_visible: Some(point_markers_visible),
            ..self
        }
    }

    pub fn with_point_markers_radius(self, point_markers_radius: f64) -> Self {
        Self {
            point_markers_radius: Some(point_markers_radius),
            ..self
        }
    }

    pub fn with_crosshair_marker_visible(self, crosshair_marker_visible: bool) -> Self {
        Self {
            crosshair_marker_visible: Some(crosshair_marker_visible),
            ..self
        }
    }

    pub fn with_crosshair_marker_radius(self, crosshair_marker_radius: f64) -> Self {
        Self {
            crosshair_marker_radius: Some(crosshair_marker_radius),
            ..self
        }
    }

    pub fn with_crosshair_marker_border_color(self, crosshair_marker_border_color: String) -> Self {
        Self {
            crosshair_marker_border_color: Some(crosshair_marker_border_color),
            ..self
        }
    }

    pub fn with_crosshair_marker_background_color(self, crosshair_marker_background_color: String) -> Self {
        Self {
            crosshair_marker_background_color: Some(crosshair_marker_background_color),
            ..self
        }
    }

    pub fn with_crosshair_marker_border_width(self, crosshair_marker_border_width: f64) -> Self {
        Self {
            crosshair_marker_border_width: Some(crosshair_marker_border_width),
            ..self
        }
    }

    pub fn with_last_price_animation(self, last_price_animation: LastPriceAnimationMode) -> Self {
        Self {
            last_price_animation: Some(last_price_animation),
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

    pub fn with_price_line_source(self, price_line_source: PriceLineSource) -> Self {
        Self {
            price_line_source: Some(price_line_source),
            ..self
        }
    }

    pub fn with_price_line_width(self, price_line_width: LineWidth) -> Self {
        Self {
            price_line_width: Some(price_line_width),
            ..self
        }
    }

    pub fn with_price_line_color(self, price_line_color: String) -> Self {
        Self {
            price_line_color: Some(price_line_color),
            ..self
        }
    }

    pub fn with_price_line_style(self, price_line_style: LineStyle) -> Self {
        Self {
            price_line_style: Some(price_line_style),
            ..self
        }
    }

    pub fn with_price_format(self, price_format: PriceFormatOptions) -> Self {
        Self {
            price_format: Some(price_format),
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
