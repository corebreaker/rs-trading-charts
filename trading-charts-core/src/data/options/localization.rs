use serde::{Deserialize, Serialize};

/// Represents options for formatting dates, times, and prices according to a locale.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalizationOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    locale:      Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    date_format: Option<String>,
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

    /// Date formatting string.
    ///
    /// Can contain `yyyy`, `yy`, `MMMM`, `MMM`, `MM` and `dd` literals which will be replaced with corresponding date's
    /// value.
    ///
    /// Ignored if `timeFormatter` has been specified.
    ///
    /// Default: `"dd MMM \'yy"`.
    pub fn with_date_format(self, date_format: impl Into<String>) -> Self {
        Self {
            date_format: Some(date_format.into()),
            ..self
        }
    }

    pub fn date_format(&self) -> Option<&str> {
        self.date_format.as_deref()
    }

    /// Set or reset (with `None`) the `dateFormat` option.
    pub fn set_date_format(&mut self, date_format: Option<String>) {
        self.date_format = date_format;
    }
}
