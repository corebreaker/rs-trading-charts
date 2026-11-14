use crate::data::format::FontFamily;
use serde::{Deserialize, Serialize};

/// Layout of the built-in legend.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LegendLayout {
    /// The legend text, then one row per series with its value and a visibility toggle.
    #[default]
    Series,

    /// Three lines for a single series: its title (or the legend text), its price in a larger font and the time of
    /// the price, following the crosshair.
    ThreeLines,
}

/// Corner of the chart where the built-in legend is placed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LegendPosition {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Options of the built-in legend of a chart.
///
/// The `left` and `top` offsets are the horizontal and vertical distances to the corner given by the position. For
/// a custom legend, use an overlay component of the framework adapter instead.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LegendOptions {
    visible:                  bool,
    show_ohlc:                bool,
    show_percent:             bool,
    show_series:              bool,
    show_volume:              bool,
    toggle_series_visibility: bool,
    text:                     String,
    text_color:               String,
    background_color:         String,
    font_size:                f64,
    font_family:              FontFamily,
    top:                      f64,
    left:                     f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    layout:                   Option<LegendLayout>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    position:                 Option<LegendPosition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    series_id:                Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    time_format:              Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    locale:                   Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_font_size:          Option<f64>,
}

impl LegendOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_visible(self, visible: bool) -> Self {
        Self {
            visible,
            ..self
        }
    }

    pub fn with_show_ohlc(self, show_ohlc: bool) -> Self {
        Self {
            show_ohlc,
            ..self
        }
    }

    pub fn with_show_percent(self, show_percent: bool) -> Self {
        Self {
            show_percent,
            ..self
        }
    }

    pub fn with_show_series(self, show_series: bool) -> Self {
        Self {
            show_series,
            ..self
        }
    }

    pub fn with_show_volume(self, show_volume: bool) -> Self {
        Self {
            show_volume,
            ..self
        }
    }

    pub fn with_toggle_series_visibility(self, toggle_series_visibility: bool) -> Self {
        Self {
            toggle_series_visibility,
            ..self
        }
    }

    pub fn with_text(self, text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
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
        Self {
            font_size,
            ..self
        }
    }

    pub fn with_font_family(self, font_family: impl Into<FontFamily>) -> Self {
        Self {
            font_family: font_family.into(),
            ..self
        }
    }

    pub fn with_top(self, top: f64) -> Self {
        Self {
            top,
            ..self
        }
    }

    pub fn with_left(self, left: f64) -> Self {
        Self {
            left,
            ..self
        }
    }

    /// Layout of the legend. Default: [`LegendLayout::Series`].
    pub fn with_layout(self, layout: LegendLayout) -> Self {
        Self {
            layout: Some(layout),
            ..self
        }
    }

    /// Corner of the chart where the legend is placed. Default: [`LegendPosition::TopLeft`].
    pub fn with_position(self, position: LegendPosition) -> Self {
        Self {
            position: Some(position),
            ..self
        }
    }

    /// Series shown by the [`LegendLayout::ThreeLines`] layout. Default: the first series.
    pub fn with_series_id(self, series_id: impl Into<String>) -> Self {
        Self {
            series_id: Some(series_id.into()),
            ..self
        }
    }

    /// Pattern of the time of the [`LegendLayout::ThreeLines`] layout, with the tokens of
    /// [`TimeFormatter`](crate::data::format::TimeFormatter). Default: `yyyy-MM-dd HH:mm`.
    pub fn with_time_format(self, time_format: impl Into<String>) -> Self {
        Self {
            time_format: Some(time_format.into()),
            ..self
        }
    }

    /// Locale of the month and day names of the time format, such as `fr-FR`. Default: the browser locale.
    pub fn with_locale(self, locale: impl Into<String>) -> Self {
        Self {
            locale: Some(locale.into()),
            ..self
        }
    }

    /// Font size of the price of the [`LegendLayout::ThreeLines`] layout. Default: twice the font size.
    pub fn with_price_font_size(self, price_font_size: f64) -> Self {
        Self {
            price_font_size: Some(price_font_size),
            ..self
        }
    }
}

impl Default for LegendOptions {
    fn default() -> Self {
        Self {
            visible:                  true,
            show_ohlc:                true,
            show_percent:             true,
            show_series:              true,
            show_volume:              true,
            toggle_series_visibility: true,
            text:                     String::new(),
            text_color:               String::from("#0f172a"),
            background_color:         String::from("rgba(0, 0, 0, 0)"),
            font_size:                12.0,
            font_family:              FontFamily::from(
                "Avenir Next, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
            ),
            top:                      10.0,
            left:                     10.0,
            layout:                   None,
            position:                 None,
            series_id:                None,
            time_format:              None,
            locale:                   None,
            price_font_size:          None,
        }
    }
}
