use {
    crate::data::options::FlagableOptions,
    crate::data::options::HandleScrollOptions,
    crate::data::options::KineticScrollOptions,
    crate::data::options::LocalizationOptions,
    crate::data::options::TimeScaleOptions,
    crate::data::options::crosshair::CrosshairOptions,
    crate::data::options::grid::GridOptions,
    crate::data::options::handle_scale::HandleScaleOptions,
    crate::data::options::layout::LayoutOptions,
    crate::data::options::overlay_price_scale::OverlayPriceScaleOptions,
    crate::data::options::price_scale::PriceScaleOptions,
    crate::data::options::price_scale::PriceScaleSide,
    crate::data::options::tracking_mode::TrackingModeOptions,
};
use serde::{Deserialize, Serialize};

/// Options of a chart.
///
/// Every option is optional: an option which is not set keeps its current value,
/// or the default value of Lightweight Charts when the chart is created.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    width:                          Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    height:                         Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    auto_size:                      Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    layout:                         Option<LayoutOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    left_price_scale:               Option<PriceScaleOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    right_price_scale:              Option<PriceScaleOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default_visible_price_scale_id: Option<PriceScaleSide>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    overlay_price_scales:           Option<OverlayPriceScaleOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    crosshair:                      Option<CrosshairOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    grid:                           Option<GridOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    handle_scroll:                  Option<FlagableOptions<HandleScrollOptions>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    handle_scale:                   Option<FlagableOptions<HandleScaleOptions>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    kinetic_scroll:                 Option<KineticScrollOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tracking_mode:                  Option<TrackingModeOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    add_default_pane:               Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hovered_series_on_top:          Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    localization:                   Option<LocalizationOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    time_scale:                     Option<TimeScaleOptions>,
}

