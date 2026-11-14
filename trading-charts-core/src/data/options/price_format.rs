use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PriceFormatType {
    #[default]
    Price,
    Volume,
    Percent,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PriceFormatOptions {
    #[serde(rename = "type")]
    kind:      PriceFormatType,
    #[serde(skip_serializing_if = "Option::is_none")]
    precision: Option<usize>,
    #[serde(rename = "minMove", skip_serializing_if = "Option::is_none")]
    min_move:  Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base:      Option<f64>,
}

impl PriceFormatOptions {
    pub fn price() -> Self {
        Self::new(PriceFormatType::Price)
    }

    pub fn volume() -> Self {
        Self::new(PriceFormatType::Volume)
    }

    pub fn percent() -> Self {
        Self::new(PriceFormatType::Percent)
    }

    pub fn new(kind: PriceFormatType) -> Self {
        Self {
            kind,
            precision: None,
            min_move: None,
            base: None,
        }
    }

    pub fn with_precision(self, precision: usize) -> Self {
        Self {
            precision: Some(precision),
            ..self
        }
    }

    /// Minimal step of the price, used to round the prices. Default: `0.01`.
    pub fn with_min_move(self, min_move: f64) -> Self {
        Self {
            min_move: Some(min_move),
            ..self
        }
    }

    /// Base of the prices, the inverse of `min_move`, which takes precedence over `min_move` when both are set.
    pub fn with_base(self, base: f64) -> Self {
        Self {
            base: Some(base),
            ..self
        }
    }
}
