use crate::data::options::tracking_mode::TrackingModeExitMode;
use serde::{Deserialize, Serialize};

/// Represent options for the tracking mode's behavior.
///
/// Mobile users will not have the ability to see the values/dates like they do on desktop. To see it,
/// they should enter the tracking mode. The tracking mode will deactivate the scrolling and make it possible to check
/// values and dates.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingModeOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    exit_mode: Option<TrackingModeExitMode>,
}

impl TrackingModeOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Default: `TrackingModeExitMode.OnNextTap`.
    pub fn with_exit_mode(self, exit_mode: TrackingModeExitMode) -> Self {
        Self {
            exit_mode: Some(exit_mode),
        }
    }

    pub fn exit_mode(&self) -> Option<TrackingModeExitMode> {
        self.exit_mode
    }

    /// Set or reset (with `None`) the `exitMode` option.
    pub fn set_exit_mode(&mut self, exit_mode: Option<TrackingModeExitMode>) {
        self.exit_mode = exit_mode;
    }
}
