use serde::{Deserialize, Serialize};

/// Base value of a baseline series: the series is drawn above and below this value with different colors.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum BaselineBaseValue {
    /// A price of the series price scale.
    Price { price: f64 },
}

impl BaselineBaseValue {
    pub fn price(price: f64) -> Self {
        Self::Price {
            price,
        }
    }
}

impl Default for BaselineBaseValue {
    fn default() -> Self {
        Self::price(0.0)
    }
}
