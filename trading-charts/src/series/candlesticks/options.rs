use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CandlestickOptions {
    #[serde(default = "String::new", skip_serializing_if = "String::is_empty")]
    title: String,

    #[serde(
        rename = "lastValueVisible",
        skip_serializing_if = "Option::is_none",
        default = "default_options::last_value_visible"
    )]
    last_value_visible: Option<bool>,

    #[serde(default = "default_options::visible", skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,

    #[serde(
        rename = "priceLineVisible",
        skip_serializing_if = "Option::is_none",
        default = "default_options::price_line_visible"
    )]
    price_line_visible: Option<bool>,

    #[serde(rename = "upColor", skip_serializing_if = "Option::is_none")]
    up_color: Option<String>,

    #[serde(rename = "down_color", skip_serializing_if = "Option::is_none")]
    down_color: Option<String>,

    #[serde(
        rename = "wickVisible",
        skip_serializing_if = "Option::is_none",
        default = "default_options::wick_visible"
    )]
    wick_visible: Option<bool>,

    #[serde(
        rename = "borderVisible",
        skip_serializing_if = "Option::is_none",
        default = "default_options::border_visible"
    )]
    border_visible: Option<bool>,

    #[serde(rename = "borderColor", skip_serializing_if = "Option::is_none")]
    border_color: Option<String>,

    #[serde(rename = "borderUpColor", skip_serializing_if = "Option::is_none")]
    border_up_color: Option<String>,

    #[serde(rename = "borderDownColor", skip_serializing_if = "Option::is_none")]
    border_down_color: Option<String>,

    #[serde(rename = "wickColor", skip_serializing_if = "Option::is_none")]
    wick_color: Option<String>,

    #[serde(rename = "wickUpColor", skip_serializing_if = "Option::is_none")]
    wick_up_color: Option<String>,

    #[serde(rename = "wickDownColor", skip_serializing_if = "Option::is_none")]
    wick_down_color: Option<String>,
}

impl CandlestickOptions {
    pub fn new() -> Self {
        Self {
            title:              String::new(),
            last_value_visible: None,
            visible:            None,
            price_line_visible: None,
            up_color:           None,
            down_color:         None,
            wick_visible:       None,
            border_visible:     None,
            border_color:       None,
            border_up_color:    None,
            border_down_color:  None,
            wick_color:         None,
            wick_up_color:      None,
            wick_down_color:    None,
        }
    }

    pub fn with_title(self, title: String) -> Self {
        Self {
            title,
            ..self
        }
    }

    pub fn with_last_value_visible(self, last_value_visible: bool) -> Self {
        Self {
            last_value_visible: Some(last_value_visible),
            ..self
        }
    }

    pub fn with_visible(self, visible: bool) -> Self {
        Self {
            visible: Some(visible),
            ..self
        }
    }

