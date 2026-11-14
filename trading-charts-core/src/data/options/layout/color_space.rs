use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Default, PartialEq, Debug)]
pub enum ColorSpace {
    #[serde(rename = "display-p3")]
    DisplayP3,

    #[serde(rename = "srgb")]
    #[default]
    Srgb,
}
