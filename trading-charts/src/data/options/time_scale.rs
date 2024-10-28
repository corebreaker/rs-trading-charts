use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TimeScaleOptions {
    #[serde(rename = "rightOffset", default = "defaults::right_offset")]
    pub right_offset: f64,

    #[serde(rename = "barSpacing", default = "defaults::bar_spacing")]
    pub bar_spacing: f64,

    #[serde(rename = "minBarWidth", default = "defaults::min_bar_width")]
    pub min_bar_width: f64,

    #[serde(rename = "fixLeftEdge", default = "defaults::fix_left_edge")]
    pub fix_left_edge: bool,

    #[serde(rename = "fixRightEdge", default = "defaults::fix_right_edge")]
    pub fix_right_edge: bool,

    #[serde(
        rename = "lockVisibleTimeRangeOnResize",
        default = "defaults::lock_visible_time_range_on_resize"
    )]
    pub lock_visible_time_range_on_resize: bool,

    #[serde(rename = "rightBarStaysOnScroll", default = "defaults::right_bar_stays_on_scroll")]
    pub right_bar_stays_on_scroll: bool,

    #[serde(rename = "borderVisible", default = "defaults::border_visible")]
    pub border_visible: bool,

    #[serde(rename = "borderColor", default = "defaults::border_color")]
    pub border_color: String,

    #[serde(rename = "visible", default = "defaults::visible")]
    pub visible: bool,

    #[serde(rename = "timeVisible", default = "defaults::time_visible")]
    pub time_visible: bool,

    #[serde(rename = "secondsVisible", default = "defaults::seconds_visible")]
    pub seconds_visible: bool,

    #[serde(
        rename = "shiftVisibleRangeOnNewBar",
        default = "defaults::shift_visible_range_on_new_bar"
    )]
    pub shift_visible_range_on_new_bar: bool,

    #[serde(
        rename = "allowShiftVisibleRangeOnWhitespaceReplacement",
        default = "defaults::allow_shift_visible_etc"
    )]
    pub allow_shift_visible_range_on_whitespace_replacement: bool,

    #[serde(rename = "ticksVisible", default = "defaults::ticks_visible")]
    pub ticks_visible: bool,

    #[serde(
        rename = "tickMarkMaxCharacterLength",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub tick_mark_max_character_length: Option<usize>,

    #[serde(rename = "uniformDistribution", default = "defaults::uniform_distribution")]
    pub uniform_distribution: bool,

    #[serde(rename = "minimumHeight", default = "defaults::minimum_height")]
    pub minimum_height: f64,

    #[serde(rename = "allowBoldLabels", default = "defaults::allow_bold_labels")]
    pub allow_bold_labels: bool,
}

