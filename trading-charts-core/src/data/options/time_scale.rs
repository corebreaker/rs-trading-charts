use {crate::data::format::TickMarkFormatter, crate::data::options::ConflationPriority};
use serde::{Deserialize, Serialize};

/// Extended time scale options for time-based horizontal scale
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeScaleOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    right_offset: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    right_offset_pixels: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bar_spacing: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    min_bar_spacing: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max_bar_spacing: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    fix_left_edge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    fix_right_edge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lock_visible_time_range_on_resize: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    right_bar_stays_on_scroll: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    border_visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    border_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    time_visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    seconds_visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    shift_visible_range_on_new_bar: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    allow_shift_visible_range_on_whitespace_replacement: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ticks_visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tick_mark_max_character_length: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    uniform_distribution: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    minimum_height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    allow_bold_labels: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ignore_whitespace_indices: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    enable_conflation: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    conflation_threshold_factor: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    precompute_conflation_on_init: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    precompute_conflation_priority: Option<ConflationPriority>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tick_mark_formatter: Option<TickMarkFormatter>,
}

impl TimeScaleOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// The margin space in bars from the right side of the chart.
    ///
    /// Default: `0`.
    pub fn with_right_offset(self, right_offset: impl Into<f64>) -> Self {
        Self {
            right_offset: Some(right_offset.into()),
            ..self
        }
    }

    pub fn right_offset(&self) -> Option<f64> {
        self.right_offset
    }

    /// Set or reset (with `None`) the `rightOffset` option.
    pub fn set_right_offset(&mut self, right_offset: Option<f64>) {
        self.right_offset = right_offset;
    }

    /// The margin space in pixels from the right side of the chart. This option has priority over `rightOffset`.
    ///
    /// Default: `undefined`.
    pub fn with_right_offset_pixels(self, right_offset_pixels: impl Into<f64>) -> Self {
        Self {
            right_offset_pixels: Some(right_offset_pixels.into()),
            ..self
        }
    }

    pub fn right_offset_pixels(&self) -> Option<f64> {
        self.right_offset_pixels
    }

    /// Set or reset (with `None`) the `rightOffsetPixels` option.
    pub fn set_right_offset_pixels(&mut self, right_offset_pixels: Option<f64>) {
        self.right_offset_pixels = right_offset_pixels;
    }

    /// The space between bars in pixels.
    ///
    /// Default: `6`.
    pub fn with_bar_spacing(self, bar_spacing: impl Into<f64>) -> Self {
        Self {
            bar_spacing: Some(bar_spacing.into()),
            ..self
        }
    }

    pub fn bar_spacing(&self) -> Option<f64> {
        self.bar_spacing
    }

    /// Set or reset (with `None`) the `barSpacing` option.
    pub fn set_bar_spacing(&mut self, bar_spacing: Option<f64>) {
        self.bar_spacing = bar_spacing;
    }

    /// The minimum space between bars in pixels.
    ///
    /// Default: `0.5`.
    pub fn with_min_bar_spacing(self, min_bar_spacing: impl Into<f64>) -> Self {
        Self {
            min_bar_spacing: Some(min_bar_spacing.into()),
            ..self
        }
    }

    pub fn min_bar_spacing(&self) -> Option<f64> {
        self.min_bar_spacing
    }

    /// Set or reset (with `None`) the `minBarSpacing` option.
    pub fn set_min_bar_spacing(&mut self, min_bar_spacing: Option<f64>) {
        self.min_bar_spacing = min_bar_spacing;
    }

    /// The maximum space between bars in pixels.
    ///
    /// Has no effect if value is set to `0`.
    ///
    /// Default: `0`.
    pub fn with_max_bar_spacing(self, max_bar_spacing: impl Into<f64>) -> Self {
        Self {
            max_bar_spacing: Some(max_bar_spacing.into()),
            ..self
        }
    }

    pub fn max_bar_spacing(&self) -> Option<f64> {
        self.max_bar_spacing
    }

    /// Set or reset (with `None`) the `maxBarSpacing` option.
    pub fn set_max_bar_spacing(&mut self, max_bar_spacing: Option<f64>) {
        self.max_bar_spacing = max_bar_spacing;
    }

    /// Prevent scrolling to the left of the first bar.
    ///
    /// Default: `false`.
    pub fn with_fix_left_edge(self, fix_left_edge: bool) -> Self {
        Self {
            fix_left_edge: Some(fix_left_edge),
            ..self
        }
    }

    pub fn fix_left_edge(&self) -> Option<bool> {
        self.fix_left_edge
    }

    /// Set or reset (with `None`) the `fixLeftEdge` option.
    pub fn set_fix_left_edge(&mut self, fix_left_edge: Option<bool>) {
        self.fix_left_edge = fix_left_edge;
    }

    /// Prevent scrolling to the right of the most recent bar.
    ///
    /// Default: `false`.
    pub fn with_fix_right_edge(self, fix_right_edge: bool) -> Self {
        Self {
            fix_right_edge: Some(fix_right_edge),
            ..self
        }
    }

    pub fn fix_right_edge(&self) -> Option<bool> {
        self.fix_right_edge
    }

    /// Set or reset (with `None`) the `fixRightEdge` option.
    pub fn set_fix_right_edge(&mut self, fix_right_edge: Option<bool>) {
        self.fix_right_edge = fix_right_edge;
    }

    /// Prevent changing the visible time range during chart resizing.
    ///
    /// Default: `false`.
    pub fn with_lock_visible_time_range_on_resize(self, lock_visible_time_range_on_resize: bool) -> Self {
        Self {
            lock_visible_time_range_on_resize: Some(lock_visible_time_range_on_resize),
            ..self
        }
    }

    pub fn lock_visible_time_range_on_resize(&self) -> Option<bool> {
        self.lock_visible_time_range_on_resize
    }

    /// Set or reset (with `None`) the `lockVisibleTimeRangeOnResize` option.
    pub fn set_lock_visible_time_range_on_resize(&mut self, lock_visible_time_range_on_resize: Option<bool>) {
        self.lock_visible_time_range_on_resize = lock_visible_time_range_on_resize;
    }

    /// Prevent the hovered bar from moving when scrolling.
    ///
    /// Default: `false`.
    pub fn with_right_bar_stays_on_scroll(self, right_bar_stays_on_scroll: bool) -> Self {
        Self {
            right_bar_stays_on_scroll: Some(right_bar_stays_on_scroll),
            ..self
        }
    }

    pub fn right_bar_stays_on_scroll(&self) -> Option<bool> {
        self.right_bar_stays_on_scroll
    }

    /// Set or reset (with `None`) the `rightBarStaysOnScroll` option.
    pub fn set_right_bar_stays_on_scroll(&mut self, right_bar_stays_on_scroll: Option<bool>) {
        self.right_bar_stays_on_scroll = right_bar_stays_on_scroll;
    }

    /// Show the time scale border.
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

    /// The time scale border color.
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

    /// Show the time scale.
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

    /// Show the time, not just the date, in the time scale and vertical crosshair label.
    ///
    /// Default: `false`.
    pub fn with_time_visible(self, time_visible: bool) -> Self {
        Self {
            time_visible: Some(time_visible),
            ..self
        }
    }

    pub fn time_visible(&self) -> Option<bool> {
        self.time_visible
    }

    /// Set or reset (with `None`) the `timeVisible` option.
    pub fn set_time_visible(&mut self, time_visible: Option<bool>) {
        self.time_visible = time_visible;
    }

    /// Show seconds in the time scale and vertical crosshair label in `hh:mm:ss` format for intraday data.
    ///
    /// Default: `true`.
    pub fn with_seconds_visible(self, seconds_visible: bool) -> Self {
        Self {
            seconds_visible: Some(seconds_visible),
            ..self
        }
    }

    pub fn seconds_visible(&self) -> Option<bool> {
        self.seconds_visible
    }

    /// Set or reset (with `None`) the `secondsVisible` option.
    pub fn set_seconds_visible(&mut self, seconds_visible: Option<bool>) {
        self.seconds_visible = seconds_visible;
    }

    /// Shift the visible range to the right (into the future) by the number of new bars when new data is added.
    ///
    /// Note that this only applies when the last bar is visible.
    ///
    /// Default: `true`.
    pub fn with_shift_visible_range_on_new_bar(self, shift_visible_range_on_new_bar: bool) -> Self {
        Self {
            shift_visible_range_on_new_bar: Some(shift_visible_range_on_new_bar),
            ..self
        }
    }

    pub fn shift_visible_range_on_new_bar(&self) -> Option<bool> {
        self.shift_visible_range_on_new_bar
    }

    /// Set or reset (with `None`) the `shiftVisibleRangeOnNewBar` option.
    pub fn set_shift_visible_range_on_new_bar(&mut self, shift_visible_range_on_new_bar: Option<bool>) {
        self.shift_visible_range_on_new_bar = shift_visible_range_on_new_bar;
    }

    /// Allow the visible range to be shifted to the right when a new bar is added which is replacing an existing
    /// whitespace time point on the chart.
    ///
    /// Note that this only applies when the last bar is visible & `shiftVisibleRangeOnNewBar` is enabled.
    ///
    /// Default: `false`.
    pub fn with_allow_shift_visible_range_on_whitespace_replacement(
        self,
        allow_shift_visible_range_on_whitespace_replacement: bool,
    ) -> Self {
        Self {
            allow_shift_visible_range_on_whitespace_replacement: Some(
                allow_shift_visible_range_on_whitespace_replacement,
            ),
            ..self
        }
    }

    pub fn allow_shift_visible_range_on_whitespace_replacement(&self) -> Option<bool> {
        self.allow_shift_visible_range_on_whitespace_replacement
    }

    /// Set or reset (with `None`) the `allowShiftVisibleRangeOnWhitespaceReplacement` option.
    pub fn set_allow_shift_visible_range_on_whitespace_replacement(
        &mut self,
        allow_shift_visible_range_on_whitespace_replacement: Option<bool>,
    ) {
        self.allow_shift_visible_range_on_whitespace_replacement = allow_shift_visible_range_on_whitespace_replacement;
    }

    /// Draw small vertical line on time axis labels.
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

    /// Maximum tick mark label length. Used to override the default 8 character maximum length.
    ///
    /// Default: `undefined`.
    pub fn with_tick_mark_max_character_length(self, tick_mark_max_character_length: impl Into<f64>) -> Self {
        Self {
            tick_mark_max_character_length: Some(tick_mark_max_character_length.into()),
            ..self
        }
    }

    pub fn tick_mark_max_character_length(&self) -> Option<f64> {
        self.tick_mark_max_character_length
    }

    /// Set or reset (with `None`) the `tickMarkMaxCharacterLength` option.
    pub fn set_tick_mark_max_character_length(&mut self, tick_mark_max_character_length: Option<f64>) {
        self.tick_mark_max_character_length = tick_mark_max_character_length;
    }

    /// Changes horizontal scale marks generation. With this flag equal to `true`,
    /// marks of the same weight are either all drawn or none are drawn at all.
    pub fn with_uniform_distribution(self, uniform_distribution: bool) -> Self {
        Self {
            uniform_distribution: Some(uniform_distribution),
            ..self
        }
    }

    pub fn uniform_distribution(&self) -> Option<bool> {
        self.uniform_distribution
    }

    /// Set or reset (with `None`) the `uniformDistribution` option.
    pub fn set_uniform_distribution(&mut self, uniform_distribution: Option<bool>) {
        self.uniform_distribution = uniform_distribution;
    }

    /// Define a minimum height for the time scale. Note: This value will be exceeded if the time scale needs more space
    /// to display it's contents.
    ///
    /// Setting a minimum height could be useful for ensuring that multiple charts positioned in a horizontal stack each
    /// have an identical time scale height, or for plugins which require a bit more space within the time scale pane.
    ///
    /// Default: `0`.
    pub fn with_minimum_height(self, minimum_height: impl Into<f64>) -> Self {
        Self {
            minimum_height: Some(minimum_height.into()),
            ..self
        }
    }

    pub fn minimum_height(&self) -> Option<f64> {
        self.minimum_height
    }

    /// Set or reset (with `None`) the `minimumHeight` option.
    pub fn set_minimum_height(&mut self, minimum_height: Option<f64>) {
        self.minimum_height = minimum_height;
    }

    /// Allow major time scale labels to be rendered in a bolder font weight.
    ///
    /// Default: `true`.
    pub fn with_allow_bold_labels(self, allow_bold_labels: bool) -> Self {
        Self {
            allow_bold_labels: Some(allow_bold_labels),
            ..self
        }
    }

    pub fn allow_bold_labels(&self) -> Option<bool> {
        self.allow_bold_labels
    }

    /// Set or reset (with `None`) the `allowBoldLabels` option.
    pub fn set_allow_bold_labels(&mut self, allow_bold_labels: Option<bool>) {
        self.allow_bold_labels = allow_bold_labels;
    }

    /// Ignore time scale points containing only whitespace (for all series) when drawing grid lines, tick marks,
    /// and snapping the crosshair to time scale points.
    ///
    /// For the yield curve chart type it defaults to `true`.
    ///
    /// Default: `false`.
    pub fn with_ignore_whitespace_indices(self, ignore_whitespace_indices: bool) -> Self {
        Self {
            ignore_whitespace_indices: Some(ignore_whitespace_indices),
            ..self
        }
    }

    pub fn ignore_whitespace_indices(&self) -> Option<bool> {
        self.ignore_whitespace_indices
    }

    /// Set or reset (with `None`) the `ignoreWhitespaceIndices` option.
    pub fn set_ignore_whitespace_indices(&mut self, ignore_whitespace_indices: Option<bool>) {
        self.ignore_whitespace_indices = ignore_whitespace_indices;
    }

    /// Enable data conflation for performance optimization when bar spacing is very small. When enabled,
    /// multiple data points are automatically combined into single points when they would be rendered in less than 0.5
    /// pixels of screen space. This significantly improves rendering performance for large datasets when zoomed out.
    ///
    /// Default: `false`.
    pub fn with_enable_conflation(self, enable_conflation: bool) -> Self {
        Self {
            enable_conflation: Some(enable_conflation),
            ..self
        }
    }

    pub fn enable_conflation(&self) -> Option<bool> {
        self.enable_conflation
    }

    /// Set or reset (with `None`) the `enableConflation` option.
    pub fn set_enable_conflation(&mut self, enable_conflation: Option<bool>) {
        self.enable_conflation = enable_conflation;
    }

    /// Smoothing factor for conflation thresholds. Controls how aggressively conflation is applied.
    /// This can be used to create smoother-looking charts, especially useful for sparklines and small charts.
    ///
    /// - 1.0 = conflate only when display can't show detail (default, performance-focused)
    /// - 2.0 = conflate at 2x the display threshold (moderate smoothing)
    /// - 4.0 = conflate at 4x the display threshold (strong smoothing)
    /// - 8.0+ = very aggressive smoothing for very small charts
    ///
    /// Higher values result in fewer data points being displayed, creating smoother but less detailed charts.
    /// This is particularly useful for sparklines and small charts where smooth appearance is prioritized over showing
    /// every data point.
    ///
    /// Note: Should be used with continuous series types (line, area, baseline) for best visual results.
    /// Candlestick and bar series may look less natural with high smoothing factors.
    ///
    /// Default: `1.0`.
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

    /// Precompute conflation chunks for common levels right after data load. When enabled,
    /// the system will precompute conflation data in the background,
    /// which improves performance when zooming out but increases initial load time and memory usage.
    ///
    /// Performance impact: - Initial load: +100-500ms depending on dataset size - Memory usage:
    /// +20-50% of original dataset size - Zoom performance: Significant improvement (10-100x faster)
    ///
    /// Recommended for: Large datasets (\>10K points) on machines with sufficient memory
    ///
    /// Default: `false`.
    pub fn with_precompute_conflation_on_init(self, precompute_conflation_on_init: bool) -> Self {
        Self {
            precompute_conflation_on_init: Some(precompute_conflation_on_init),
            ..self
        }
    }

    pub fn precompute_conflation_on_init(&self) -> Option<bool> {
        self.precompute_conflation_on_init
    }

    /// Set or reset (with `None`) the `precomputeConflationOnInit` option.
    pub fn set_precompute_conflation_on_init(&mut self, precompute_conflation_on_init: Option<bool>) {
        self.precompute_conflation_on_init = precompute_conflation_on_init;
    }

    /// Priority used for background precompute tasks when the Prioritized Task Scheduling API is available.
    ///
    /// Options: - 'background': Lowest priority, tasks run only when the browser is idle - 'user-visible':
    /// Medium priority, tasks run when they might affect visible content - 'user-blocking': Highest priority,
    /// tasks run immediately and may block user interaction
    ///
    /// Recommendation: Use 'background' for most cases to avoid impacting user experience.
    /// Only use higher priorities if conflation is critical for your application's functionality.
    ///
    /// Default: `"background"`.
    pub fn with_precompute_conflation_priority(self, precompute_conflation_priority: ConflationPriority) -> Self {
        Self {
            precompute_conflation_priority: Some(precompute_conflation_priority),
            ..self
        }
    }

    pub fn precompute_conflation_priority(&self) -> Option<ConflationPriority> {
        self.precompute_conflation_priority
    }

    /// Set or reset (with `None`) the `precomputeConflationPriority` option.
    pub fn set_precompute_conflation_priority(&mut self, precompute_conflation_priority: Option<ConflationPriority>) {
        self.precompute_conflation_priority = precompute_conflation_priority;
    }

    /// Tick marks formatter can be used to customize tick marks labels on the time axis.
    ///
    /// Default: `undefined`.
    pub fn with_tick_mark_formatter(self, tick_mark_formatter: TickMarkFormatter) -> Self {
        Self {
            tick_mark_formatter: Some(tick_mark_formatter),
            ..self
        }
    }

    pub fn tick_mark_formatter(&self) -> Option<&TickMarkFormatter> {
        self.tick_mark_formatter.as_ref()
    }

    /// Set or reset (with `None`) the `tickMarkFormatter` option.
    pub fn set_tick_mark_formatter(&mut self, tick_mark_formatter: Option<TickMarkFormatter>) {
        self.tick_mark_formatter = tick_mark_formatter;
    }
}
