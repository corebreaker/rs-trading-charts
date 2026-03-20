use super::UTCTimestamp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ValueData {
    time: UTCTimestamp,
    value: f64,
}

impl ValueData {
    pub fn new(time: UTCTimestamp, value: f64) -> Self {
        Self { time, value }
    }

    pub fn time(&self) -> UTCTimestamp {
        self.time
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
