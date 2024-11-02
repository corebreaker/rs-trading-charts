use crate::data::options::{LineWidth, LineStyle, PriceLineSource};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CandlestickOptions {
    #[serde(default = "String::new")]
    title: String,

    #[serde(rename = "lastValueVisible", default = "defaults::last_value_visible")]
    last_value_visible: bool,

    #[serde(default = "defaults::visible")]
    visible: bool,

    #[serde(rename = "priceLineVisible", default = "defaults::price_line_visible")]
    price_line_visible: bool,

    #[serde(rename = "priceLineSource", default)]
    price_line_source: PriceLineSource,

    #[serde(rename = "priceLineWidth", default)]
    price_line_width: LineWidth,

    #[serde(rename = "priceLineColor", default = "defaults::price_line_color")]
    price_line_color: String,

    #[serde(rename = "priceLineStyle", default)]
    price_line_style: LineStyle,

    #[serde(rename = "baseLineVisible", default = "defaults::base_line_visible")]
    base_line_visible: bool,

    #[serde(rename = "baseLineColor", default = "defaults::base_line_color")]
    base_line_color: String,

    #[serde(rename = "baseLineWidth", default)]
    base_line_width: LineWidth,

    #[serde(rename = "baseLineStyle", default)]
    base_line_style: LineStyle,

    #[serde(rename = "upColor", default = "defaults::up_color")]
    up_color: String,

    #[serde(rename = "downColor", default = "defaults::down_color")]
    down_color: String,

    #[serde(rename = "wickVisible", default = "defaults::wick_visible")]
    wick_visible: bool,

    #[serde(rename = "borderVisible", default = "defaults::border_visible")]
    border_visible: bool,

    #[serde(rename = "borderColor", default = "defaults::border_color")]
    border_color: String,

    #[serde(rename = "borderUpColor", default = "defaults::border_up_color")]
    border_up_color: String,

    #[serde(rename = "borderDownColor", default = "defaults::border_down_color")]
    border_down_color: String,

    #[serde(rename = "wickColor", default = "defaults::wick_color")]
    wick_color: String,

    #[serde(rename = "wickUpColor", default = "defaults::wick_up_color")]
    wick_up_color: String,

    #[serde(rename = "wickDownColor", default = "defaults::wick_down_color")]
    wick_down_color: String,
}

