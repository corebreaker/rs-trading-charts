use super::LayoutOptions;
use num_traits::Zero;
use serde::{Deserialize, Serialize};

fn default_auto_size() -> bool {
    true
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ChartOptions {
    layout: LayoutOptions,

    #[serde(skip_serializing_if = "Zero::is_zero")]
    width: usize,

    #[serde(skip_serializing_if = "Zero::is_zero")]
    height: usize,

    #[serde(rename = "autoSize", default = "default_auto_size")]
    auto_size: bool,
}

impl ChartOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_layout(self, layout: LayoutOptions) -> Self {
        Self {
            layout,
            ..self
        }
    }

    pub fn with_width(self, width: usize) -> Self {
        Self {
            width,
            ..self
        }
    }

    pub fn with_height(self, height: usize) -> Self {
        Self {
            height,
            ..self
        }
    }

    pub fn with_auto_size(self, auto_size: bool) -> Self {
        Self {
            auto_size,
            ..self
        }
    }

    pub fn layout(&self) -> &LayoutOptions {
        &self.layout
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn auto_size(&self) -> bool {
        self.auto_size
    }
}
