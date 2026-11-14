use {
    crate::data::options::LastPriceAnimationMode,
    crate::data::options::LineStyle,
    crate::data::options::LineType,
    crate::data::options::LineWidth,
    crate::data::options::PriceFormatOptions,
    crate::data::options::PriceLineSource,
    crate::series::baselines::BaselineBaseValue,
};
use serde::{Deserialize, Serialize};

/// Represents style options for a baseline series.
///
/// The options common to every series type are included.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaselineSeriesOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base_value:                        Option<BaselineBaseValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    relative_gradient:                 Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    top_fill_color1:                   Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    top_fill_color2:                   Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    top_line_color:                    Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bottom_fill_color1:                Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bottom_fill_color2:                Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bottom_line_color:                 Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    line_width:                        Option<LineWidth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    line_style:                        Option<LineStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    line_type:                         Option<LineType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    line_visible:                      Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    point_markers_visible:             Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    point_markers_radius:              Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    crosshair_marker_visible:          Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    crosshair_marker_radius:           Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    crosshair_marker_border_color:     Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    crosshair_marker_background_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    crosshair_marker_border_width:     Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    last_price_animation:              Option<LastPriceAnimationMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    last_value_visible:                Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    title:                             Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_scale_id:                    Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    visible:                           Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hit_test_tolerance:                Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_line_visible:                Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_line_source:                 Option<PriceLineSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_line_width:                  Option<LineWidth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_line_color:                  Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_line_style:                  Option<LineStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_format:                      Option<PriceFormatOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base_line_visible:                 Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base_line_color:                   Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base_line_width:                   Option<LineWidth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base_line_style:                   Option<LineStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    conflation_threshold_factor:       Option<f64>,
}

