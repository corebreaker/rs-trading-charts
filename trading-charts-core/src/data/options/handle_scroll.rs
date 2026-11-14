use serde::{Deserialize, Serialize};

/// Represents options for how the chart is scrolled by the mouse and touch gestures.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleScrollOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    mouse_wheel:        Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pressed_mouse_move: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    horz_touch_drag:    Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    vert_touch_drag:    Option<bool>,
}

impl HandleScrollOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable scrolling with the mouse wheel.
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

    /// Enable scrolling by holding down the left mouse button and moving the mouse.
    ///
    /// Default: `true`.
    pub fn with_pressed_mouse_move(self, pressed_mouse_move: bool) -> Self {
        Self {
            pressed_mouse_move: Some(pressed_mouse_move),
            ..self
        }
    }

    pub fn pressed_mouse_move(&self) -> Option<bool> {
        self.pressed_mouse_move
    }

    /// Set or reset (with `None`) the `pressedMouseMove` option.
    pub fn set_pressed_mouse_move(&mut self, pressed_mouse_move: Option<bool>) {
        self.pressed_mouse_move = pressed_mouse_move;
    }

    /// Enable horizontal touch scrolling.
    ///
    /// When enabled the chart handles touch gestures that would normally scroll the webpage horizontally.
    ///
    /// Default: `true`.
    pub fn with_horz_touch_drag(self, horz_touch_drag: bool) -> Self {
        Self {
            horz_touch_drag: Some(horz_touch_drag),
            ..self
        }
    }

    pub fn horz_touch_drag(&self) -> Option<bool> {
        self.horz_touch_drag
    }

    /// Set or reset (with `None`) the `horzTouchDrag` option.
    pub fn set_horz_touch_drag(&mut self, horz_touch_drag: Option<bool>) {
        self.horz_touch_drag = horz_touch_drag;
    }

    /// Enable vertical touch scrolling.
    ///
    /// When enabled the chart handles touch gestures that would normally scroll the webpage vertically.
    ///
    /// Default: `true`.
    pub fn with_vert_touch_drag(self, vert_touch_drag: bool) -> Self {
        Self {
            vert_touch_drag: Some(vert_touch_drag),
            ..self
        }
    }

    pub fn vert_touch_drag(&self) -> Option<bool> {
        self.vert_touch_drag
    }

    /// Set or reset (with `None`) the `vertTouchDrag` option.
    pub fn set_vert_touch_drag(&mut self, vert_touch_drag: Option<bool>) {
        self.vert_touch_drag = vert_touch_drag;
    }
}
