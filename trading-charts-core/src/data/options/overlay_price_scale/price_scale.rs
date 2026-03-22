use super::{OverlayPriceScaleMode, OverlayPriceScaleMargins};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct OverlayPriceScaleOptions {
    #[serde(default)]
    mode: OverlayPriceScaleMode,

    #[serde(rename = "invertScale", default = "defaults::invert_scale")]
    invert_scale: bool,

    #[serde(rename = "alignLabels", default = "defaults::align_labels")]
    align_labels: bool,

    #[serde(rename = "scaleMargins", default)]
    scale_margins: OverlayPriceScaleMargins,

    #[serde(rename = "borderVisible", default = "defaults::border_visible")]
    border_visible: bool,

    #[serde(rename = "borderColor", default = "defaults::border_color")]
    border_color: String,

    #[serde(
        rename = "textColor",
        default = "defaults::text_color",
        skip_serializing_if = "String::is_empty"
    )]
    text_color: String,

    #[serde(rename = "entireTextOnly", default = "defaults::entire_text_only")]
    entire_text_only: bool,

    #[serde(rename = "ticksVisible", default = "defaults::ticks_visible")]
    ticks_visible: bool,

    #[serde(rename = "minimumWidth", default = "defaults::minimum_width")]
    minimum_width: f64,
}

impl OverlayPriceScaleOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mode(self, mode: OverlayPriceScaleMode) -> Self {
        Self {
            mode,
            ..self
        }
    }

    pub fn with_invert_scale(self, invert_scale: bool) -> Self {
        Self {
            invert_scale,
            ..self
        }
    }

    pub fn with_align_labels(self, align_labels: bool) -> Self {
        Self {
            align_labels,
            ..self
        }
    }

    pub fn with_scale_margins(self, scale_margins: OverlayPriceScaleMargins) -> Self {
        Self {
            scale_margins,
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

    pub fn with_text_color(self, text_color: String) -> Self {
        Self {
            text_color,
            ..self
        }
    }

    pub fn with_entire_text_only(self, entire_text_only: bool) -> Self {
        Self {
            entire_text_only,
            ..self
        }
    }

    pub fn with_ticks_visible(self, ticks_visible: bool) -> Self {
        Self {
            ticks_visible,
            ..self
        }
    }

    pub fn with_minimum_size(self, minimum_size: f64) -> Self {
        Self {
            minimum_width: minimum_size,
            ..self
        }
    }

    pub fn mode(&self) -> &OverlayPriceScaleMode {
        &self.mode
    }

    pub fn mode_mut(&mut self) -> &mut OverlayPriceScaleMode {
        &mut self.mode
    }

    pub fn set_mode(&mut self, mode: OverlayPriceScaleMode) {
        self.mode = mode;
    }

    pub fn invert_scale(&self) -> bool {
        self.invert_scale
    }

    pub fn set_invert_scale(&mut self, invert_scale: bool) {
        self.invert_scale = invert_scale;
    }

    pub fn align_labels(&self) -> bool {
        self.align_labels
    }

    pub fn set_align_labels(&mut self, align_labels: bool) {
        self.align_labels = align_labels;
    }

    pub fn scale_margins(&self) -> &OverlayPriceScaleMargins {
        &self.scale_margins
    }

    pub fn scale_margins_mut(&mut self) -> &mut OverlayPriceScaleMargins {
        &mut self.scale_margins
    }

    pub fn set_scale_margins(&mut self, scale_margins: OverlayPriceScaleMargins) {
        self.scale_margins = scale_margins;
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

    pub fn text_color(&self) -> &str {
        &self.text_color
    }

    pub fn text_color_mut(&mut self) -> &mut String {
        &mut self.text_color
    }

    pub fn set_text_color(&mut self, text_color: String) {
        self.text_color = text_color;
    }

    pub fn entire_text_only(&self) -> bool {
        self.entire_text_only
    }

    pub fn set_entire_text_only(&mut self, entire_text_only: bool) {
        self.entire_text_only = entire_text_only;
    }

    pub fn ticks_visible(&self) -> bool {
        self.ticks_visible
    }

    pub fn set_ticks_visible(&mut self, ticks_visible: bool) {
        self.ticks_visible = ticks_visible;
    }

    pub fn minimum_width(&self) -> f64 {
        self.minimum_width
    }

    pub fn set_minimum_width(&mut self, minimum_size: f64) {
        self.minimum_width = minimum_size;
    }
}

impl Default for OverlayPriceScaleOptions {
    fn default() -> Self {
        Self {
            mode:             OverlayPriceScaleMode::default(),
            invert_scale:     defaults::invert_scale(),
            align_labels:     defaults::align_labels(),
            scale_margins:    OverlayPriceScaleMargins::default(),
            border_visible:   defaults::border_visible(),
            border_color:     defaults::border_color(),
            text_color:       defaults::text_color(),
            entire_text_only: defaults::entire_text_only(),
            ticks_visible:    defaults::ticks_visible(),
            minimum_width:    defaults::minimum_width(),
        }
    }
}

mod defaults {
    pub(super) fn invert_scale() -> bool {
        false
    }

    pub(super) fn align_labels() -> bool {
        true
    }

    pub(super) fn border_visible() -> bool {
        true
    }

    pub(super) fn border_color() -> String {
        String::from("#2B2B43")
    }

    pub(super) fn text_color() -> String {
        String::new()
    }

    pub(super) fn entire_text_only() -> bool {
        false
    }

    pub(super) fn ticks_visible() -> bool {
        false
    }

    pub(super) fn minimum_width() -> f64 {
        0.
    }
}
