use super::MarkerType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Marker {
    time: String,
    text: String,

    #[serde(rename = "type")]
    r#type: MarkerType,
}

impl Marker {
    pub fn new(time: String, r#type: MarkerType) -> Self {
        Self {
            time,
            r#type,
            text: String::new(),
        }
    }

    pub fn buy(time: String) -> Self {
        Self {
            time,
            r#type: MarkerType::Buy,
            text: String::new(),
        }
    }

    pub fn sell(time: String) -> Self {
        Self {
            time,
            r#type: MarkerType::Sell,
            text: String::new(),
        }
    }

    pub fn remove(time: String) -> Self {
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

    pub fn time(&self) -> &str {
        &self.time
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn r#type(&self) -> MarkerType {
        self.r#type
    }
}
