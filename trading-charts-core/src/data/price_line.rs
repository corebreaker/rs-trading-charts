use {crate::data::options::LineStyle, crate::data::options::LineWidth};
use serde::{Deserialize, Serialize};

/// Represents a price line options.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceLineOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    id:                    Option<String>,
    price:                 f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    color:                 Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    line_width:            Option<LineWidth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    line_style:            Option<LineStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    line_visible:          Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    axis_label_visible:    Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    title:                 Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    axis_label_color:      Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    axis_label_text_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hit_test_tolerance:    Option<f64>,
}

impl PriceLineOptions {
    /// Create options with the required values, the other options keeping their default value.
    pub fn new(price: impl Into<f64>) -> Self {
        Self {
            price:                 price.into(),
            id:                    None,
            color:                 None,
            line_width:            None,
            line_style:            None,
            line_visible:          None,
            axis_label_visible:    None,
            title:                 None,
            axis_label_color:      None,
            axis_label_text_color: None,
            hit_test_tolerance:    None,
        }
    }

    /// The optional ID of this price line.
    pub fn with_id(self, id: impl Into<String>) -> Self {
        Self {
            id: Some(id.into()),
            ..self
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Set or reset (with `None`) the `id` option.
    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }

    /// Price line's value.
    pub fn with_price(self, price: impl Into<f64>) -> Self {
        Self {
            price: price.into(),
            ..self
        }
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn set_price(&mut self, price: impl Into<f64>) {
        self.price = price.into();
    }

    /// Price line's color.
    ///
    /// Default: `""`.
    pub fn with_color(self, color: impl Into<String>) -> Self {
        Self {
            color: Some(color.into()),
            ..self
        }
    }

    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    /// Set or reset (with `None`) the `color` option.
    pub fn set_color(&mut self, color: Option<String>) {
        self.color = color;
    }

    /// Price line's width in pixels.
    ///
    /// Default: `1`.
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

    /// Price line's style.
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

    /// Display line.
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

    /// Display the current price value in on the price scale.
    ///
    /// Default: `true`.
    pub fn with_axis_label_visible(self, axis_label_visible: bool) -> Self {
        Self {
            axis_label_visible: Some(axis_label_visible),
            ..self
        }
    }

    pub fn axis_label_visible(&self) -> Option<bool> {
        self.axis_label_visible
    }

    /// Set or reset (with `None`) the `axisLabelVisible` option.
    pub fn set_axis_label_visible(&mut self, axis_label_visible: Option<bool>) {
        self.axis_label_visible = axis_label_visible;
    }

    /// Price line's on the chart pane.
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

    /// Background color for the axis label. Will default to the price line color if unspecified.
    ///
    /// Default: `""`.
    pub fn with_axis_label_color(self, axis_label_color: impl Into<String>) -> Self {
        Self {
            axis_label_color: Some(axis_label_color.into()),
            ..self
        }
    }

    pub fn axis_label_color(&self) -> Option<&str> {
        self.axis_label_color.as_deref()
    }

    /// Set or reset (with `None`) the `axisLabelColor` option.
    pub fn set_axis_label_color(&mut self, axis_label_color: Option<String>) {
        self.axis_label_color = axis_label_color;
    }

    /// Text color for the axis label.
    ///
    /// Default: `""`.
    pub fn with_axis_label_text_color(self, axis_label_text_color: impl Into<String>) -> Self {
        Self {
            axis_label_text_color: Some(axis_label_text_color.into()),
            ..self
        }
    }

    pub fn axis_label_text_color(&self) -> Option<&str> {
        self.axis_label_text_color.as_deref()
    }

    /// Set or reset (with `None`) the `axisLabelTextColor` option.
    pub fn set_axis_label_text_color(&mut self, axis_label_text_color: Option<String>) {
        self.axis_label_text_color = axis_label_text_color;
    }

    /// Hit-test tolerance in pixels, added on each side of the line (in addition to its `lineWidth`)
    /// when determining whether a pointer event hits this price line. Lower values require more precise pointing;
    /// `0` limits hits to the line itself. Useful on desktop where the default tolerance can cause nearby price lines
    /// to register as the same hit.
    ///
    /// Default: `7`.
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
}
