use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HandleScrollOptions {
    #[serde(rename = "mouseWheel", default = "defaults::mouse_wheel")]
    mouse_wheel: bool,

    #[serde(rename = "pressedMouseMove", default = "defaults::pressed_mouse_move")]
    pressed_mouse_move: bool,

    #[serde(rename = "horzTouchDrag", default = "defaults::horz_touch_drag")]
    horz_touch_drag: bool,

    #[serde(rename = "vertTouchDrag", default = "defaults::vert_touch_drag")]
    vert_touch_drag: bool,
}

impl HandleScrollOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mouse_wheel(self, mouse_wheel: bool) -> Self {
        Self {
            mouse_wheel,
            ..self
        }
    }

    pub fn with_pressed_mouse_move(self, pressed_mouse_move: bool) -> Self {
        Self {
            pressed_mouse_move,
            ..self
        }
    }

    pub fn with_horz_touch_drag(self, horz_touch_drag: bool) -> Self {
        Self {
            horz_touch_drag,
            ..self
        }
    }

    pub fn with_vert_touch_drag(self, vert_touch_drag: bool) -> Self {
        Self {
            vert_touch_drag,
            ..self
        }
    }

    pub fn mouse_wheel(&self) -> bool {
        self.mouse_wheel
    }

    pub fn set_mouse_wheel(&mut self, mouse_wheel: bool) {
        self.mouse_wheel = mouse_wheel;
    }

    pub fn pressed_mouse_move(&self) -> bool {
        self.pressed_mouse_move
    }

    pub fn set_pressed_mouse_move(&mut self, pressed_mouse_move: bool) {
        self.pressed_mouse_move = pressed_mouse_move;
    }

    pub fn horz_touch_drag(&self) -> bool {
        self.horz_touch_drag
    }

    pub fn set_horz_touch_drag(&mut self, horz_touch_drag: bool) {
        self.horz_touch_drag = horz_touch_drag;
    }

    pub fn vert_touch_drag(&self) -> bool {
        self.vert_touch_drag
    }

    pub fn set_vert_touch_drag(&mut self, vert_touch_drag: bool) {
        self.vert_touch_drag = vert_touch_drag;
    }
}

impl Default for HandleScrollOptions {
    fn default() -> Self {
        Self {
            mouse_wheel:        defaults::mouse_wheel(),
            pressed_mouse_move: defaults::pressed_mouse_move(),
            horz_touch_drag:    defaults::horz_touch_drag(),
            vert_touch_drag:    defaults::vert_touch_drag(),
        }
    }
}

mod defaults {
    pub(super) fn mouse_wheel() -> bool {
        true
    }

    pub(super) fn pressed_mouse_move() -> bool {
        true
    }

    pub(super) fn horz_touch_drag() -> bool {
        true
    }

    pub(super) fn vert_touch_drag() -> bool {
        true
    }
}
