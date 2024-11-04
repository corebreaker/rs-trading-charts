use super::parse_str::parse_str;
use crate::JsError;
use serde::{de::Error as SerdeError, Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, Utc, SecondsFormat};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UTCTimestamp {
    timestamp: i64,
}

impl UTCTimestamp {
    pub fn now() -> Self {
        Self {
            timestamp: Utc::now().timestamp(),
        }
    }

    pub fn from_datetime(datetime: DateTime<Utc>) -> Self {
        Self {
            timestamp: datetime.timestamp(),
        }
    }

    pub fn from_value(timestamp: i64) -> Option<Self> {
        let dt = DateTime::<Utc>::from_timestamp(timestamp, 0)?;

        Some(Self::from_datetime(dt))
    }

    pub fn into_datetime(self) -> DateTime<Utc> {
        DateTime::<Utc>::from_timestamp(self.timestamp, 0).unwrap()
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl FromStr for UTCTimestamp {
    type Err = JsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dt = parse_str(s.as_ref())?;

        Ok(Self::from_datetime(dt))
    }
}

impl Display for UTCTimestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let dt = self.into_datetime().to_rfc3339_opts(SecondsFormat::Secs, true);

        <String as Display>::fmt(&dt, f)
    }
}

impl From<DateTime<Utc>> for UTCTimestamp {
    fn from(datetime: DateTime<Utc>) -> Self {
        Self::from_datetime(datetime)
    }
}

impl Into<DateTime<Utc>> for UTCTimestamp {
    fn into(self) -> DateTime<Utc> {
        self.into_datetime()
    }
}

impl Default for UTCTimestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl Serialize for UTCTimestamp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self.timestamp)
    }
}

impl<'de> Deserialize<'de> for UTCTimestamp {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let timestamp = i64::deserialize(deserializer)?;
        if DateTime::<Utc>::from_timestamp(timestamp, 0).is_none() {
            return Err(SerdeError::custom(format!("invalid timestamp: {timestamp}")));
        }

        Ok(Self {
            timestamp,
        })
    }
}
