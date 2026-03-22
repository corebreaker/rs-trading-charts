use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum ColorSpace {
    #[serde(rename = "display-p3")]
    DisplayP3,

    #[serde(rename = "srgb")]
    #[default]
    Srgb,
}
