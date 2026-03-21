use super::UTCTimestamp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Candlestick {
    time:  UTCTimestamp,
    open:  f64,
    high:  f64,
    low:   f64,
    close: f64,
}

impl Candlestick {
    pub fn new(time: UTCTimestamp, open: f64, high: f64, low: f64, close: f64) -> Self {
        Self {
            time,
            open,
            high,
            low,
            close,
        }
    }

    pub fn time(&self) -> UTCTimestamp {
        self.time
    }

    pub fn open(&self) -> f64 {
        self.open
    }

    pub fn high(&self) -> f64 {
        self.high
    }

    pub fn low(&self) -> f64 {
        self.low
    }

    pub fn close(&self) -> f64 {
        self.close
    }
}
