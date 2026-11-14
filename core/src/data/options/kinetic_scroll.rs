use serde::{Deserialize, Serialize};

/// Represents options for enabling or disabling kinetic scrolling with mouse and touch gestures.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KineticScrollOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    touch: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    mouse: Option<bool>,
}

impl KineticScrollOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable kinetic scroll with touch gestures.
    ///
    /// Default: `true`.
    pub fn with_touch(self, touch: bool) -> Self {
        Self {
            touch: Some(touch),
            ..self
        }
    }

    pub fn touch(&self) -> Option<bool> {
        self.touch
    }

    /// Set or reset (with `None`) the `touch` option.
    pub fn set_touch(&mut self, touch: Option<bool>) {
        self.touch = touch;
    }

    /// Enable kinetic scroll with the mouse.
    ///
    /// Default: `false`.
    pub fn with_mouse(self, mouse: bool) -> Self {
        Self {
            mouse: Some(mouse),
            ..self
        }
    }

    pub fn mouse(&self) -> Option<bool> {
        self.mouse
    }

    /// Set or reset (with `None`) the `mouse` option.
    pub fn set_mouse(&mut self, mouse: Option<bool>) {
        self.mouse = mouse;
    }
}