impl BaselineSeriesOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Base value of the series.
    ///
    /// Default: `{ type: 'price', price: 0 }`.
    pub fn with_base_value(self, base_value: BaselineBaseValue) -> Self {
        Self {
            base_value: Some(base_value),
            ..self
        }
    }

    pub fn base_value(&self) -> Option<BaselineBaseValue> {
        self.base_value
    }

    /// Set or reset (with `None`) the `baseValue` option.
    pub fn set_base_value(&mut self, base_value: Option<BaselineBaseValue>) {
        self.base_value = base_value;
    }

    /// Gradient is relative to the base value and the currently visible range. If it is false,
    /// the gradient is relative to the top and bottom of the chart.
    ///
    /// Default: `false`.
    pub fn with_relative_gradient(self, relative_gradient: bool) -> Self {
        Self {
            relative_gradient: Some(relative_gradient),
            ..self
        }
    }

    pub fn relative_gradient(&self) -> Option<bool> {
        self.relative_gradient
    }

    /// Set or reset (with `None`) the `relativeGradient` option.
    pub fn set_relative_gradient(&mut self, relative_gradient: Option<bool>) {
        self.relative_gradient = relative_gradient;
    }

    /// The first color of the top area.
    ///
    /// Default: `"rgba(38, 166, 154, 0.28)"`.
    pub fn with_top_fill_color1(self, top_fill_color1: impl Into<String>) -> Self {
        Self {
            top_fill_color1: Some(top_fill_color1.into()),
            ..self
        }
    }

    pub fn top_fill_color1(&self) -> Option<&str> {
        self.top_fill_color1.as_deref()
    }

    /// Set or reset (with `None`) the `topFillColor1` option.
    pub fn set_top_fill_color1(&mut self, top_fill_color1: Option<String>) {
        self.top_fill_color1 = top_fill_color1;
    }

    /// The second color of the top area.
    ///
    /// Default: `"rgba(38, 166, 154, 0.05)"`.
    pub fn with_top_fill_color2(self, top_fill_color2: impl Into<String>) -> Self {
        Self {
            top_fill_color2: Some(top_fill_color2.into()),
            ..self
        }
    }

    pub fn top_fill_color2(&self) -> Option<&str> {
        self.top_fill_color2.as_deref()
    }

    /// Set or reset (with `None`) the `topFillColor2` option.
    pub fn set_top_fill_color2(&mut self, top_fill_color2: Option<String>) {
        self.top_fill_color2 = top_fill_color2;
    }

    /// The line color of the top area.
    ///
    /// Default: `"rgba(38, 166, 154, 1)"`.
    pub fn with_top_line_color(self, top_line_color: impl Into<String>) -> Self {
        Self {
            top_line_color: Some(top_line_color.into()),
            ..self
        }
    }

    pub fn top_line_color(&self) -> Option<&str> {
        self.top_line_color.as_deref()
    }

    /// Set or reset (with `None`) the `topLineColor` option.
    pub fn set_top_line_color(&mut self, top_line_color: Option<String>) {
        self.top_line_color = top_line_color;
    }

    /// The first color of the bottom area.
    ///
    /// Default: `"rgba(239, 83, 80, 0.05)"`.
    pub fn with_bottom_fill_color1(self, bottom_fill_color1: impl Into<String>) -> Self {
        Self {
            bottom_fill_color1: Some(bottom_fill_color1.into()),
            ..self
        }
    }

    pub fn bottom_fill_color1(&self) -> Option<&str> {
        self.bottom_fill_color1.as_deref()
    }

    /// Set or reset (with `None`) the `bottomFillColor1` option.
    pub fn set_bottom_fill_color1(&mut self, bottom_fill_color1: Option<String>) {
        self.bottom_fill_color1 = bottom_fill_color1;
    }

    /// The second color of the bottom area.
    ///
    /// Default: `"rgba(239, 83, 80, 0.28)"`.
    pub fn with_bottom_fill_color2(self, bottom_fill_color2: impl Into<String>) -> Self {
        Self {
            bottom_fill_color2: Some(bottom_fill_color2.into()),
            ..self
        }
    }

    pub fn bottom_fill_color2(&self) -> Option<&str> {
        self.bottom_fill_color2.as_deref()
    }

    /// Set or reset (with `None`) the `bottomFillColor2` option.
    pub fn set_bottom_fill_color2(&mut self, bottom_fill_color2: Option<String>) {
        self.bottom_fill_color2 = bottom_fill_color2;
    }

    /// The line color of the bottom area.
    ///
    /// Default: `"rgba(239, 83, 80, 1)"`.
    pub fn with_bottom_line_color(self, bottom_line_color: impl Into<String>) -> Self {
        Self {
            bottom_line_color: Some(bottom_line_color.into()),
            ..self
        }
    }

    pub fn bottom_line_color(&self) -> Option<&str> {
        self.bottom_line_color.as_deref()
    }

    /// Set or reset (with `None`) the `bottomLineColor` option.
    pub fn set_bottom_line_color(&mut self, bottom_line_color: Option<String>) {
        self.bottom_line_color = bottom_line_color;
    }

    /// Line width.
    ///
    /// Default: `3`.
    pub fn with_line_width(self, line_width: LineWidth) -> Self {
        Self {
            line_width: Some(line_width),
            ..self
        }
    }

    pub fn line_width(&self) -> Option<LineWidth> {
        self.line_width
    }

    /// Set or reset (with `None`) the `lineWidth` option.
    pub fn set_line_width(&mut self, line_width: Option<LineWidth>) {
        self.line_width = line_width;
    }

    /// Line style.
    ///
    /// Default: `LineStyle.Solid`.
    pub fn with_line_style(self, line_style: LineStyle) -> Self {
        Self {
            line_style: Some(line_style),
            ..self
        }
    }

    pub fn line_style(&self) -> Option<LineStyle> {
        self.line_style
    }

    /// Set or reset (with `None`) the `lineStyle` option.
    pub fn set_line_style(&mut self, line_style: Option<LineStyle>) {
        self.line_style = line_style;
    }

    /// Line type.
    ///
    /// Default: `LineType.Simple`.
    pub fn with_line_type(self, line_type: LineType) -> Self {
        Self {
            line_type: Some(line_type),
            ..self
        }
    }

    pub fn line_type(&self) -> Option<LineType> {
        self.line_type
    }

    /// Set or reset (with `None`) the `lineType` option.
    pub fn set_line_type(&mut self, line_type: Option<LineType>) {
        self.line_type = line_type;
    }

    /// Show series line.
    ///
    /// Default: `true`.
    pub fn with_line_visible(self, line_visible: bool) -> Self {
        Self {
            line_visible: Some(line_visible),
            ..self
        }
    }

    pub fn line_visible(&self) -> Option<bool> {
        self.line_visible
    }

    /// Set or reset (with `None`) the `lineVisible` option.
    pub fn set_line_visible(&mut self, line_visible: Option<bool>) {
        self.line_visible = line_visible;
    }

    /// Show circle markers on each point.
    ///
    /// Default: `false`.
    pub fn with_point_markers_visible(self, point_markers_visible: bool) -> Self {
        Self {
            point_markers_visible: Some(point_markers_visible),
            ..self
        }
    }

    pub fn point_markers_visible(&self) -> Option<bool> {
        self.point_markers_visible
    }

    /// Set or reset (with `None`) the `pointMarkersVisible` option.
    pub fn set_point_markers_visible(&mut self, point_markers_visible: Option<bool>) {
        self.point_markers_visible = point_markers_visible;
    }

    /// Circle markers radius in pixels.
    ///
    /// Default: `undefined`.
    pub fn with_point_markers_radius(self, point_markers_radius: impl Into<f64>) -> Self {
        Self {
            point_markers_radius: Some(point_markers_radius.into()),
            ..self
        }
    }

    pub fn point_markers_radius(&self) -> Option<f64> {
        self.point_markers_radius
    }

    /// Set or reset (with `None`) the `pointMarkersRadius` option.
    pub fn set_point_markers_radius(&mut self, point_markers_radius: Option<f64>) {
        self.point_markers_radius = point_markers_radius;
    }

    /// Show the crosshair marker.
    ///
    /// Default: `true`.
    pub fn with_crosshair_marker_visible(self, crosshair_marker_visible: bool) -> Self {
        Self {
            crosshair_marker_visible: Some(crosshair_marker_visible),
            ..self
        }
    }

    pub fn crosshair_marker_visible(&self) -> Option<bool> {
        self.crosshair_marker_visible
    }

    /// Set or reset (with `None`) the `crosshairMarkerVisible` option.
    pub fn set_crosshair_marker_visible(&mut self, crosshair_marker_visible: Option<bool>) {
        self.crosshair_marker_visible = crosshair_marker_visible;
    }

    /// Crosshair marker radius in pixels.
    ///
    /// Default: `4`.
    pub fn with_crosshair_marker_radius(self, crosshair_marker_radius: impl Into<f64>) -> Self {
        Self {
            crosshair_marker_radius: Some(crosshair_marker_radius.into()),
            ..self
        }
    }

    pub fn crosshair_marker_radius(&self) -> Option<f64> {
        self.crosshair_marker_radius
    }

    /// Set or reset (with `None`) the `crosshairMarkerRadius` option.
    pub fn set_crosshair_marker_radius(&mut self, crosshair_marker_radius: Option<f64>) {
        self.crosshair_marker_radius = crosshair_marker_radius;
    }

    /// Crosshair marker border color. An empty string falls back to the color of the series under the crosshair.
    ///
    /// Default: `""`.
    pub fn with_crosshair_marker_border_color(self, crosshair_marker_border_color: impl Into<String>) -> Self {
        Self {
            crosshair_marker_border_color: Some(crosshair_marker_border_color.into()),
            ..self
        }
    }

    pub fn crosshair_marker_border_color(&self) -> Option<&str> {
        self.crosshair_marker_border_color.as_deref()
    }

    /// Set or reset (with `None`) the `crosshairMarkerBorderColor` option.
    pub fn set_crosshair_marker_border_color(&mut self, crosshair_marker_border_color: Option<String>) {
        self.crosshair_marker_border_color = crosshair_marker_border_color;
    }

    /// The crosshair marker background color. An empty string falls back to the color of the series under the
    /// crosshair.
    ///
    /// Default: `""`.
    pub fn with_crosshair_marker_background_color(self, crosshair_marker_background_color: impl Into<String>) -> Self {
        Self {
            crosshair_marker_background_color: Some(crosshair_marker_background_color.into()),
            ..self
        }
    }

    pub fn crosshair_marker_background_color(&self) -> Option<&str> {
        self.crosshair_marker_background_color.as_deref()
    }

    /// Set or reset (with `None`) the `crosshairMarkerBackgroundColor` option.
    pub fn set_crosshair_marker_background_color(&mut self, crosshair_marker_background_color: Option<String>) {
        self.crosshair_marker_background_color = crosshair_marker_background_color;
    }

    /// Crosshair marker border width in pixels.
    ///
    /// Default: `2`.
    pub fn with_crosshair_marker_border_width(self, crosshair_marker_border_width: impl Into<f64>) -> Self {
        Self {
            crosshair_marker_border_width: Some(crosshair_marker_border_width.into()),
            ..self
        }
    }

    pub fn crosshair_marker_border_width(&self) -> Option<f64> {
        self.crosshair_marker_border_width
    }

    /// Set or reset (with `None`) the `crosshairMarkerBorderWidth` option.
    pub fn set_crosshair_marker_border_width(&mut self, crosshair_marker_border_width: Option<f64>) {
        self.crosshair_marker_border_width = crosshair_marker_border_width;
    }

    /// Last price animation mode.
    ///
    /// Default: `LastPriceAnimationMode.Disabled`.
    pub fn with_last_price_animation(self, last_price_animation: LastPriceAnimationMode) -> Self {
        Self {
            last_price_animation: Some(last_price_animation),
            ..self
        }
    }

    pub fn last_price_animation(&self) -> Option<LastPriceAnimationMode> {
        self.last_price_animation
    }

    /// Set or reset (with `None`) the `lastPriceAnimation` option.
    pub fn set_last_price_animation(&mut self, last_price_animation: Option<LastPriceAnimationMode>) {
        self.last_price_animation = last_price_animation;
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
