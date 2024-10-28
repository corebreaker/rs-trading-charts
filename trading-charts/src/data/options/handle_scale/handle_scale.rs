use super::{super::FlagableOptions, AxisDoubleClickOptions, AxisPressedMouseMoveOptions};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HandleScaleOptions {
    mouse_wheel: bool,

    pinch: bool,

    #[serde(
        rename = "axisPressedMouseMove",
        skip_serializing_if = "FlagableOptions::is_none",
        default
    )]
    axis_pressed_mouse_move: FlagableOptions<AxisPressedMouseMoveOptions>,

    #[serde(
        rename = "axisDoubleClickReset",
        skip_serializing_if = "FlagableOptions::is_none",
        default
    )]
    axis_double_click_reset: FlagableOptions<AxisDoubleClickOptions>,
}

impl HandleScaleOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_options(
        mouse_wheel: bool,
        pinch: bool,
        axis_pressed_mouse_move: FlagableOptions<AxisPressedMouseMoveOptions>,
        axis_double_click_reset: FlagableOptions<AxisDoubleClickOptions>,
    ) -> Self {
        Self {
            mouse_wheel,
            pinch,
            axis_pressed_mouse_move,
            axis_double_click_reset,
        }
    }

    pub fn mouse_wheel(&self) -> bool {
        self.mouse_wheel
    }

    pub fn set_mouse_wheel(&mut self, value: bool) {
        self.mouse_wheel = value;
    }

    pub fn pinch(&self) -> bool {
        self.pinch
    }

    pub fn set_pinch(&mut self, value: bool) {
        self.pinch = value;
    }

    pub fn axis_pressed_mouse_move(&self) -> &FlagableOptions<AxisPressedMouseMoveOptions> {
        &self.axis_pressed_mouse_move
    }

    pub fn axis_pressed_mouse_move_mut(&mut self) -> &mut FlagableOptions<AxisPressedMouseMoveOptions> {
        &mut self.axis_pressed_mouse_move
    }

    pub fn set_axis_pressed_mouse_move(&mut self, value: FlagableOptions<AxisPressedMouseMoveOptions>) {
        self.axis_pressed_mouse_move = value;
    }

    pub fn axis_double_click_reset(&self) -> &FlagableOptions<AxisDoubleClickOptions> {
        &self.axis_double_click_reset
    }

    pub fn axis_double_click_reset_mut(&mut self) -> &mut FlagableOptions<AxisDoubleClickOptions> {
        &mut self.axis_double_click_reset
    }

    pub fn set_axis_double_click_reset(&mut self, value: FlagableOptions<AxisDoubleClickOptions>) {
        self.axis_double_click_reset = value;
    }
}

impl Default for HandleScaleOptions {
    fn default() -> Self {
        Self {
            mouse_wheel:             defaults::mouse_wheel(),
            pinch:                   defaults::pinch(),
            axis_pressed_mouse_move: FlagableOptions::default(),
            axis_double_click_reset: FlagableOptions::default(),
        }
    }
}

mod defaults {
    pub(super) fn mouse_wheel() -> bool {
        true
    }

    pub(super) fn pinch() -> bool {
        true
    }
}
