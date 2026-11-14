use {
    crate::data::options::FlagableOptions,
    crate::data::options::handle_scale::AxisDoubleClickOptions,
    crate::data::options::handle_scale::AxisPressedMouseMoveOptions,
};
use serde::{Deserialize, Serialize};

/// Represents options for how the chart is scaled by the mouse and touch gestures.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleScaleOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    mouse_wheel:             Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pinch:                   Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    axis_pressed_mouse_move: Option<FlagableOptions<AxisPressedMouseMoveOptions>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    axis_double_click_reset: Option<FlagableOptions<AxisDoubleClickOptions>>,
}

impl HandleScaleOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable scaling with the mouse wheel.
    ///
    /// Default: `true`.
    pub fn with_mouse_wheel(self, mouse_wheel: bool) -> Self {
        Self {
            mouse_wheel: Some(mouse_wheel),
            ..self
        }
    }

    pub fn mouse_wheel(&self) -> Option<bool> {
        self.mouse_wheel
    }

    /// Set or reset (with `None`) the `mouseWheel` option.
    pub fn set_mouse_wheel(&mut self, mouse_wheel: Option<bool>) {
        self.mouse_wheel = mouse_wheel;
    }

    /// Enable scaling with pinch/zoom gestures.
    ///
    /// Default: `true`.
    pub fn with_pinch(self, pinch: bool) -> Self {
        Self {
            pinch: Some(pinch),
            ..self
        }
    }

    pub fn pinch(&self) -> Option<bool> {
        self.pinch
    }

    /// Set or reset (with `None`) the `pinch` option.
    pub fn set_pinch(&mut self, pinch: Option<bool>) {
        self.pinch = pinch;
    }

    /// Enable scaling the price and/or time scales by holding down the left mouse button and moving the mouse.
    pub fn with_axis_pressed_mouse_move(
        self,
        axis_pressed_mouse_move: impl Into<FlagableOptions<AxisPressedMouseMoveOptions>>,
    ) -> Self {
        Self {
            axis_pressed_mouse_move: Some(axis_pressed_mouse_move.into()),
            ..self
        }
    }

    pub fn axis_pressed_mouse_move(&self) -> Option<&FlagableOptions<AxisPressedMouseMoveOptions>> {
        self.axis_pressed_mouse_move.as_ref()
    }

    /// Set or reset (with `None`) the `axisPressedMouseMove` option.
    pub fn set_axis_pressed_mouse_move(
        &mut self,
        axis_pressed_mouse_move: Option<FlagableOptions<AxisPressedMouseMoveOptions>>,
    ) {
        self.axis_pressed_mouse_move = axis_pressed_mouse_move;
    }

    /// Enable resetting scaling by double-clicking the left mouse button.
    pub fn with_axis_double_click_reset(
        self,
        axis_double_click_reset: impl Into<FlagableOptions<AxisDoubleClickOptions>>,
    ) -> Self {
        Self {
            axis_double_click_reset: Some(axis_double_click_reset.into()),
            ..self
        }
    }

    pub fn axis_double_click_reset(&self) -> Option<&FlagableOptions<AxisDoubleClickOptions>> {
        self.axis_double_click_reset.as_ref()
    }

    /// Set or reset (with `None`) the `axisDoubleClickReset` option.
    pub fn set_axis_double_click_reset(
        &mut self,
        axis_double_click_reset: Option<FlagableOptions<AxisDoubleClickOptions>>,
    ) {
        self.axis_double_click_reset = axis_double_click_reset;
    }
}
