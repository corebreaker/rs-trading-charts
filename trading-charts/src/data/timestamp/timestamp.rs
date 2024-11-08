use super::parse_str::parse_str;
use crate::JsError;
use serde::{de::{Visitor, Error as SerdeError}, Deserialize, Deserializer, Serialize, Serializer};
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
        struct TimestampVisitor;
        impl Visitor<'_> for TimestampVisitor {
            type Value = UTCTimestamp;

            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                formatter.write_str("a valid timestamp from a number or a string")
            }

            fn visit_i8<E: SerdeError>(self, value: i8) -> Result<Self::Value, E> {
                self.visit_i64(value as i64)
            }

            fn visit_i16<E: SerdeError>(self, value: i16) -> Result<Self::Value, E> {
                self.visit_i64(value as i64)
            }

            fn visit_i32<E: SerdeError>(self, value: i32) -> Result<Self::Value, E> {
                self.visit_i64(value as i64)
            }

            fn visit_i64<E: SerdeError>(self, value: i64) -> Result<Self::Value, E> {
                match DateTime::<Utc>::from_timestamp(value, 0) {
                    Some(ts) => Ok(UTCTimestamp::from_datetime(ts)),
                    None => Err(SerdeError::custom(format!("invalid timestamp: {value}"))),
                }
            }

            fn visit_i128<E: SerdeError>(self, value: i128) -> Result<Self::Value, E> {
                if value.unsigned_abs().leading_zeros() < 64 {
                    return Err(SerdeError::custom(format!("timestamp overflow: {value}")))
                }

                self.visit_i64(value as i64)
            }

            fn visit_u8<E: SerdeError>(self, value: u8) -> Result<Self::Value, E> {
                self.visit_i64(value as i64)
            }

            fn visit_u16<E: SerdeError>(self, value: u16) -> Result<Self::Value, E> {
                self.visit_i64(value as i64)
            }

            fn visit_u32<E: SerdeError>(self, value: u32) -> Result<Self::Value, E> {
                self.visit_i64(value as i64)
            }

            fn visit_u64<E: SerdeError>(self, value: u64) -> Result<Self::Value, E> {
                self.visit_i64(value as i64)
            }

            fn visit_u128<E: SerdeError>(self, value: u128) -> Result<Self::Value, E> {
                self.visit_i128(value as i128)
            }

            fn visit_f32<E: SerdeError>(self, value: f32) -> Result<Self::Value, E> {
                self.visit_f64(value as f64)
            }

            fn visit_f64<E: SerdeError>(self, value: f64) -> Result<Self::Value, E> {
                match DateTime::<Utc>::from_timestamp(value as i64, (value.fract() * 1e9) as u32) {
                    Some(ts) => Ok(UTCTimestamp::from_datetime(ts)),
                    None => Err(SerdeError::custom(format!("invalid timestamp: {value}"))),
                }
            }

            fn visit_str<E: SerdeError>(self, value: &str) -> Result<Self::Value, E> {
                match parse_str(value) {
                    Ok(ts) => Ok(UTCTimestamp::from_datetime(ts)),
                    Err(err) => Err(SerdeError::custom(format!("invalid timestamp: {err}"))),
                }
            }

            fn visit_string<E: SerdeError>(self, v: String) -> Result<Self::Value, E> {
                self.visit_str(&v)
            }
        }

        deserializer.deserialize_any(TimestampVisitor)
    }
}
