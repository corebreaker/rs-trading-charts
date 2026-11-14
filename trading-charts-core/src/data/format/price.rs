use serde::{Deserialize, Serialize};

/// How a currency is displayed by [`PriceFormatter::Currency`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CurrencyDisplay {
    /// Localized symbol, such as `€`.
    #[default]
    Symbol,

    /// Narrow symbol, such as `$` instead of `US$`.
    NarrowSymbol,

    /// ISO currency code, such as `EUR`.
    Code,

    /// Localized currency name, such as `euros`.
    Name,
}

/// Formatter of prices, implemented by the JS binding.
///
/// It is used by the localization options of the chart (prices and percentages of the price scales and of the
/// crosshair labels) and by the price format of the series (see
/// [`PriceFormatOptions::custom`](crate::data::options::PriceFormatOptions::custom)).
///
/// The `locale` parameters are BCP 47 language tags, such as `en-US` or `fr-FR`; when not set, the locale of the
/// browser is used.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "$priceFormatter", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PriceFormatter {
    /// The built-in formatting of Lightweight Charts, to remove a formatter set before.
    Default,

    /// Fixed number of decimals, without grouping: `1234.50`.
    Fixed { precision: u8 },

    /// Localized number, with grouping: `1,234.5` or `1 234,5`.
    Locale {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        locale:                  Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        minimum_fraction_digits: Option<u8>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        maximum_fraction_digits: Option<u8>,
    },

    /// Localized amount of a currency: `$1,234.50` or `1 234,50 €`.
    Currency {
        /// ISO 4217 currency code, such as `USD` or `EUR`.
        currency:                String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        locale:                  Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        currency_display:        Option<CurrencyDisplay>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        minimum_fraction_digits: Option<u8>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        maximum_fraction_digits: Option<u8>,
    },

    /// Value followed by a percent sign, the value being already a percentage: `12.50%`.
    Percent { precision: u8 },

    /// Compact localized number, for volumes: `1.2K`, `3.4M`.
    Compact {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        locale:                  Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        minimum_fraction_digits: Option<u8>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        maximum_fraction_digits: Option<u8>,
    },

    /// Scientific notation: `1.23e+4`.
    Scientific { precision: u8 },

    /// Fixed number of decimals between a prefix and a suffix, for units: `≈ 12.5 pts`.
    Affix {
        prefix:    String,
        suffix:    String,
        precision: u8,
    },

    /// Function registered with [`register_price_formatter`](super::register_price_formatter).
    Custom { name: String },
}

impl PriceFormatter {
    pub fn fixed(precision: u8) -> Self {
        Self::Fixed {
            precision,
        }
    }

    /// Localized number, in the given locale or in the browser locale when `None`.
    pub fn locale(locale: Option<&str>) -> Self {
        Self::Locale {
            locale:                  locale.map(String::from),
            minimum_fraction_digits: None,
            maximum_fraction_digits: None,
        }
    }

    /// Localized amount of the given ISO 4217 currency, in the browser locale.
    pub fn currency(currency: impl Into<String>) -> Self {
        Self::Currency {
            currency:                currency.into(),
            locale:                  None,
            currency_display:        None,
            minimum_fraction_digits: None,
            maximum_fraction_digits: None,
        }
    }

    /// Localized amount of the given ISO 4217 currency, in the given locale.
    pub fn currency_in_locale(currency: impl Into<String>, locale: impl Into<String>) -> Self {
        Self::Currency {
            currency:                currency.into(),
            locale:                  Some(locale.into()),
            currency_display:        None,
            minimum_fraction_digits: None,
            maximum_fraction_digits: None,
        }
    }

    pub fn percent(precision: u8) -> Self {
        Self::Percent {
            precision,
        }
    }

    /// Compact number in the browser locale.
    pub fn compact() -> Self {
        Self::Compact {
            locale:                  None,
            minimum_fraction_digits: None,
            maximum_fraction_digits: None,
        }
    }

    pub fn scientific(precision: u8) -> Self {
        Self::Scientific {
            precision,
        }
    }

    pub fn affix(prefix: impl Into<String>, suffix: impl Into<String>, precision: u8) -> Self {
        Self::Affix {
            prefix: prefix.into(),
            suffix: suffix.into(),
            precision,
        }
    }

    /// Function registered with [`register_price_formatter`](super::register_price_formatter) under `name`.
    pub fn custom(name: impl Into<String>) -> Self {
        Self::Custom {
            name: name.into()
        }
    }

    /// Set the number of decimals of the `Locale`, `Currency` and `Compact` formatters, which is
    /// `minimum..=maximum`. Other formatters are returned unchanged.
    pub fn with_fraction_digits(self, minimum: u8, maximum: u8) -> Self {
        match self {
            Self::Locale {
                locale, ..
            } => Self::Locale {
                locale,
                minimum_fraction_digits: Some(minimum),
                maximum_fraction_digits: Some(maximum),
            },
            Self::Currency {
                currency,
                locale,
                currency_display,
                ..
            } => Self::Currency {
                currency,
                locale,
                currency_display,
                minimum_fraction_digits: Some(minimum),
                maximum_fraction_digits: Some(maximum),
            },
            Self::Compact {
                locale, ..
            } => Self::Compact {
                locale,
                minimum_fraction_digits: Some(minimum),
                maximum_fraction_digits: Some(maximum),
            },
            other => other,
        }
    }

    /// Set the currency display of a `Currency` formatter. Other formatters are returned unchanged.
    pub fn with_currency_display(self, display: CurrencyDisplay) -> Self {
        match self {
            Self::Currency {
                currency,
                locale,
                minimum_fraction_digits,
                maximum_fraction_digits,
                ..
            } => Self::Currency {
                currency,
                locale,
                currency_display: Some(display),
                minimum_fraction_digits,
                maximum_fraction_digits,
            },
            other => other,
        }
    }
}
