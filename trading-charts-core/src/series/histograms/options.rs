use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HistogramSeriesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_value_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_line_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_scale_id: Option<String>,
}

impl HistogramSeriesOptions {
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

    pub fn with_color(self, color: String) -> Self {
        Self {
            color: Some(color),
            ..self
        }
    }

    pub fn with_base(self, base: f64) -> Self {
        Self {
            base: Some(base),
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
