use super::UTCTimestamp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HistogramData {
    time:  UTCTimestamp,
    value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
}

impl HistogramData {
    pub fn new(time: UTCTimestamp, value: f64) -> Self {
        Self {
            time,
            value,
            color: None,
        }
    }

    pub fn with_color(self, color: String) -> Self {
        Self {
            color: Some(color),
            ..self
        }
    }

    pub fn time(&self) -> UTCTimestamp {
        self.time
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }
}
