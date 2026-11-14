use serde::{Deserialize, Serialize};

/// Side of a visible price scale.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PriceScaleSide {
    Left,
    #[default]
    Right,
}

impl PriceScaleSide {
    /// Identifier of the price scale, to use as the `priceScaleId` option of a series.
    pub fn id(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

impl From<PriceScaleSide> for String {
    fn from(side: PriceScaleSide) -> Self {
        side.id().to_string()
    }
}
