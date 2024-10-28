use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct KineticScrollOptions {
    #[serde(default = "defaults::touch")]
    touch: bool,

    #[serde(default = "defaults::mouse")]
    mouse: bool,
}

impl KineticScrollOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_params(touch: bool, mouse: bool) -> Self {
        Self {
            touch,
            mouse,
        }
    }

    pub fn with_touch(self, touch: bool) -> Self {
        Self {
            touch,
            ..self
        }
    }

    pub fn with_mouse(self, mouse: bool) -> Self {
        Self {
            mouse,
            ..self
        }
    }

    pub fn touch(&self) -> bool {
        self.touch
    }

    pub fn set_touch(&mut self, touch: bool) {
        self.touch = touch;
    }

    pub fn mouse(&self) -> bool {
        self.mouse
    }

    pub fn set_mouse(&mut self, mouse: bool) {
        self.mouse = mouse;
    }
}

impl Default for KineticScrollOptions {
    fn default() -> Self {
        Self {
            touch: defaults::touch(),
            mouse: defaults::mouse(),
        }
    }
}

mod defaults {
    pub(super) fn touch() -> bool {
        true
    }

    pub(super) fn mouse() -> bool {
        false
    }
}
