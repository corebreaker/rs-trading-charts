use super::{MarkerType, UTCTimestamp};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Marker {
    time: UTCTimestamp,
    text: String,

    #[serde(rename = "type")]
    r#type: MarkerType,
}

impl Marker {
    pub fn new(time: UTCTimestamp, r#type: MarkerType) -> Self {
        Self {
            time,
            r#type,
            text: String::new(),
        }
    }

    pub fn buy(time: UTCTimestamp) -> Self {
        Self {
            time,
            r#type: MarkerType::Buy,
            text: String::new(),
        }
    }

    pub fn sell(time: UTCTimestamp) -> Self {
        Self {
            time,
            r#type: MarkerType::Sell,
            text: String::new(),
        }
    }

    pub fn remove(time: UTCTimestamp) -> Self {
        Self {
            time,
            r#type: MarkerType::Remove,
            text: String::new(),
        }
    }

    pub fn with_text(self, text: String) -> Self {
        Self {
            text,
            ..self
        }
    }

    pub fn time(&self) -> UTCTimestamp {
        self.time
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn r#type(&self) -> MarkerType {
        self.r#type
    }
}