impl TimeScaleOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_right_offset(self, right_offset: f64) -> Self {
        Self {
            right_offset,
            ..self
        }
    }

    pub fn with_bar_spacing(self, bar_spacing: f64) -> Self {
        Self {
            bar_spacing,
            ..self
        }
    }

    pub fn with_min_bar_width(self, min_bar_width: f64) -> Self {
        Self {
            min_bar_width,
            ..self
        }
    }

    pub fn with_fix_left_edge(self, fix_left_edge: bool) -> Self {
        Self {
            fix_left_edge,
            ..self
        }
    }

    pub fn with_fix_right_edge(self, fix_right_edge: bool) -> Self {
        Self {
            fix_right_edge,
            ..self
        }
    }

    pub fn with_lock_visible_time_range_on_resize(self, lock_visible_time_range_on_resize: bool) -> Self {
        Self {
            lock_visible_time_range_on_resize,
            ..self
        }
    }

    pub fn with_right_bar_stays_on_scroll(self, right_bar_stays_on_scroll: bool) -> Self {
        Self {
            right_bar_stays_on_scroll,
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

    pub fn with_visible(self, visible: bool) -> Self {
        Self {
            visible,
            ..self
        }
    }

    pub fn with_time_visible(self, time_visible: bool) -> Self {
        Self {
            time_visible,
            ..self
        }
    }

    pub fn with_seconds_visible(self, seconds_visible: bool) -> Self {
        Self {
            seconds_visible,
            ..self
        }
    }

    pub fn with_shift_visible_range_on_new_bar(self, shift_visible_range_on_new_bar: bool) -> Self {
        Self {
            shift_visible_range_on_new_bar,
            ..self
        }
    }

    pub fn with_allow_shift_visible_range_on_whitespace_replacement(
        self,
        allow_shift_visible_range_on_whitespace_replacement: bool,
    ) -> Self {
        Self {
            allow_shift_visible_range_on_whitespace_replacement,
            ..self
        }
    }

    pub fn with_ticks_visible(self, ticks_visible: bool) -> Self {
        Self {
            ticks_visible,
            ..self
        }
    }

    pub fn with_tick_mark_max_character_length(self, tick_mark_max_character_length: Option<usize>) -> Self {
        Self {
            tick_mark_max_character_length,
            ..self
        }
    }

    pub fn with_uniform_distribution(self, uniform_distribution: bool) -> Self {
        Self {
            uniform_distribution,
            ..self
        }
    }

    pub fn with_minimum_height(self, minimum_height: f64) -> Self {
        Self {
            minimum_height,
            ..self
        }
    }

    pub fn with_allow_bold_labels(self, allow_bold_labels: bool) -> Self {
        Self {
            allow_bold_labels,
            ..self
        }
    }

    pub fn right_offset(&self) -> f64 {
        self.right_offset
    }

    pub fn set_right_offset(&mut self, right_offset: f64) {
        self.right_offset = right_offset;
    }

    pub fn bar_spacing(&self) -> f64 {
        self.bar_spacing
    }

    pub fn set_bar_spacing(&mut self, bar_spacing: f64) {
        self.bar_spacing = bar_spacing;
    }

    pub fn min_bar_width(&self) -> f64 {
        self.min_bar_width
    }

    pub fn set_min_bar_width(&mut self, min_bar_width: f64) {
        self.min_bar_width = min_bar_width;
    }

    pub fn fix_left_edge(&self) -> bool {
        self.fix_left_edge
    }

    pub fn set_fix_left_edge(&mut self, fix_left_edge: bool) {
        self.fix_left_edge = fix_left_edge;
    }

    pub fn fix_right_edge(&self) -> bool {
        self.fix_right_edge
    }

    pub fn set_fix_right_edge(&mut self, fix_right_edge: bool) {
        self.fix_right_edge = fix_right_edge;
    }

    pub fn lock_visible_time_range_on_resize(&self) -> bool {
        self.lock_visible_time_range_on_resize
    }

    pub fn set_lock_visible_time_range_on_resize(&mut self, lock_visible_time_range_on_resize: bool) {
        self.lock_visible_time_range_on_resize = lock_visible_time_range_on_resize;
    }

    pub fn right_bar_stays_on_scroll(&self) -> bool {
        self.right_bar_stays_on_scroll
    }

    pub fn set_right_bar_stays_on_scroll(&mut self, right_bar_stays_on_scroll: bool) {
        self.right_bar_stays_on_scroll = right_bar_stays_on_scroll;
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

    pub fn set_border_color(&mut self, border_color: String) {
        self.border_color = border_color;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn time_visible(&self) -> bool {
        self.time_visible
    }

    pub fn set_time_visible(&mut self, time_visible: bool) {
        self.time_visible = time_visible;
    }

    pub fn seconds_visible(&self) -> bool {
        self.seconds_visible
    }

    pub fn set_seconds_visible(&mut self, seconds_visible: bool) {
        self.seconds_visible = seconds_visible;
    }

    pub fn shift_visible_range_on_new_bar(&self) -> bool {
        self.shift_visible_range_on_new_bar
    }

    pub fn set_shift_visible_range_on_new_bar(&mut self, shift_visible_range_on_new_bar: bool) {
        self.shift_visible_range_on_new_bar = shift_visible_range_on_new_bar;
    }

    pub fn allow_shift_visible_range_on_whitespace_replacement(&self) -> bool {
        self.allow_shift_visible_range_on_whitespace_replacement
    }

    pub fn set_allow_shift_visible_range_on_whitespace_replacement(
        &mut self,
        allow_shift_visible_range_on_whitespace_replacement: bool,
    ) {
        self.allow_shift_visible_range_on_whitespace_replacement = allow_shift_visible_range_on_whitespace_replacement;
    }

    pub fn ticks_visible(&self) -> bool {
        self.ticks_visible
    }

    pub fn set_ticks_visible(&mut self, ticks_visible: bool) {
        self.ticks_visible = ticks_visible;
    }

    pub fn tick_mark_max_character_length(&self) -> Option<usize> {
        self.tick_mark_max_character_length
    }

    pub fn set_tick_mark_max_character_length_value(&mut self, tick_mark_max_character_length: Option<usize>) {
        self.tick_mark_max_character_length = tick_mark_max_character_length;
    }

    pub fn set_tick_mark_max_character_length(&mut self, tick_mark_max_character_length: usize) {
        self.tick_mark_max_character_length
            .replace(tick_mark_max_character_length);
    }

    pub fn reset_tick_mark_max_character_length(&mut self) {
        self.tick_mark_max_character_length = None;
    }

    pub fn uniform_distribution(&self) -> bool {
        self.uniform_distribution
    }

    pub fn set_uniform_distribution(&mut self, uniform_distribution: bool) {
        self.uniform_distribution = uniform_distribution;
    }

    pub fn minimum_height(&self) -> f64 {
        self.minimum_height
    }

    pub fn set_minimum_height(&mut self, minimum_height: f64) {
        self.minimum_height = minimum_height;
    }

    pub fn allow_bold_labels(&self) -> bool {
        self.allow_bold_labels
    }

    pub fn set_allow_bold_labels(&mut self, allow_bold_labels: bool) {
        self.allow_bold_labels = allow_bold_labels;
    }
}

impl Default for TimeScaleOptions {
    fn default() -> Self {
        Self {
            right_offset: defaults::right_offset(),
            bar_spacing: defaults::bar_spacing(),
            min_bar_width: defaults::min_bar_width(),
            fix_left_edge: defaults::fix_left_edge(),
            fix_right_edge: defaults::fix_right_edge(),
            lock_visible_time_range_on_resize: defaults::lock_visible_time_range_on_resize(),
            right_bar_stays_on_scroll: defaults::right_bar_stays_on_scroll(),
            border_visible: defaults::border_visible(),
            border_color: defaults::border_color(),
            visible: defaults::visible(),
            time_visible: defaults::time_visible(),
            seconds_visible: defaults::seconds_visible(),
            shift_visible_range_on_new_bar: defaults::shift_visible_range_on_new_bar(),
            allow_shift_visible_range_on_whitespace_replacement: defaults::allow_shift_visible_etc(),
            ticks_visible: defaults::ticks_visible(),
            tick_mark_max_character_length: None,
            uniform_distribution: defaults::uniform_distribution(),
            minimum_height: defaults::minimum_height(),
            allow_bold_labels: defaults::allow_bold_labels(),
        }
    }
}

mod defaults {
    pub(super) fn right_offset() -> f64 {
        0.
    }

    pub(super) fn bar_spacing() -> f64 {
        6.
    }

    pub(super) fn min_bar_width() -> f64 {
        0.5
    }

    pub(super) fn fix_left_edge() -> bool {
        false
    }

    pub(super) fn fix_right_edge() -> bool {
        false
    }

    pub(super) fn lock_visible_time_range_on_resize() -> bool {
        false
    }

    pub(super) fn right_bar_stays_on_scroll() -> bool {
        false
    }

    pub(super) fn border_visible() -> bool {
        true
    }

    pub(super) fn border_color() -> String {
        String::from("#2B2B43")
    }

    pub(super) fn visible() -> bool {
        true
    }

    pub(super) fn time_visible() -> bool {
        false
    }

    pub(super) fn seconds_visible() -> bool {
        true
    }

    pub(super) fn shift_visible_range_on_new_bar() -> bool {
        true
    }

    pub(super) fn allow_shift_visible_etc() -> bool {
        false
    }

    pub(super) fn ticks_visible() -> bool {
        false
    }

    pub(super) fn uniform_distribution() -> bool {
        false
    }

    pub(super) fn minimum_height() -> f64 {
        0.
    }

    pub(super) fn allow_bold_labels() -> bool {
        true
    }
}
