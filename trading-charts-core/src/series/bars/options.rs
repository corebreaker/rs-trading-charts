use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BarSeriesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    up_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    down_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thin_bars: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_value_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_line_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_scale_id: Option<String>,
}

impl BarSeriesOptions {
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

    pub fn with_up_color(self, up_color: String) -> Self {
        Self {
            up_color: Some(up_color),
            ..self
        }
    }

    pub fn with_down_color(self, down_color: String) -> Self {
        Self {
            down_color: Some(down_color),
            ..self
        }
    }

    pub fn with_open_visible(self, open_visible: bool) -> Self {
        Self {
            open_visible: Some(open_visible),
            ..self
        }
    }

    pub fn with_thin_bars(self, thin_bars: bool) -> Self {
        Self {
            thin_bars: Some(thin_bars),
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
