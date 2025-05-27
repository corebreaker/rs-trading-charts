use super::{
    cross_hair::CrossHairOptions,
    grid::GridOptions,
    handle_scale::HandleScaleOptions,
    layout::LayoutOptions,
    overlay_price_scale::OverlayPriceScaleOptions,
    price_scale::PriceScaleOptions,
    FlagableOptions,
    HandleScrollOptions,
    KineticScrollOptions,
    TimeScaleOptions,
    TrackingModeOptions,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ChartOptions {
    #[serde(default = "defaults::width")]
    width: usize,

    #[serde(default = "defaults::height")]
    height: usize,

    #[serde(rename = "autoSize", default = "defaults::auto_size")]
    auto_size: bool,

    #[serde(default)]
    layout: LayoutOptions,

    #[serde(rename = "leftPriceScale", default)]
    left_price_scale: PriceScaleOptions,

    #[serde(rename = "rightPriceScale", default)]
    right_price_scale: PriceScaleOptions,

    #[serde(rename = "overlayPriceScale", default)]
    overlay_price_scale_options: OverlayPriceScaleOptions,

    #[serde(rename = "timeScale", default)]
    time_scale: TimeScaleOptions,

    #[serde(rename = "crossHair", default)]
    cross_hair: CrossHairOptions,

    #[serde(default)]
    grid: GridOptions,

    #[serde(rename = "handleScroll", default)]
    handle_scroll: FlagableOptions<HandleScrollOptions>,

    #[serde(rename = "handleScale", default)]
    handle_scale: FlagableOptions<HandleScaleOptions>,

    #[serde(rename = "kineticScroll", default)]
    kinetic_scroll: KineticScrollOptions,

    #[serde(rename = "trackingMode", default)]
    tracking_mode: TrackingModeOptions,
}

impl ChartOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_width(self, width: usize) -> Self {
        Self {
            width,
            ..self
        }
    }

    pub fn with_height(self, height: usize) -> Self {
        Self {
            height,
            ..self
        }
    }

    pub fn with_auto_size(self, auto_size: bool) -> Self {
        Self {
            auto_size,
            ..self
        }
    }

    pub fn with_layout(self, layout: LayoutOptions) -> Self {
        Self {
            layout,
            ..self
        }
    }

    pub fn with_left_price_scale(self, left_price_scale: PriceScaleOptions) -> Self {
        Self {
            left_price_scale,
            ..self
        }
    }

    pub fn with_right_price_scale(self, right_price_scale: PriceScaleOptions) -> Self {
        Self {
            right_price_scale,
            ..self
        }
    }

    pub fn with_overlay_price_scale_options(self, overlay_price_scale_options: OverlayPriceScaleOptions) -> Self {
        Self {
            overlay_price_scale_options,
            ..self
        }
    }

    pub fn with_time_scale(self, time_scale: TimeScaleOptions) -> Self {
        Self {
            time_scale,
            ..self
        }
    }

    pub fn with_cross_hair(self, cross_hair: CrossHairOptions) -> Self {
        Self {
            cross_hair,
            ..self
        }
    }

    pub fn with_grid(self, grid: GridOptions) -> Self {
        Self {
            grid,
            ..self
        }
    }

    pub fn with_handle_scroll(self, handle_scroll: FlagableOptions<HandleScrollOptions>) -> Self {
        Self {
            handle_scroll,
            ..self
        }
    }

    pub fn with_handle_scale(self, handle_scale: FlagableOptions<HandleScaleOptions>) -> Self {
        Self {
            handle_scale,
            ..self
        }
    }

    pub fn with_kinetic_scroll_options(self, kinetic_scroll_options: KineticScrollOptions) -> Self {
        Self {
            kinetic_scroll: kinetic_scroll_options,
            ..self
        }
    }

    pub fn with_tracking_mode(self, tracking_mode: TrackingModeOptions) -> Self {
        Self {
            tracking_mode,
            ..self
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    pub fn auto_size(&self) -> bool {
        self.auto_size
    }

    pub fn set_auto_size(&mut self, auto_size: bool) {
        self.auto_size = auto_size;
    }

    pub fn layout(&self) -> &LayoutOptions {
        &self.layout
    }

    pub fn layout_mut(&mut self) -> &mut LayoutOptions {
        &mut self.layout
    }

    pub fn set_layout(&mut self, layout: LayoutOptions) {
        self.layout = layout;
    }

    pub fn left_price_scale(&self) -> &PriceScaleOptions {
        &self.left_price_scale
    }

    pub fn left_price_scale_mut(&mut self) -> &mut PriceScaleOptions {
        &mut self.left_price_scale
    }

    pub fn set_left_price_scale(&mut self, left_price_scale: PriceScaleOptions) {
        self.left_price_scale = left_price_scale;
    }

    pub fn right_price_scale(&self) -> &PriceScaleOptions {
        &self.right_price_scale
    }

    pub fn right_price_scale_mut(&mut self) -> &mut PriceScaleOptions {
        &mut self.right_price_scale
    }

    pub fn set_right_price_scale(&mut self, right_price_scale: PriceScaleOptions) {
        self.right_price_scale = right_price_scale;
    }

    pub fn overlay_price_scale_options(&self) -> &OverlayPriceScaleOptions {
        &self.overlay_price_scale_options
    }

    pub fn overlay_price_scale_options_mut(&mut self) -> &mut OverlayPriceScaleOptions {
        &mut self.overlay_price_scale_options
    }

    pub fn set_overlay_price_scale_options(&mut self, overlay_price_scale_options: OverlayPriceScaleOptions) {
        self.overlay_price_scale_options = overlay_price_scale_options;
    }

    pub fn time_scale(&self) -> &TimeScaleOptions {
        &self.time_scale
    }

    pub fn time_scale_mut(&mut self) -> &mut TimeScaleOptions {
        &mut self.time_scale
    }

    pub fn set_time_scale(&mut self, time_scale: TimeScaleOptions) {
        self.time_scale = time_scale;
    }

    pub fn cross_hair(&self) -> &CrossHairOptions {
        &self.cross_hair
    }

    pub fn cross_hair_mut(&mut self) -> &mut CrossHairOptions {
        &mut self.cross_hair
    }

    pub fn set_cross_hair(&mut self, cross_hair: CrossHairOptions) {
        self.cross_hair = cross_hair;
    }

    pub fn grid(&self) -> &GridOptions {
        &self.grid
    }

    pub fn grid_mut(&mut self) -> &mut GridOptions {
        &mut self.grid
    }

    pub fn set_grid(&mut self, grid: GridOptions) {
        self.grid = grid;
    }

    pub fn handle_scroll(&self) -> &FlagableOptions<HandleScrollOptions> {
        &self.handle_scroll
    }

    pub fn handle_scroll_mut(&mut self) -> &mut FlagableOptions<HandleScrollOptions> {
        &mut self.handle_scroll
    }

    pub fn set_handle_scroll(&mut self, handle_scroll: FlagableOptions<HandleScrollOptions>) {
        self.handle_scroll = handle_scroll;
    }

    pub fn handle_scale(&self) -> &FlagableOptions<HandleScaleOptions> {
        &self.handle_scale
    }

    pub fn handle_scale_mut(&mut self) -> &mut FlagableOptions<HandleScaleOptions> {
        &mut self.handle_scale
    }

    pub fn set_handle_scale(&mut self, handle_scale: FlagableOptions<HandleScaleOptions>) {
        self.handle_scale = handle_scale;
    }

    pub fn kinetic_scroll_options(&self) -> &KineticScrollOptions {
        &self.kinetic_scroll
    }

    pub fn kinetic_scroll_options_mut(&mut self) -> &mut KineticScrollOptions {
        &mut self.kinetic_scroll
    }

    pub fn set_kinetic_scroll_options(&mut self, kinetic_scroll_options: KineticScrollOptions) {
        self.kinetic_scroll = kinetic_scroll_options;
    }

    pub fn tracking_mode(&self) -> &TrackingModeOptions {
        &self.tracking_mode
    }

    pub fn tracking_mode_mut(&mut self) -> &mut TrackingModeOptions {
        &mut self.tracking_mode
    }

    pub fn set_tracking_mode(&mut self, tracking_mode: TrackingModeOptions) {
        self.tracking_mode = tracking_mode;
    }
}

mod defaults {
    pub(super) fn width() -> usize {
        0
    }

    pub(super) fn height() -> usize {
        0
    }

    pub(super) fn auto_size() -> bool {
        false
    }
}