    pub fn with_price_line_visible(self, price_line_visible: bool) -> Self {
        Self {
            price_line_visible: Some(price_line_visible),
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

    pub fn with_wick_visible(self, wick_visible: bool) -> Self {
        Self {
            wick_visible: Some(wick_visible),
            ..self
        }
    }

    pub fn with_border_visible(self, border_visible: bool) -> Self {
        Self {
            border_visible: Some(border_visible),
            ..self
        }
    }

    pub fn with_border_color(self, border_color: String) -> Self {
        Self {
            border_color: Some(border_color),
            ..self
        }
    }

    pub fn with_border_up_color(self, border_up_color: String) -> Self {
        Self {
            border_up_color: Some(border_up_color),
            ..self
        }
    }

    pub fn with_border_down_color(self, border_down_color: String) -> Self {
        Self {
            border_down_color: Some(border_down_color),
            ..self
        }
    }

    pub fn with_wick_color(self, wick_color: String) -> Self {
        Self {
            wick_color: Some(wick_color),
            ..self
        }
    }

    pub fn with_wick_up_color(self, wick_up_color: String) -> Self {
        Self {
            wick_up_color: Some(wick_up_color),
            ..self
        }
    }

    pub fn with_wick_down_color(self, wick_down_color: String) -> Self {
        Self {
            wick_down_color: Some(wick_down_color),
            ..self
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn last_value_visible(&self) -> Option<bool> {
        self.last_value_visible
    }

    pub fn visible(&self) -> Option<bool> {
        self.visible
    }

    pub fn price_line_visible(&self) -> Option<bool> {
        self.price_line_visible
    }

    pub fn up_color(&self) -> Option<&String> {
        self.up_color.as_ref()
    }

    pub fn down_color(&self) -> Option<&String> {
        self.down_color.as_ref()
    }

    pub fn wick_visible(&self) -> Option<bool> {
        self.wick_visible
    }

    pub fn border_visible(&self) -> Option<bool> {
        self.border_visible
    }

    pub fn border_color(&self) -> Option<&String> {
        self.border_color.as_ref()
    }

    pub fn border_up_color(&self) -> Option<&String> {
        self.border_up_color.as_ref()
    }

    pub fn border_down_color(&self) -> Option<&String> {
        self.border_down_color.as_ref()
    }

    pub fn wick_color(&self) -> Option<&String> {
        self.wick_color.as_ref()
    }

    pub fn wick_up_color(&self) -> Option<&String> {
        self.wick_up_color.as_ref()
    }

    pub fn wick_down_color(&self) -> Option<&String> {
        self.wick_down_color.as_ref()
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_last_value_visible(&mut self, last_value_visible: bool) {
        self.last_value_visible = Some(last_value_visible);
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = Some(visible);
    }

    pub fn set_price_line_visible(&mut self, price_line_visible: bool) {
        self.price_line_visible = Some(price_line_visible);
    }

    pub fn set_up_color(&mut self, up_color: String) {
        self.up_color = Some(up_color);
    }

    pub fn set_down_color(&mut self, down_color: String) {
        self.down_color = Some(down_color);
    }

    pub fn set_wick_visible(&mut self, wick_visible: bool) {
        self.wick_visible = Some(wick_visible);
    }

    pub fn set_border_visible(&mut self, border_visible: bool) {
        self.border_visible = Some(border_visible);
    }

    pub fn set_border_color(&mut self, border_color: String) {
        self.border_color = Some(border_color);
    }

    pub fn set_border_up_color(&mut self, border_up_color: String) {
        self.border_up_color = Some(border_up_color);
    }

    pub fn set_border_down_color(&mut self, border_down_color: String) {
        self.border_down_color = Some(border_down_color);
    }

    pub fn set_wick_color(&mut self, wick_color: String) {
        self.wick_color = Some(wick_color);
    }

    pub fn set_wick_up_color(&mut self, wick_up_color: String) {
        self.wick_up_color = Some(wick_up_color);
    }

    pub fn set_wick_down_color(&mut self, wick_down_color: String) {
        self.wick_down_color = Some(wick_down_color);
    }

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    pub fn clear_last_value_visible(&mut self) {
        self.last_value_visible = None;
    }

    pub fn clear_visible(&mut self) {
        self.visible = None;
    }

    pub fn clear_price_line_visible(&mut self) {
        self.price_line_visible = None;
    }

    pub fn clear_up_color(&mut self) {
        self.up_color = None;
    }

    pub fn clear_down_color(&mut self) {
        self.down_color = None;
    }

    pub fn clear_wick_visible(&mut self) {
        self.wick_visible = None;
    }

    pub fn clear_border_visible(&mut self) {
        self.border_visible = None;
    }

    pub fn clear_border_color(&mut self) {
        self.border_color = None;
    }

    pub fn clear_border_up_color(&mut self) {
        self.border_up_color = None;
    }

    pub fn clear_border_down_color(&mut self) {
        self.border_down_color = None;
    }

    pub fn clear_wick_color(&mut self) {
        self.wick_color = None;
    }

    pub fn clear_wick_up_color(&mut self) {
        self.wick_up_color = None;
    }

    pub fn clear_wick_down_color(&mut self) {
        self.wick_down_color = None;
    }

    pub fn is_empty(&self) -> bool {
        self.title.is_empty()
            && self.last_value_visible.is_none()
            && self.visible.is_none()
            && self.price_line_visible.is_none()
            && self.up_color.is_none()
            && self.down_color.is_none()
            && self.wick_visible.is_none()
            && self.border_visible.is_none()
            && self.border_color.is_none()
            && self.border_up_color.is_none()
            && self.border_down_color.is_none()
            && self.wick_color.is_none()
            && self.wick_up_color.is_none()
            && self.wick_down_color.is_none()
    }
}

mod default_options {
    pub(super) fn last_value_visible() -> Option<bool> {
        Some(true)
    }

    pub(super) fn visible() -> Option<bool> {
        Some(true)
    }

    pub(super) fn price_line_visible() -> Option<bool> {
        Some(true)
    }

    pub(super) fn wick_visible() -> Option<bool> {
        Some(true)
    }

    pub(super) fn border_visible() -> Option<bool> {
        Some(true)
    }
}