impl ChartOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Width of the chart in pixels
    pub fn with_width(self, width: impl Into<f64>) -> Self {
        Self {
            width: Some(width.into()),
            ..self
        }
    }

    pub fn width(&self) -> Option<f64> {
        self.width
    }

    /// Set or reset (with `None`) the `width` option.
    pub fn set_width(&mut self, width: Option<f64>) {
        self.width = width;
    }

    /// Height of the chart in pixels
    pub fn with_height(self, height: impl Into<f64>) -> Self {
        Self {
            height: Some(height.into()),
            ..self
        }
    }

    pub fn height(&self) -> Option<f64> {
        self.height
    }

    /// Set or reset (with `None`) the `height` option.
    pub fn set_height(&mut self, height: Option<f64>) {
        self.height = height;
    }

    /// Setting this flag to `true` will make the chart watch the chart container's size and automatically resize the
    /// chart to fit its container whenever the size changes.
    ///
    /// This feature requires [`ResizeObserver`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver)
    /// class to be available in the global scope. Note that calling code is responsible for providing a polyfill if
    /// required. If the global scope does not have `ResizeObserver`,
    /// a warning will appear and the flag will be ignored.
    ///
    /// Please pay attention that `autoSize` option and explicit sizes options `width` and `height` don't conflict with
    /// one another. If you specify `autoSize` flag, then `width` and `height` options will be ignored unless
    /// `ResizeObserver` has failed. If it fails then the values will be used as fallback.
    ///
    /// The flag `autoSize` could also be set with and unset with `applyOptions` function.
    /// ```js const chart = LightweightCharts.createChart(document.body, { autoSize: true, }); ```
    pub fn with_auto_size(self, auto_size: bool) -> Self {
        Self {
            auto_size: Some(auto_size),
            ..self
        }
    }

    pub fn auto_size(&self) -> Option<bool> {
        self.auto_size
    }

    /// Set or reset (with `None`) the `autoSize` option.
    pub fn set_auto_size(&mut self, auto_size: Option<bool>) {
        self.auto_size = auto_size;
    }

    /// Layout options
    pub fn with_layout(self, layout: LayoutOptions) -> Self {
        Self {
            layout: Some(layout),
            ..self
        }
    }

    pub fn layout(&self) -> Option<&LayoutOptions> {
        self.layout.as_ref()
    }

    /// Set or reset (with `None`) the `layout` option.
    pub fn set_layout(&mut self, layout: Option<LayoutOptions>) {
        self.layout = layout;
    }

    /// Left price scale options
    pub fn with_left_price_scale(self, left_price_scale: PriceScaleOptions) -> Self {
        Self {
            left_price_scale: Some(left_price_scale),
            ..self
        }
    }

    pub fn left_price_scale(&self) -> Option<&PriceScaleOptions> {
        self.left_price_scale.as_ref()
    }

    /// Set or reset (with `None`) the `leftPriceScale` option.
    pub fn set_left_price_scale(&mut self, left_price_scale: Option<PriceScaleOptions>) {
        self.left_price_scale = left_price_scale;
    }

    /// Right price scale options
    pub fn with_right_price_scale(self, right_price_scale: PriceScaleOptions) -> Self {
        Self {
            right_price_scale: Some(right_price_scale),
            ..self
        }
    }

    pub fn right_price_scale(&self) -> Option<&PriceScaleOptions> {
        self.right_price_scale.as_ref()
    }

    /// Set or reset (with `None`) the `rightPriceScale` option.
    pub fn set_right_price_scale(&mut self, right_price_scale: Option<PriceScaleOptions>) {
        self.right_price_scale = right_price_scale;
    }

    /// The price scale to prefer when the chart needs a default side and both left and right price scales share the
    /// same visibility state (both visible or both hidden). This affects behaviors that depend on the pane's default
    /// side, such as: - horizontal grid lines - overlay series label placement - the price scale used when adding a
    /// series without an explicit `priceScaleId` - crosshair price coordinate conversion and magnet snapping
    ///
    /// Default: `"right"`.
    pub fn with_default_visible_price_scale_id(self, default_visible_price_scale_id: PriceScaleSide) -> Self {
        Self {
            default_visible_price_scale_id: Some(default_visible_price_scale_id),
            ..self
        }
    }

    pub fn default_visible_price_scale_id(&self) -> Option<PriceScaleSide> {
        self.default_visible_price_scale_id
    }

    /// Set or reset (with `None`) the `defaultVisiblePriceScaleId` option.
    pub fn set_default_visible_price_scale_id(&mut self, default_visible_price_scale_id: Option<PriceScaleSide>) {
        self.default_visible_price_scale_id = default_visible_price_scale_id;
    }

    /// Overlay price scale options
    pub fn with_overlay_price_scales(self, overlay_price_scales: OverlayPriceScaleOptions) -> Self {
        Self {
            overlay_price_scales: Some(overlay_price_scales),
            ..self
        }
    }

    pub fn overlay_price_scales(&self) -> Option<&OverlayPriceScaleOptions> {
        self.overlay_price_scales.as_ref()
    }

    /// Set or reset (with `None`) the `overlayPriceScales` option.
    pub fn set_overlay_price_scales(&mut self, overlay_price_scales: Option<OverlayPriceScaleOptions>) {
        self.overlay_price_scales = overlay_price_scales;
    }

    /// The crosshair shows the intersection of the price and time scale values at any point on the chart.
    pub fn with_crosshair(self, crosshair: CrosshairOptions) -> Self {
        Self {
            crosshair: Some(crosshair),
            ..self
        }
    }

    pub fn crosshair(&self) -> Option<&CrosshairOptions> {
        self.crosshair.as_ref()
    }

    /// Set or reset (with `None`) the `crosshair` option.
    pub fn set_crosshair(&mut self, crosshair: Option<CrosshairOptions>) {
        self.crosshair = crosshair;
    }

    /// A grid is represented in the chart background as a vertical and horizontal lines drawn at the levels of visible
    /// marks of price and the time scales.
    pub fn with_grid(self, grid: GridOptions) -> Self {
        Self {
            grid: Some(grid),
            ..self
        }
    }

    pub fn grid(&self) -> Option<&GridOptions> {
        self.grid.as_ref()
    }

    /// Set or reset (with `None`) the `grid` option.
    pub fn set_grid(&mut self, grid: Option<GridOptions>) {
        self.grid = grid;
    }

    /// Scroll options, or a boolean flag that enables/disables scrolling
    pub fn with_handle_scroll(self, handle_scroll: impl Into<FlagableOptions<HandleScrollOptions>>) -> Self {
        Self {
            handle_scroll: Some(handle_scroll.into()),
            ..self
        }
    }

    pub fn handle_scroll(&self) -> Option<&FlagableOptions<HandleScrollOptions>> {
        self.handle_scroll.as_ref()
    }

    /// Set or reset (with `None`) the `handleScroll` option.
    pub fn set_handle_scroll(&mut self, handle_scroll: Option<FlagableOptions<HandleScrollOptions>>) {
        self.handle_scroll = handle_scroll;
    }

    /// Scale options, or a boolean flag that enables/disables scaling
    pub fn with_handle_scale(self, handle_scale: impl Into<FlagableOptions<HandleScaleOptions>>) -> Self {
        Self {
            handle_scale: Some(handle_scale.into()),
            ..self
        }
    }

    pub fn handle_scale(&self) -> Option<&FlagableOptions<HandleScaleOptions>> {
        self.handle_scale.as_ref()
    }

    /// Set or reset (with `None`) the `handleScale` option.
    pub fn set_handle_scale(&mut self, handle_scale: Option<FlagableOptions<HandleScaleOptions>>) {
        self.handle_scale = handle_scale;
    }

    /// Kinetic scroll options
    pub fn with_kinetic_scroll(self, kinetic_scroll: KineticScrollOptions) -> Self {
        Self {
            kinetic_scroll: Some(kinetic_scroll),
            ..self
        }
    }

    pub fn kinetic_scroll(&self) -> Option<&KineticScrollOptions> {
        self.kinetic_scroll.as_ref()
    }

    /// Set or reset (with `None`) the `kineticScroll` option.
    pub fn set_kinetic_scroll(&mut self, kinetic_scroll: Option<KineticScrollOptions>) {
        self.kinetic_scroll = kinetic_scroll;
    }

    pub fn with_tracking_mode(self, tracking_mode: TrackingModeOptions) -> Self {
        Self {
            tracking_mode: Some(tracking_mode),
            ..self
        }
    }

    pub fn tracking_mode(&self) -> Option<&TrackingModeOptions> {
        self.tracking_mode.as_ref()
    }

    /// Set or reset (with `None`) the `trackingMode` option.
    pub fn set_tracking_mode(&mut self, tracking_mode: Option<TrackingModeOptions>) {
        self.tracking_mode = tracking_mode;
    }

    /// Whether to add a default pane to the chart Disable this option when you want to create a chart with no panes and
    /// add them manually
    ///
    /// Default: `true`.
    pub fn with_add_default_pane(self, add_default_pane: bool) -> Self {
        Self {
            add_default_pane: Some(add_default_pane),
            ..self
        }
    }

    pub fn add_default_pane(&self) -> Option<bool> {
        self.add_default_pane
    }

    /// Set or reset (with `None`) the `addDefaultPane` option.
    pub fn set_add_default_pane(&mut self, add_default_pane: Option<bool>) {
        self.add_default_pane = add_default_pane;
    }

    /// Whether to draw the currently hovered series above the other series in the same pane.
    ///
    /// This only affects drawing and hit-testing order while the series is hovered;
    /// it doesn't change the stored series order.
    ///
    /// Default: `true`.
    pub fn with_hovered_series_on_top(self, hovered_series_on_top: bool) -> Self {
        Self {
            hovered_series_on_top: Some(hovered_series_on_top),
            ..self
        }
    }

    pub fn hovered_series_on_top(&self) -> Option<bool> {
        self.hovered_series_on_top
    }

    /// Set or reset (with `None`) the `hoveredSeriesOnTop` option.
    pub fn set_hovered_series_on_top(&mut self, hovered_series_on_top: Option<bool>) {
        self.hovered_series_on_top = hovered_series_on_top;
    }

    /// Localization options.
    pub fn with_localization(self, localization: LocalizationOptions) -> Self {
        Self {
            localization: Some(localization),
            ..self
        }
    }

    pub fn localization(&self) -> Option<&LocalizationOptions> {
        self.localization.as_ref()
    }

    /// Set or reset (with `None`) the `localization` option.
    pub fn set_localization(&mut self, localization: Option<LocalizationOptions>) {
        self.localization = localization;
    }

    /// Extended time scale options with option to override tickMarkFormatter
    pub fn with_time_scale(self, time_scale: TimeScaleOptions) -> Self {
        Self {
            time_scale: Some(time_scale),
            ..self
        }
    }

    pub fn time_scale(&self) -> Option<&TimeScaleOptions> {
        self.time_scale.as_ref()
    }

    /// Set or reset (with `None`) the `timeScale` option.
    pub fn set_time_scale(&mut self, time_scale: Option<TimeScaleOptions>) {
        self.time_scale = time_scale;
    }
}
