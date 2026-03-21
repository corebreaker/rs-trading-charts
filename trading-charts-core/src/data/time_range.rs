use super::UTCTimestamp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct TimeRange {
    from: UTCTimestamp,
    to:   UTCTimestamp,
}

impl TimeRange {
    pub fn new(from: UTCTimestamp, to: UTCTimestamp) -> Self {
        Self {
            from,
            to,
        }
    }

    pub fn from(&self) -> UTCTimestamp {
        self.from
    }

    pub fn to(&self) -> UTCTimestamp {
        self.to
    }
}
