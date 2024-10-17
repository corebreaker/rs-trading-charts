use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Candle {
    time:  String,
    open:  f64,
    high:  f64,
    low:   f64,
    close: f64,
}

impl Candle {
    pub fn new(time: String, open: f64, high: f64, low: f64, close: f64) -> Self {
        Self {
            time,
            open,
            high,
            low,
            close,
        }
    }

    pub fn time(&self) -> &str {
        &self.time
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