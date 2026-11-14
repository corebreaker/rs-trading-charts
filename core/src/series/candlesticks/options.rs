use {
    crate::data::options::LineStyle,
    crate::data::options::LineWidth,
    crate::data::options::PriceFormatOptions,
    crate::data::options::PriceLineSource,
};
use serde::{Deserialize, Serialize};

/// Represents style options for a candlestick series.
///
/// The options common to every series type are included.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandlestickOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    up_color:                    Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    down_color:                  Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    wick_visible:                Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    border_visible:              Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    border_color:                Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    border_up_color:             Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    border_down_color:           Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    wick_color:                  Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    wick_up_color:               Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    wick_down_color:             Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    last_value_visible:          Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    title:                       Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_scale_id:              Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    visible:                     Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hit_test_tolerance:          Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_line_visible:          Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_line_source:           Option<PriceLineSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_line_width:            Option<LineWidth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_line_color:            Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_line_style:            Option<LineStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_format:                Option<PriceFormatOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base_line_visible:           Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base_line_color:             Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base_line_width:             Option<LineWidth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base_line_style:             Option<LineStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    conflation_threshold_factor: Option<f64>,
}

impl CandlestickOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Color of rising candles.
    ///
    /// Default: `"#26a69a"`.
    pub fn with_up_color(self, up_color: impl Into<String>) -> Self {
        Self {
            up_color: Some(up_color.into()),
            ..self
        }
    }

    pub fn up_color(&self) -> Option<&str> {
        self.up_color.as_deref()
    }

    /// Set or reset (with `None`) the `upColor` option.
    pub fn set_up_color(&mut self, up_color: Option<String>) {
        self.up_color = up_color;
    }

    /// Color of falling candles.
    ///
    /// Default: `"#ef5350"`.
    pub fn with_down_color(self, down_color: impl Into<String>) -> Self {
        Self {
            down_color: Some(down_color.into()),
            ..self
        }
    }

    pub fn down_color(&self) -> Option<&str> {
        self.down_color.as_deref()
    }

    /// Set or reset (with `None`) the `downColor` option.
    pub fn set_down_color(&mut self, down_color: Option<String>) {
        self.down_color = down_color;
    }

    /// Enable high and low prices candle wicks.
    ///
    /// Default: `true`.
    pub fn with_wick_visible(self, wick_visible: bool) -> Self {
        Self {
            wick_visible: Some(wick_visible),
            ..self
        }
    }

    pub fn wick_visible(&self) -> Option<bool> {
        self.wick_visible
    }

    /// Set or reset (with `None`) the `wickVisible` option.
    pub fn set_wick_visible(&mut self, wick_visible: Option<bool>) {
        self.wick_visible = wick_visible;
    }

    /// Enable candle borders.
    ///
    /// Default: `true`.
    pub fn with_border_visible(self, border_visible: bool) -> Self {
        Self {
            border_visible: Some(border_visible),
            ..self
        }
    }

    pub fn border_visible(&self) -> Option<bool> {
        self.border_visible
    }

    /// Set or reset (with `None`) the `borderVisible` option.
    pub fn set_border_visible(&mut self, border_visible: Option<bool>) {
        self.border_visible = border_visible;
    }

    /// Border color.
    ///
    /// Default: `"#378658"`.
    pub fn with_border_color(self, border_color: impl Into<String>) -> Self {
        Self {
            border_color: Some(border_color.into()),
            ..self
        }
    }

    pub fn border_color(&self) -> Option<&str> {
        self.border_color.as_deref()
    }

    /// Set or reset (with `None`) the `borderColor` option.
    pub fn set_border_color(&mut self, border_color: Option<String>) {
        self.border_color = border_color;
    }

    /// Border color of rising candles.
    ///
    /// Default: `"#26a69a"`.
    pub fn with_border_up_color(self, border_up_color: impl Into<String>) -> Self {
        Self {
            border_up_color: Some(border_up_color.into()),
            ..self
        }
    }

    pub fn border_up_color(&self) -> Option<&str> {
        self.border_up_color.as_deref()
    }

    /// Set or reset (with `None`) the `borderUpColor` option.
    pub fn set_border_up_color(&mut self, border_up_color: Option<String>) {
        self.border_up_color = border_up_color;
    }

    /// Border color of falling candles.
    ///
    /// Default: `"#ef5350"`.
    pub fn with_border_down_color(self, border_down_color: impl Into<String>) -> Self {
        Self {
            border_down_color: Some(border_down_color.into()),
            ..self
        }
    }

    pub fn border_down_color(&self) -> Option<&str> {
        self.border_down_color.as_deref()
    }

    /// Set or reset (with `None`) the `borderDownColor` option.
    pub fn set_border_down_color(&mut self, border_down_color: Option<String>) {
        self.border_down_color = border_down_color;
    }

    /// Wick color.
    ///
    /// Default: `"#737375"`.
    pub fn with_wick_color(self, wick_color: impl Into<String>) -> Self {
        Self {
            wick_color: Some(wick_color.into()),
            ..self
        }
    }

    pub fn wick_color(&self) -> Option<&str> {
        self.wick_color.as_deref()
    }

    /// Set or reset (with `None`) the `wickColor` option.
    pub fn set_wick_color(&mut self, wick_color: Option<String>) {
        self.wick_color = wick_color;
    }

    /// Wick color of rising candles.
    ///
    /// Default: `"#26a69a"`.
    pub fn with_wick_up_color(self, wick_up_color: impl Into<String>) -> Self {
        Self {
            wick_up_color: Some(wick_up_color.into()),
            ..self
        }
    }

    pub fn wick_up_color(&self) -> Option<&str> {
        self.wick_up_color.as_deref()
    }

    /// Set or reset (with `None`) the `wickUpColor` option.
    pub fn set_wick_up_color(&mut self, wick_up_color: Option<String>) {
        self.wick_up_color = wick_up_color;
    }

    /// Wick color of falling candles.
    ///
    /// Default: `"#ef5350"`.
    pub fn with_wick_down_color(self, wick_down_color: impl Into<String>) -> Self {
        Self {
            wick_down_color: Some(wick_down_color.into()),
            ..self
        }
    }

    pub fn wick_down_color(&self) -> Option<&str> {
        self.wick_down_color.as_deref()
    }

    /// Set or reset (with `None`) the `wickDownColor` option.
    pub fn set_wick_down_color(&mut self, wick_down_color: Option<String>) {
        self.wick_down_color = wick_down_color;
    }

    /// Visibility of the label with the latest visible price on the price scale.
    ///
    /// Default: `true`, `false` for yield curve charts`.
    pub fn with_last_value_visible(self, last_value_visible: bool) -> Self {
        Self {
            last_value_visible: Some(last_value_visible),
            ..self
        }
    }

    pub fn last_value_visible(&self) -> Option<bool> {
        self.last_value_visible
    }

    /// Set or reset (with `None`) the `lastValueVisible` option.
    pub fn set_last_value_visible(&mut self, last_value_visible: Option<bool>) {
        self.last_value_visible = last_value_visible;
    }

    /// You can name series when adding it to a chart. This name will be displayed on the label next to the last value
    /// label.
    ///
    /// Default: `""`.
    pub fn with_title(self, title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            ..self
        }
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Set or reset (with `None`) the `title` option.
    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    /// Target price scale to bind new series to.
    ///
    /// Default: `'right'` if right scale is visible and `'left'` otherwise`.
    pub fn with_price_scale_id(self, price_scale_id: impl Into<String>) -> Self {
        Self {
            price_scale_id: Some(price_scale_id.into()),
            ..self
        }
    }

    pub fn price_scale_id(&self) -> Option<&str> {
        self.price_scale_id.as_deref()
    }

    /// Set or reset (with `None`) the `priceScaleId` option.
    pub fn set_price_scale_id(&mut self, price_scale_id: Option<String>) {
        self.price_scale_id = price_scale_id;
    }

    /// Visibility of the series. If the series is hidden, everything including price lines, baseline,
    /// price labels and markers, will also be hidden. Please note that hiding a series is not equivalent to deleting
    /// it, since hiding does not affect the timeline at all, unlike deleting where the timeline can be changed (some
    /// points can be deleted).
    ///
    /// Default: `true`.
    pub fn with_visible(self, visible: bool) -> Self {
        Self {
            visible: Some(visible),
            ..self
        }
    }

    pub fn visible(&self) -> Option<bool> {
        self.visible
    }

    /// Set or reset (with `None`) the `visible` option.
    pub fn set_visible(&mut self, visible: Option<bool>) {
        self.visible = visible;
    }

    /// Additional tolerance in pixels used when hit testing the series.
    ///
    /// Default: `3`.
    pub fn with_hit_test_tolerance(self, hit_test_tolerance: impl Into<f64>) -> Self {
        Self {
            hit_test_tolerance: Some(hit_test_tolerance.into()),
            ..self
        }
    }

    pub fn hit_test_tolerance(&self) -> Option<f64> {
        self.hit_test_tolerance
    }

    /// Set or reset (with `None`) the `hitTestTolerance` option.
    pub fn set_hit_test_tolerance(&mut self, hit_test_tolerance: Option<f64>) {
        self.hit_test_tolerance = hit_test_tolerance;
    }

    /// Show the price line. Price line is a horizontal line indicating the last price of the series.
    ///
    /// Default: `true`, `false` for yield curve charts`.
    pub fn with_price_line_visible(self, price_line_visible: bool) -> Self {
        Self {
            price_line_visible: Some(price_line_visible),
            ..self
        }
    }

    pub fn price_line_visible(&self) -> Option<bool> {
        self.price_line_visible
    }

    /// Set or reset (with `None`) the `priceLineVisible` option.
    pub fn set_price_line_visible(&mut self, price_line_visible: Option<bool>) {
        self.price_line_visible = price_line_visible;
    }

    /// The source to use for the value of the price line.
    ///
    /// Default: `PriceLineSource.LastBar`.
    pub fn with_price_line_source(self, price_line_source: PriceLineSource) -> Self {
        Self {
            price_line_source: Some(price_line_source),
            ..self
        }
    }

    pub fn price_line_source(&self) -> Option<PriceLineSource> {
        self.price_line_source
    }

    /// Set or reset (with `None`) the `priceLineSource` option.
    pub fn set_price_line_source(&mut self, price_line_source: Option<PriceLineSource>) {
        self.price_line_source = price_line_source;
    }

    /// Width of the price line.
    ///
    /// Default: `1`.
    pub fn with_price_line_width(self, price_line_width: LineWidth) -> Self {
        Self {
            price_line_width: Some(price_line_width),
            ..self
        }
    }

    pub fn price_line_width(&self) -> Option<LineWidth> {
        self.price_line_width
    }

    /// Set or reset (with `None`) the `priceLineWidth` option.
    pub fn set_price_line_width(&mut self, price_line_width: Option<LineWidth>) {
        self.price_line_width = price_line_width;
    }

    /// Color of the price line. By default, its color is set by the last bar color (or by line color on Line and Area
    /// charts).
    ///
    /// Default: `""`.
    pub fn with_price_line_color(self, price_line_color: impl Into<String>) -> Self {
        Self {
            price_line_color: Some(price_line_color.into()),
            ..self
        }
    }

    pub fn price_line_color(&self) -> Option<&str> {
        self.price_line_color.as_deref()
    }

    /// Set or reset (with `None`) the `priceLineColor` option.
    pub fn set_price_line_color(&mut self, price_line_color: Option<String>) {
        self.price_line_color = price_line_color;
    }

    /// Price line style.
    ///
    /// Default: `LineStyle.Dashed`.
    pub fn with_price_line_style(self, price_line_style: LineStyle) -> Self {
        Self {
            price_line_style: Some(price_line_style),
            ..self
        }
    }

    pub fn price_line_style(&self) -> Option<LineStyle> {
        self.price_line_style
    }

    /// Set or reset (with `None`) the `priceLineStyle` option.
    pub fn set_price_line_style(&mut self, price_line_style: Option<LineStyle>) {
        self.price_line_style = price_line_style;
    }

    /// Price format.
    ///
    /// Default: `{ type: 'price', precision: 2, minMove: 0.01 }`.
    pub fn with_price_format(self, price_format: PriceFormatOptions) -> Self {
        Self {
            price_format: Some(price_format),
            ..self
        }
    }

    pub fn price_format(&self) -> Option<&PriceFormatOptions> {
        self.price_format.as_ref()
    }

    /// Set or reset (with `None`) the `priceFormat` option.
    pub fn set_price_format(&mut self, price_format: Option<PriceFormatOptions>) {
        self.price_format = price_format;
    }

    /// Visibility of base line. Suitable for percentage and `IndexedTo100` scales.
    ///
    /// Default: `true`.
    pub fn with_base_line_visible(self, base_line_visible: bool) -> Self {
        Self {
            base_line_visible: Some(base_line_visible),
            ..self
        }
    }

    pub fn base_line_visible(&self) -> Option<bool> {
        self.base_line_visible
    }

    /// Set or reset (with `None`) the `baseLineVisible` option.
    pub fn set_base_line_visible(&mut self, base_line_visible: Option<bool>) {
        self.base_line_visible = base_line_visible;
    }

    /// Color of the base line in `IndexedTo100` mode.
    ///
    /// Default: `"#B2B5BE"`.
    pub fn with_base_line_color(self, base_line_color: impl Into<String>) -> Self {
        Self {
            base_line_color: Some(base_line_color.into()),
            ..self
        }
    }

    pub fn base_line_color(&self) -> Option<&str> {
        self.base_line_color.as_deref()
    }

    /// Set or reset (with `None`) the `baseLineColor` option.
    pub fn set_base_line_color(&mut self, base_line_color: Option<String>) {
        self.base_line_color = base_line_color;
    }

    /// Base line width. Suitable for percentage and `IndexedTo10` scales.
    ///
    /// Default: `1`.
    pub fn with_base_line_width(self, base_line_width: LineWidth) -> Self {
        Self {
            base_line_width: Some(base_line_width),
            ..self
        }
    }

    pub fn base_line_width(&self) -> Option<LineWidth> {
        self.base_line_width
    }

    /// Set or reset (with `None`) the `baseLineWidth` option.
    pub fn set_base_line_width(&mut self, base_line_width: Option<LineWidth>) {
        self.base_line_width = base_line_width;
    }

    /// Base line style. Suitable for percentage and indexedTo100 scales.
    ///
    /// Default: `LineStyle.Solid`.
    pub fn with_base_line_style(self, base_line_style: LineStyle) -> Self {
        Self {
            base_line_style: Some(base_line_style),
            ..self
        }
    }

    pub fn base_line_style(&self) -> Option<LineStyle> {
        self.base_line_style
    }

    /// Set or reset (with `None`) the `baseLineStyle` option.
    pub fn set_base_line_style(&mut self, base_line_style: Option<LineStyle>) {
        self.base_line_style = base_line_style;
    }

    /// Conflation smoothing factor for this series. Overrides the global time scale option.
    /// Controls how aggressively conflation is applied specifically to this series.
    ///
    /// - 1.0 = conflate only when display can't show detail (default, performance-focused)
    /// - 2.0 = conflate at 2x the display threshold (moderate smoothing)
    /// - 4.0 = conflate at 4x the display threshold (strong smoothing)
    /// - 8.0+ = very aggressive smoothing for very small charts
    ///
    /// Higher values result in fewer data points being displayed for this series,
    /// creating smoother but less detailed charts. This allows different series on the same chart to have different
    /// conflation levels.
    ///
    /// Default: `undefined` (uses global time scale option)`.
    pub fn with_conflation_threshold_factor(self, conflation_threshold_factor: impl Into<f64>) -> Self {
        Self {
            conflation_threshold_factor: Some(conflation_threshold_factor.into()),
            ..self
        }
    }

    pub fn conflation_threshold_factor(&self) -> Option<f64> {
        self.conflation_threshold_factor
    }

    /// Set or reset (with `None`) the `conflationThresholdFactor` option.
    pub fn set_conflation_threshold_factor(&mut self, conflation_threshold_factor: Option<f64>) {
        self.conflation_threshold_factor = conflation_threshold_factor;
    }
}
