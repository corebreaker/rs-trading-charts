use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct SolidColor {
    #[serde(default = "defaults::color")]
    color: String,
}

impl SolidColor {
    pub fn new(color: impl Into<String>) -> Self {
        Self {
            color: color.into()
        }
    }

    pub fn color(&self) -> &str {
        &self.color
    }
}

impl Default for SolidColor {
    fn default() -> Self {
        Self {
            color: defaults::color(),
        }
    }
}

mod defaults {
    pub(super) fn color() -> String {
        String::from("#FFFFFF")
    }
}
