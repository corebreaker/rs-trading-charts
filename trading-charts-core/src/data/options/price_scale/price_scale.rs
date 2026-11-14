use {crate::data::options::price_scale::PriceScaleMargins, crate::data::options::price_scale::PriceScaleMode};
use serde::{Deserialize, Serialize};

/// Structure that describes price scale options
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceScaleOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    auto_scale:                     Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    mode:                           Option<PriceScaleMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    invert_scale:                   Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    align_labels:                   Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    scale_margins:                  Option<PriceScaleMargins>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    border_visible:                 Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    border_color:                   Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    text_color:                     Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entire_text_only:               Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    visible:                        Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ticks_visible:                  Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    minimum_width:                  Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ensure_edge_tick_marks_visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tick_mark_density:              Option<f64>,
}

impl PriceScaleOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Autoscaling is a feature that automatically adjusts a price scale to fit the visible range of data.
    /// Note that overlay price scales are always auto-scaled.
    ///
    /// Default: `true`.
    pub fn with_auto_scale(self, auto_scale: bool) -> Self {
        Self {
            auto_scale: Some(auto_scale),
            ..self
        }
    }

    pub fn auto_scale(&self) -> Option<bool> {
        self.auto_scale
    }

    /// Set or reset (with `None`) the `autoScale` option.
    pub fn set_auto_scale(&mut self, auto_scale: Option<bool>) {
        self.auto_scale = auto_scale;
    }

    /// Price scale mode.
    ///
    /// Default: `PriceScaleMode.Normal`.
    pub fn with_mode(self, mode: PriceScaleMode) -> Self {
        Self {
            mode: Some(mode),
            ..self
        }
    }

    pub fn mode(&self) -> Option<PriceScaleMode> {
        self.mode
    }

    /// Set or reset (with `None`) the `mode` option.
    pub fn set_mode(&mut self, mode: Option<PriceScaleMode>) {
        self.mode = mode;
    }

    /// Invert the price scale, so that a upwards trend is shown as a downwards trend and vice versa.
    /// Affects both the price scale and the data on the chart.
    ///
    /// Default: `false`.
    pub fn with_invert_scale(self, invert_scale: bool) -> Self {
        Self {
            invert_scale: Some(invert_scale),
            ..self
        }
    }

    pub fn invert_scale(&self) -> Option<bool> {
        self.invert_scale
    }

    /// Set or reset (with `None`) the `invertScale` option.
    pub fn set_invert_scale(&mut self, invert_scale: Option<bool>) {
        self.invert_scale = invert_scale;
    }

    /// Align price scale labels to prevent them from overlapping.
    ///
    /// Default: `true`.
    pub fn with_align_labels(self, align_labels: bool) -> Self {
        Self {
            align_labels: Some(align_labels),
            ..self
        }
    }

    pub fn align_labels(&self) -> Option<bool> {
        self.align_labels
    }

    /// Set or reset (with `None`) the `alignLabels` option.
    pub fn set_align_labels(&mut self, align_labels: Option<bool>) {
        self.align_labels = align_labels;
    }

    /// Price scale margins.
    ///
    /// Default: `{ bottom: 0.1, top: 0.2 }`.
    pub fn with_scale_margins(self, scale_margins: PriceScaleMargins) -> Self {
        Self {
            scale_margins: Some(scale_margins),
            ..self
        }
    }

    pub fn scale_margins(&self) -> Option<&PriceScaleMargins> {
        self.scale_margins.as_ref()
    }

    /// Set or reset (with `None`) the `scaleMargins` option.
    pub fn set_scale_margins(&mut self, scale_margins: Option<PriceScaleMargins>) {
        self.scale_margins = scale_margins;
    }

    /// Set true to draw a border between the price scale and the chart area.
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

    /// Price scale border color.
    ///
    /// Default: `"#2B2B43"`.
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

    /// Price scale text color. If not provided `textColor` is used.
    ///
    /// Default: `undefined`.
    pub fn with_text_color(self, text_color: impl Into<String>) -> Self {
        Self {
            text_color: Some(text_color.into()),
            ..self
        }
    }

    pub fn text_color(&self) -> Option<&str> {
        self.text_color.as_deref()
    }

    /// Set or reset (with `None`) the `textColor` option.
    pub fn set_text_color(&mut self, text_color: Option<String>) {
        self.text_color = text_color;
    }

    /// Show top and bottom corner labels only if entire text is visible.
    ///
    /// Default: `false`.
    pub fn with_entire_text_only(self, entire_text_only: bool) -> Self {
        Self {
            entire_text_only: Some(entire_text_only),
            ..self
        }
    }

    pub fn entire_text_only(&self) -> Option<bool> {
        self.entire_text_only
    }

    /// Set or reset (with `None`) the `entireTextOnly` option.
    pub fn set_entire_text_only(&mut self, entire_text_only: Option<bool>) {
        self.entire_text_only = entire_text_only;
    }

    /// Indicates if this price scale visible. Ignored by overlay price scales.
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

    /// Draw small horizontal line on price axis labels.
    ///
    /// Default: `false`.
    pub fn with_ticks_visible(self, ticks_visible: bool) -> Self {
        Self {
            ticks_visible: Some(ticks_visible),
            ..self
        }
    }

    pub fn ticks_visible(&self) -> Option<bool> {
        self.ticks_visible
    }

    /// Set or reset (with `None`) the `ticksVisible` option.
    pub fn set_ticks_visible(&mut self, ticks_visible: Option<bool>) {
        self.ticks_visible = ticks_visible;
    }

    /// Define a minimum width for the price scale. Note: This value will be exceeded if the price scale needs more
    /// space to display it's contents.
    ///
    /// Setting a minimum width could be useful for ensuring that multiple charts positioned in a vertical stack each
    /// have an identical price scale width, or for plugins which require a bit more space within the price scale pane.
    ///
    /// Default: `0`.
    pub fn with_minimum_width(self, minimum_width: impl Into<f64>) -> Self {
        Self {
            minimum_width: Some(minimum_width.into()),
            ..self
        }
    }

    pub fn minimum_width(&self) -> Option<f64> {
        self.minimum_width
    }

    /// Set or reset (with `None`) the `minimumWidth` option.
    pub fn set_minimum_width(&mut self, minimum_width: Option<f64>) {
        self.minimum_width = minimum_width;
    }

    /// Ensures that tick marks are always visible at the very top and bottom of the price scale,
    /// regardless of the data range. When enabled, a tick mark will be drawn at both edges of the scale,
    /// providing clear boundary indicators.
    ///
    /// Default: `false`.
    pub fn with_ensure_edge_tick_marks_visible(self, ensure_edge_tick_marks_visible: bool) -> Self {
        Self {
            ensure_edge_tick_marks_visible: Some(ensure_edge_tick_marks_visible),
            ..self
        }
    }

    pub fn ensure_edge_tick_marks_visible(&self) -> Option<bool> {
        self.ensure_edge_tick_marks_visible
    }

    /// Set or reset (with `None`) the `ensureEdgeTickMarksVisible` option.
    pub fn set_ensure_edge_tick_marks_visible(&mut self, ensure_edge_tick_marks_visible: Option<bool>) {
        self.ensure_edge_tick_marks_visible = ensure_edge_tick_marks_visible;
    }

    /// Tick mark label density on the price scale. A higher value results in more spacing between tick marks,
    /// and thus fewer tick marks. A lower value results in less spacing and more tick marks.
    ///
    /// Default: `2.5`.
    pub fn with_tick_mark_density(self, tick_mark_density: impl Into<f64>) -> Self {
        Self {
            tick_mark_density: Some(tick_mark_density.into()),
            ..self
        }
    }

    pub fn tick_mark_density(&self) -> Option<f64> {
        self.tick_mark_density
    }

    /// Set or reset (with `None`) the `tickMarkDensity` option.
    pub fn set_tick_mark_density(&mut self, tick_mark_density: Option<f64>) {
        self.tick_mark_density = tick_mark_density;
    }
}
