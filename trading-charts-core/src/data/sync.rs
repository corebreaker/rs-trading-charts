use serde::{Deserialize, Serialize};

/// What is synchronized between the charts of a synchronization group, see
/// [`ChartHandle::set_sync_group`](crate::ChartHandle::set_sync_group).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncOptions {
    /// Synchronize the crosshair: the other charts show it on their first series, at the same time.
    pub crosshair:     bool,
    /// Synchronize the visible logical range (the visible bars), for charts sharing the same bars.
    pub logical_range: bool,
    /// Synchronize the visible time range, for charts with different bars.
    pub time_range:    bool,
}

impl SyncOptions {
    /// Synchronize the crosshair and the visible logical range.
    pub fn new() -> Self {
        Self::default()
    }

    /// Synchronize the crosshair only.
    pub fn crosshair_only() -> Self {
        Self {
            crosshair:     true,
            logical_range: false,
            time_range:    false,
        }
    }

    pub fn with_crosshair(self, crosshair: bool) -> Self {
        Self {
            crosshair,
            ..self
        }
    }

    pub fn with_logical_range(self, logical_range: bool) -> Self {
        Self {
            logical_range,
            ..self
        }
    }

    pub fn with_time_range(self, time_range: bool) -> Self {
        Self {
            time_range,
            ..self
        }
    }
}

impl Default for SyncOptions {
    fn default() -> Self {
        Self {
            crosshair:     true,
            logical_range: true,
            time_range:    false,
        }
    }
}
