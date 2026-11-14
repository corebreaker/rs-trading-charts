use serde::{Deserialize, Serialize};

/// Horizontal alignment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HorzAlign {
    Left,
    #[default]
    Center,
    Right,
}

/// Vertical alignment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VertAlign {
    Top,
    #[default]
    Center,
    Bottom,
}