impl CandlestickOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(self, title: String) -> Self {
        Self {
            title,
            ..self
        }
    }

    pub fn with_last_value_visible(self, last_value_visible: bool) -> Self {
        Self {
            last_value_visible,
            ..self
        }
    }

    pub fn with_visible(self, visible: bool) -> Self {
        Self {
            visible,
            ..self
        }
    }

    pub fn with_price_line_visible(self, price_line_visible: bool) -> Self {
        Self {
            price_line_visible,
            ..self
        }
    }

    pub fn with_price_line_source(self, price_line_source: PriceLineSource) -> Self {
        Self {
            price_line_source,
            ..self
        }
    }

    pub fn with_price_line_width(self, price_line_width: LineWidth) -> Self {
        Self {
            price_line_width,
            ..self
        }
    }

    pub fn with_price_line_color(self, price_line_color: String) -> Self {
        Self {
            price_line_color,
            ..self
        }
    }

    pub fn with_price_line_style(self, price_line_style: LineStyle) -> Self {
        Self {
            price_line_style,
            ..self
        }
    }

    pub fn with_base_line_visible(self, base_line_visible: bool) -> Self {
        Self {
            base_line_visible,
            ..self
        }
    }

    pub fn with_base_line_color(self, base_line_color: String) -> Self {
        Self {
            base_line_color,
            ..self
        }
    }

    pub fn with_base_line_width(self, base_line_width: LineWidth) -> Self {
        Self {
            base_line_width,
            ..self
        }
    }

    pub fn with_base_line_style(self, base_line_style: LineStyle) -> Self {
        Self {
            base_line_style,
            ..self
        }
    }

    pub fn with_up_color(self, up_color: String) -> Self {
        Self {
            up_color,
            ..self
        }
    }

    pub fn with_down_color(self, down_color: String) -> Self {
        Self {
            down_color,
            ..self
        }
    }

    pub fn with_wick_visible(self, wick_visible: bool) -> Self {
        Self {
            wick_visible,
            ..self
        }
    }

    pub fn with_border_visible(self, border_visible: bool) -> Self {
        Self {
            border_visible,
            ..self
        }
    }

    pub fn with_border_color(self, border_color: String) -> Self {
        Self {
            border_color,
            ..self
        }
    }

    pub fn with_border_up_color(self, border_up_color: String) -> Self {
        Self {
            border_up_color,
            ..self
        }
    }

    pub fn with_border_down_color(self, border_down_color: String) -> Self {
        Self {
            border_down_color,
            ..self
        }
    }

    pub fn with_wick_color(self, wick_color: String) -> Self {
        Self {
            wick_color,
            ..self
        }
    }

    pub fn with_wick_up_color(self, wick_up_color: String) -> Self {
        Self {
            wick_up_color,
            ..self
        }
    }

    pub fn with_wick_down_color(self, wick_down_color: String) -> Self {
        Self {
            wick_down_color,
            ..self
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn title_mut(&mut self) -> &mut String {
        &mut self.title
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn last_value_visible(&self) -> bool {
        self.last_value_visible
    }

    pub fn set_last_value_visible(&mut self, last_value_visible: bool) {
        self.last_value_visible = last_value_visible;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn price_line_visible(&self) -> bool {
        self.price_line_visible
    }

    pub fn set_price_line_visible(&mut self, price_line_visible: bool) {
        self.price_line_visible = price_line_visible;
    }

    pub fn price_line_source(&self) -> PriceLineSource {
        self.price_line_source
    }

    pub fn set_price_line_source(&mut self, price_line_source: PriceLineSource) {
        self.price_line_source = price_line_source;
    }

    pub fn price_line_width(&self) -> LineWidth {
        self.price_line_width
    }

    pub fn set_price_line_width(&mut self, price_line_width: LineWidth) {
        self.price_line_width = price_line_width;
    }

    pub fn price_line_color(&self) -> &str {
        &self.price_line_color
    }

    pub fn price_line_color_mut(&mut self) -> &mut String {
        &mut self.price_line_color
    }

    pub fn set_price_line_color(&mut self, price_line_color: String) {
        self.price_line_color = price_line_color;
    }

    pub fn price_line_style(&self) -> LineStyle {
        self.price_line_style
    }

    pub fn set_price_line_style(&mut self, price_line_style: LineStyle) {
        self.price_line_style = price_line_style;
    }

    pub fn base_line_visible(&self) -> bool {
        self.base_line_visible
    }

    pub fn set_base_line_visible(&mut self, base_line_visible: bool) {
        self.base_line_visible = base_line_visible;
    }

    pub fn base_line_color(&self) -> &str {
        &self.base_line_color
    }

    pub fn base_line_color_mut(&mut self) -> &mut String {
        &mut self.base_line_color
    }

    pub fn set_base_line_color(&mut self, base_line_color: String) {
        self.base_line_color = base_line_color;
    }

    pub fn base_line_width(&self) -> LineWidth {
        self.base_line_width
    }

    pub fn set_base_line_width(&mut self, base_line_width: LineWidth) {
        self.base_line_width = base_line_width;
    }

    pub fn base_line_style(&self) -> LineStyle {
        self.base_line_style
    }

    pub fn set_base_line_style(&mut self, base_line_style: LineStyle) {
        self.base_line_style = base_line_style;
    }

    pub fn up_color(&self) -> &str {
        &self.up_color
    }

    pub fn up_color_mut(&mut self) -> &mut String {
        &mut self.up_color
    }

    pub fn set_up_color(&mut self, up_color: String) {
        self.up_color = up_color;
    }

    pub fn down_color(&self) -> &str {
        &self.down_color
    }

    pub fn down_color_mut(&mut self) -> &mut String {
        &mut self.down_color
    }

    pub fn set_down_color(&mut self, down_color: String) {
        self.down_color = down_color;
    }

    pub fn wick_visible(&self) -> bool {
        self.wick_visible
    }

    pub fn set_wick_visible(&mut self, wick_visible: bool) {
        self.wick_visible = wick_visible;
    }

    pub fn border_visible(&self) -> bool {
        self.border_visible
    }

    pub fn set_border_visible(&mut self, border_visible: bool) {
        self.border_visible = border_visible;
    }

    pub fn border_color(&self) -> &str {
        &self.border_color
    }

    pub fn border_color_mut(&mut self) -> &mut String {
        &mut self.border_color
    }

    pub fn set_border_color(&mut self, border_color: String) {
        self.border_color = border_color;
    }

    pub fn border_up_color(&self) -> &str {
        &self.border_up_color
    }

    pub fn border_up_color_mut(&mut self) -> &mut String {
        &mut self.border_up_color
    }

    pub fn set_border_up_color(&mut self, border_up_color: String) {
        self.border_up_color = border_up_color;
    }

    pub fn border_down_color(&self) -> &str {
        &self.border_down_color
    }

    pub fn border_down_color_mut(&mut self) -> &mut String {
        &mut self.border_down_color
    }

    pub fn set_border_down_color(&mut self, border_down_color: String) {
        self.border_down_color = border_down_color;
    }

    pub fn wick_color(&self) -> &str {
        &self.wick_color
    }

    pub fn wick_color_mut(&mut self) -> &mut String {
        &mut self.wick_color
    }

    pub fn set_wick_color(&mut self, wick_color: String) {
        self.wick_color = wick_color;
    }

    pub fn wick_up_color(&self) -> &str {
        &self.wick_up_color
    }

    pub fn wick_up_color_mut(&mut self) -> &mut String {
        &mut self.wick_up_color
    }

    pub fn set_wick_up_color(&mut self, wick_up_color: String) {
        self.wick_up_color = wick_up_color;
    }

    pub fn wick_down_color(&self) -> &str {
        &self.wick_down_color
    }

    pub fn wick_down_color_mut(&mut self) -> &mut String {
        &mut self.wick_down_color
    }

    pub fn set_wick_down_color(&mut self, wick_down_color: String) {
        self.wick_down_color = wick_down_color;
    }
}

impl Default for CandlestickOptions {
    fn default() -> Self {
        Self {
            title: String::new(),
            last_value_visible: defaults::last_value_visible(),
            visible: defaults::visible(),
            price_line_visible: defaults::price_line_visible(),
            price_line_source: PriceLineSource::default(),
            price_line_width: LineWidth::default(),
            price_line_color: defaults::price_line_color(),
            price_line_style: LineStyle::default(),
            base_line_visible: defaults::base_line_visible(),
            base_line_color: defaults::base_line_color(),
            base_line_width: LineWidth::default(),
            base_line_style: LineStyle::default(),
            up_color: defaults::up_color(),
            down_color: defaults::down_color(),
            wick_visible: defaults::wick_visible(),
            border_visible: defaults::border_visible(),
            border_color: defaults::border_color(),
            border_up_color: defaults::border_up_color(),
            border_down_color: defaults::border_down_color(),
            wick_color: defaults::wick_color(),
            wick_up_color: defaults::wick_up_color(),
            wick_down_color: defaults::wick_down_color(),
        }
    }
}

mod defaults {
    pub(super) fn last_value_visible() -> bool {
        true
    }

    pub(super) fn visible() -> bool {
        true
    }

    pub(super) fn price_line_visible() -> bool {
        true
    }

    pub(super) fn price_line_color() -> String {
        String::new()
    }

    pub(super) fn base_line_visible() -> bool {
        true
    }

    pub(super) fn base_line_color() -> String {
        String::from("#b2b5be")
    }

    pub(super) fn up_color() -> String {
        String::from("#26a69a")
    }

    pub(super) fn down_color() -> String {
        String::from("#ef5350")
    }

    pub(super) fn wick_visible() -> bool {
        true
    }

    pub(super) fn border_visible() -> bool {
        true
    }

    pub(super) fn border_color() -> String {
        String::from("#378658")
    }

    pub(super) fn border_up_color() -> String {
        String::from("#26a69a")
    }

    pub(super) fn border_down_color() -> String {
        String::from("#ef5350")
    }

    pub(super) fn wick_color() -> String {
        String::from("#737375")
    }

    pub(super) fn wick_up_color() -> String {
        String::from("#26a69a")
    }

    pub(super) fn wick_down_color() -> String {
        String::from("#ef5350")
    }
}
