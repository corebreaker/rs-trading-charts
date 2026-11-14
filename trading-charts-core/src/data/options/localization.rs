use {crate::data::format::DateFormat, crate::data::format::PriceFormatter, crate::data::format::TimeFormatter};
use serde::{Deserialize, Serialize};

/// Represents options for formatting dates, times, and prices according to a locale.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalizationOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    locale:                         Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_formatter:                Option<PriceFormatter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tickmarks_price_formatter:      Option<PriceFormatter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    percentage_formatter:           Option<PriceFormatter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tickmarks_percentage_formatter: Option<PriceFormatter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    time_formatter:                 Option<TimeFormatter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    date_format:                    Option<DateFormat>,
}

impl LocalizationOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Current locale used to format dates. Uses the browser's language settings by default.
    ///
    /// Default: `navigator.language`.
    pub fn with_locale(self, locale: impl Into<String>) -> Self {
        Self {
            locale: Some(locale.into()),
            ..self
        }
    }

    pub fn locale(&self) -> Option<&str> {
        self.locale.as_deref()
    }

    /// Set or reset (with `None`) the `locale` option.
    pub fn set_locale(&mut self, locale: Option<String>) {
        self.locale = locale;
    }

    /// Override formatting of the price scale tick marks, labels and crosshair labels.
    /// Can be used for cases that can't be covered with built-in price formats.
    ///
    /// Default: `undefined`.
    pub fn with_price_formatter(self, price_formatter: PriceFormatter) -> Self {
        Self {
            price_formatter: Some(price_formatter),
            ..self
        }
    }

    pub fn price_formatter(&self) -> Option<&PriceFormatter> {
        self.price_formatter.as_ref()
    }

    /// Set or reset (with `None`) the `priceFormatter` option.
    pub fn set_price_formatter(&mut self, price_formatter: Option<PriceFormatter>) {
        self.price_formatter = price_formatter;
    }

    /// Overrides the formatting of price scale tick marks. Use this to define formatting rules based on all provided
    /// price values.
    ///
    /// Default: `undefined`.
    pub fn with_tickmarks_price_formatter(self, tickmarks_price_formatter: PriceFormatter) -> Self {
        Self {
            tickmarks_price_formatter: Some(tickmarks_price_formatter),
            ..self
        }
    }

    pub fn tickmarks_price_formatter(&self) -> Option<&PriceFormatter> {
        self.tickmarks_price_formatter.as_ref()
    }

    /// Set or reset (with `None`) the `tickmarksPriceFormatter` option.
    pub fn set_tickmarks_price_formatter(&mut self, tickmarks_price_formatter: Option<PriceFormatter>) {
        self.tickmarks_price_formatter = tickmarks_price_formatter;
    }

    /// Overrides the formatting of percentage scale tick marks.
    ///
    /// Default: `undefined`.
    pub fn with_percentage_formatter(self, percentage_formatter: PriceFormatter) -> Self {
        Self {
            percentage_formatter: Some(percentage_formatter),
            ..self
        }
    }

    pub fn percentage_formatter(&self) -> Option<&PriceFormatter> {
        self.percentage_formatter.as_ref()
    }

    /// Set or reset (with `None`) the `percentageFormatter` option.
    pub fn set_percentage_formatter(&mut self, percentage_formatter: Option<PriceFormatter>) {
        self.percentage_formatter = percentage_formatter;
    }

    /// Override formatting of the percentage scale tick marks. Can be used if formatting should be adjusted based on
    /// all the values being formatted
    ///
    /// Default: `undefined`.
    pub fn with_tickmarks_percentage_formatter(self, tickmarks_percentage_formatter: PriceFormatter) -> Self {
        Self {
            tickmarks_percentage_formatter: Some(tickmarks_percentage_formatter),
            ..self
        }
    }

    pub fn tickmarks_percentage_formatter(&self) -> Option<&PriceFormatter> {
        self.tickmarks_percentage_formatter.as_ref()
    }

    /// Set or reset (with `None`) the `tickmarksPercentageFormatter` option.
    pub fn set_tickmarks_percentage_formatter(&mut self, tickmarks_percentage_formatter: Option<PriceFormatter>) {
        self.tickmarks_percentage_formatter = tickmarks_percentage_formatter;
    }

    /// Override formatting of the time scale crosshair label.
    ///
    /// Default: `undefined`.
    pub fn with_time_formatter(self, time_formatter: TimeFormatter) -> Self {
        Self {
            time_formatter: Some(time_formatter),
            ..self
        }
    }

    pub fn time_formatter(&self) -> Option<&TimeFormatter> {
        self.time_formatter.as_ref()
    }

    /// Set or reset (with `None`) the `timeFormatter` option.
    pub fn set_time_formatter(&mut self, time_formatter: Option<TimeFormatter>) {
        self.time_formatter = time_formatter;
    }

    /// Date formatting string.
    ///
    /// Can contain `yyyy`, `yy`, `MMMM`, `MMM`, `MM` and `dd` literals which will be replaced with corresponding date's
    /// value.
    ///
    /// Ignored if `timeFormatter` has been specified.
    ///
    /// Default: `"dd MMM \'yy"`.
    pub fn with_date_format(self, date_format: impl Into<DateFormat>) -> Self {
        Self {
            date_format: Some(date_format.into()),
            ..self
        }
    }

    pub fn date_format(&self) -> Option<&DateFormat> {
        self.date_format.as_ref()
    }

    /// Set or reset (with `None`) the `dateFormat` option.
    pub fn set_date_format(&mut self, date_format: Option<DateFormat>) {
        self.date_format = date_format;
    }
}
