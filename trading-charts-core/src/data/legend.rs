use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LegendOptions {
    visible: bool,
    show_ohlc: bool,
    show_percent: bool,
    show_series: bool,
    show_volume: bool,
    toggle_series_visibility: bool,
    text_color: String,
    background_color: String,
    font_size: f64,
    font_family: String,
    top: f64,
    left: f64,
}

impl LegendOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_visible(self, visible: bool) -> Self {
        Self { visible, ..self }
    }

    pub fn with_show_ohlc(self, show_ohlc: bool) -> Self {
        Self { show_ohlc, ..self }
    }

    pub fn with_show_percent(self, show_percent: bool) -> Self {
        Self { show_percent, ..self }
    }

    pub fn with_show_series(self, show_series: bool) -> Self {
        Self { show_series, ..self }
    }

    pub fn with_show_volume(self, show_volume: bool) -> Self {
        Self { show_volume, ..self }
    }

    pub fn with_toggle_series_visibility(self, toggle_series_visibility: bool) -> Self {
        Self {
            toggle_series_visibility,
            ..self
        }
    }

    pub fn with_text_color(self, text_color: impl Into<String>) -> Self {
        Self {
            text_color: text_color.into(),
            ..self
        }
    }

    pub fn with_background_color(self, background_color: impl Into<String>) -> Self {
        Self {
            background_color: background_color.into(),
            ..self
        }
    }

    pub fn with_font_size(self, font_size: f64) -> Self {
        Self { font_size, ..self }
    }

    pub fn with_font_family(self, font_family: impl Into<String>) -> Self {
        Self {
            font_family: font_family.into(),
            ..self
        }
    }

    pub fn with_top(self, top: f64) -> Self {
        Self { top, ..self }
    }

    pub fn with_left(self, left: f64) -> Self {
        Self { left, ..self }
    }
}

impl Default for LegendOptions {
    fn default() -> Self {
        Self {
            visible: true,
            show_ohlc: true,
            show_percent: true,
            show_series: true,
            show_volume: true,
            toggle_series_visibility: true,
            text_color: String::from("#0f172a"),
            background_color: String::from("rgba(255, 255, 255, 0.78)"),
            font_size: 12.0,
            font_family: String::from("-apple-system, BlinkMacSystemFont, 'Trebuchet MS', Roboto, Ubuntu, sans-serif"),
            top: 12.0,
            left: 12.0,
        }
    }
}
