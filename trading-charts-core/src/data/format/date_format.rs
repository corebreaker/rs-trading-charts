use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Format of the dates shown by Lightweight Charts, for instance in the crosshair label of the time scale.
///
/// The formats may contain the `yyyy`, `yy`, `MMMM`, `MMM`, `MM` and `dd` tokens, which are replaced by the year, the
/// two-digit year, the month name, the abbreviated month name, the month number and the day of month. Use a
/// [`TimeFormatter`](super::TimeFormatter) for more control.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum DateFormat {
    /// `dd MMM 'yy`, such as `14 Mar '24`: the default format of Lightweight Charts.
    #[default]
    Default,

    /// `yyyy-MM-dd`, such as `2024-03-14`.
    Iso,

    /// `dd/MM/yyyy`, such as `14/03/2024`.
    DayMonthYear,

    /// `MM/dd/yyyy`, such as `03/14/2024`.
    MonthDayYear,

    /// `dd.MM.yyyy`, such as `14.03.2024`.
    Dotted,

    /// `dd MMMM yyyy`, such as `14 March 2024`.
    Long,

    /// Any other format.
    Custom(String),
}

impl DateFormat {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Default => "dd MMM 'yy",
            Self::Iso => "yyyy-MM-dd",
            Self::DayMonthYear => "dd/MM/yyyy",
            Self::MonthDayYear => "MM/dd/yyyy",
            Self::Dotted => "dd.MM.yyyy",
            Self::Long => "dd MMMM yyyy",
            Self::Custom(format) => format,
        }
    }

    fn predefined() -> [Self; 6] {
        [
            Self::Default,
            Self::Iso,
            Self::DayMonthYear,
            Self::MonthDayYear,
            Self::Dotted,
            Self::Long,
        ]
    }
}

impl From<&str> for DateFormat {
    fn from(format: &str) -> Self {
        Self::predefined()
            .into_iter()
            .find(|predefined| predefined.as_str() == format)
            .unwrap_or_else(|| Self::Custom(format.to_string()))
    }
}

impl From<String> for DateFormat {
    fn from(format: String) -> Self {
        Self::from(format.as_str())
    }
}

impl Display for DateFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl Serialize for DateFormat {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for DateFormat {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(deserializer)?))
    }
}
