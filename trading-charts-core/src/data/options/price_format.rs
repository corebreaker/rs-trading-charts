use crate::data::format::PriceFormatter;
use serde::{Deserialize, Serialize};

/// Kind of price format of a series.
#[derive(Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PriceFormatType {
    /// Prices, with a fixed precision.
    #[default]
    Price,

    /// Volumes, abbreviated with the `K`, `M` and `B` suffixes.
    Volume,

    /// Percentages.
    Percent,

    /// Prices formatted by a [`PriceFormatter`].
    Custom,
}

/// Price format of a series, used by its price scale, its last value label and the crosshair label.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceFormatOptions {
    #[serde(rename = "type")]
    kind:                PriceFormatType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    precision:           Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    min_move:            Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base:                Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    formatter:           Option<PriceFormatter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tickmarks_formatter: Option<PriceFormatter>,
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

    /// Format the prices with a [`PriceFormatter`].
    pub fn custom(formatter: PriceFormatter) -> Self {
        Self {
            formatter: Some(formatter),
            ..Self::new(PriceFormatType::Custom)
        }
    }

    pub fn new(kind: PriceFormatType) -> Self {
        Self {
            kind,
            precision: None,
            min_move: None,
            base: None,
            formatter: None,
            tickmarks_formatter: None,
        }
    }

    /// Number of decimals of the prices. Default: `2`.
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

    /// Formatter of the labels of the price scale, for a custom price format. The formatter of the prices is used
    /// when it is not set.
    pub fn with_tickmarks_formatter(self, formatter: PriceFormatter) -> Self {
        Self {
            tickmarks_formatter: Some(formatter),
            ..self
        }
    }

    pub fn kind(&self) -> PriceFormatType {
        self.kind
    }

    pub fn precision(&self) -> Option<usize> {
        self.precision
    }

    pub fn min_move(&self) -> Option<f64> {
        self.min_move
    }

    pub fn base(&self) -> Option<f64> {
        self.base
    }

    pub fn formatter(&self) -> Option<&PriceFormatter> {
        self.formatter.as_ref()
    }

    pub fn tickmarks_formatter(&self) -> Option<&PriceFormatter> {
        self.tickmarks_formatter.as_ref()
    }
}
