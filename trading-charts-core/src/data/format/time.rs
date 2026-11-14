use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

/// Predefined style of the dates and times of [`TimeFormatter::Intl`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DateTimeStyle {
    /// `Thursday, March 14, 2024` / `1:30:00 PM Coordinated Universal Time`.
    Full,

    /// `March 14, 2024` / `1:30:00 PM UTC`.
    Long,

    /// `Mar 14, 2024` / `1:30:00 PM`.
    Medium,

    /// `3/14/24` / `1:30 PM`.
    Short,
}

/// Formatter of the times of the crosshair label of the time scale, implemented by the JS binding.
///
/// # Patterns
///
/// The patterns may contain these tokens, other characters being copied as is, and text between single quotes being
/// copied without the quotes:
///
/// | Token  | Meaning                   | Token | Meaning                    |
/// |--------|---------------------------|-------|----------------------------|
/// | `yyyy` | year                      | `HH`  | hour, 00-23                |
/// | `yy`   | year, two digits          | `H`   | hour, 0-23                 |
/// | `MMMM` | month name                | `hh`  | hour, 01-12                |
/// | `MMM`  | abbreviated month name    | `h`   | hour, 1-12                 |
/// | `MM`   | month, 01-12              | `mm`  | minutes, 00-59             |
/// | `M`    | month, 1-12               | `ss`  | seconds, 00-59             |
/// | `dd`   | day of month, 01-31       | `SSS` | milliseconds, 000-999      |
/// | `d`    | day of month, 1-31        | `a`   | `AM` or `PM`               |
/// | `EEEE` | day of week name          | `EEE` | abbreviated day of week    |
///
/// The names of months and days are localized with the `locale` parameter, a BCP 47 language tag such as `en-US`;
/// the browser locale is used when it is not set.
///
/// The times are formatted in the time zone of the chart (see `ChartHandle::set_time_zone`), or in UTC.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "$timeFormatter", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum TimeFormatter {
    /// Format with a pattern, such as `yyyy-MM-dd HH:mm`.
    Pattern {
        pattern: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        locale:  Option<String>,
    },

    /// Localized format with predefined date and time styles. Without style, the medium date and the short time are
    /// shown.
    Intl {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        locale:     Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        date_style: Option<DateTimeStyle>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        time_style: Option<DateTimeStyle>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        hour12:     Option<bool>,
    },

    /// Function registered with [`register_time_formatter`](super::register_time_formatter).
    Custom { name: String },
}

impl TimeFormatter {
    pub fn pattern(pattern: impl Into<String>) -> Self {
        Self::Pattern {
            pattern: pattern.into(),
            locale:  None,
        }
    }

    pub fn pattern_in_locale(pattern: impl Into<String>, locale: impl Into<String>) -> Self {
        Self::Pattern {
            pattern: pattern.into(),
            locale:  Some(locale.into()),
        }
    }

    /// Localized format with the given date and time styles, in the browser locale.
    pub fn intl(date_style: Option<DateTimeStyle>, time_style: Option<DateTimeStyle>) -> Self {
        Self::Intl {
            locale: None,
            date_style,
            time_style,
            hour12: None,
        }
    }

    /// Function registered with [`register_time_formatter`](super::register_time_formatter) under `name`.
    pub fn custom(name: impl Into<String>) -> Self {
        Self::Custom {
            name: name.into()
        }
    }
}

/// Kind of the tick marks of the time scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TickMarkType {
    /// Start of a year.
    Year            = 0,

    /// Start of a month.
    Month           = 1,

    /// Start of a day.
    DayOfMonth      = 2,

    /// Time without seconds.
    Time            = 3,

    /// Time with seconds.
    TimeWithSeconds = 4,
}

impl TickMarkType {
    pub fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::Year),
            1 => Some(Self::Month),
            2 => Some(Self::DayOfMonth),
            3 => Some(Self::Time),
            4 => Some(Self::TimeWithSeconds),
            _ => None,
        }
    }
}

impl Serialize for TickMarkType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for TickMarkType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::from_index(u8::deserialize(deserializer)?).ok_or_else(|| Error::custom("invalid value for TickMarkType"))
    }
}

/// Formatter of the labels of the tick marks of the time scale, implemented by the JS binding.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "$tickMarkFormatter",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum TickMarkFormatter {
    /// One pattern per kind of tick mark, using the tokens of [`TimeFormatter`]. A kind without pattern keeps the
    /// default label of Lightweight Charts.
    Patterns {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        year:              Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        month:             Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        day_of_month:      Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        time:              Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        time_with_seconds: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        locale:            Option<String>,
    },

    /// Function registered with [`register_tick_mark_formatter`](super::register_tick_mark_formatter).
    Custom { name: String },
}

impl TickMarkFormatter {
    /// Patterns covering every kind of tick mark: `yyyy`, `MMM`, `d`, `HH:mm` and `HH:mm:ss`.
    pub fn patterns() -> Self {
        Self::Patterns {
            year:              Some(String::from("yyyy")),
            month:             Some(String::from("MMM")),
            day_of_month:      Some(String::from("d")),
            time:              Some(String::from("HH:mm")),
            time_with_seconds: Some(String::from("HH:mm:ss")),
            locale:            None,
        }
    }

    /// Set the pattern of a kind of tick mark of a `Patterns` formatter. A `Custom` formatter is returned unchanged.
    pub fn with_pattern(self, kind: TickMarkType, pattern: impl Into<String>) -> Self {
        match self {
            Self::Patterns {
                mut year,
                mut month,
                mut day_of_month,
                mut time,
                mut time_with_seconds,
                locale,
            } => {
                let pattern = Some(pattern.into());

                match kind {
                    TickMarkType::Year => year = pattern,
                    TickMarkType::Month => month = pattern,
                    TickMarkType::DayOfMonth => day_of_month = pattern,
                    TickMarkType::Time => time = pattern,
                    TickMarkType::TimeWithSeconds => time_with_seconds = pattern,
                }

                Self::Patterns {
                    year,
                    month,
                    day_of_month,
                    time,
                    time_with_seconds,
                    locale,
                }
            }
            other => other,
        }
    }

    /// Set the locale of a `Patterns` formatter. A `Custom` formatter is returned unchanged.
    pub fn with_locale(self, locale: impl Into<String>) -> Self {
        match self {
            Self::Patterns {
                year,
                month,
                day_of_month,
                time,
                time_with_seconds,
                ..
            } => Self::Patterns {
                year,
                month,
                day_of_month,
                time,
                time_with_seconds,
                locale: Some(locale.into()),
            },
            other => other,
        }
    }

    /// Function registered with [`register_tick_mark_formatter`](super::register_tick_mark_formatter) under `name`.
    pub fn custom(name: impl Into<String>) -> Self {
        Self::Custom {
            name: name.into()
        }
    }
}
