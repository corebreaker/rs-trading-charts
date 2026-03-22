use super::TrackingModeExitMode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TrackingModeOptions {
    #[serde(rename = "exitMode", default)]
    exit_mode: TrackingModeExitMode,
}

impl TrackingModeOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_exit_mode(exit_mode: TrackingModeExitMode) -> Self {
        Self {
            exit_mode,
        }
    }

    pub fn exit_mode(&self) -> &TrackingModeExitMode {
        &self.exit_mode
    }

    pub fn set_exit_mode(&mut self, exit_mode: TrackingModeExitMode) {
        self.exit_mode = exit_mode;
    }
}
