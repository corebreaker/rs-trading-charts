use crate::JsError;
use chrono::{format::ParseErrorKind, NaiveDate, DateTime, Utc};

pub(super) fn parse_str(s: &str) -> Result<DateTime<Utc>, JsError> {
    match DateTime::parse_from_rfc3339(s) {
        Ok(dt) => Ok(dt.with_timezone(&Utc)),
        Err(err) => match err.kind() {
            ParseErrorKind::TooShort => {
                NaiveDate::parse_from_str(s, "%Y-%m-%d")
                    .map_err(|err| JsError::new_from_str(format!("Parse error, too short timestamp {s}: {err}")))
                    .and_then(|date| {
                        date.and_hms_opt(0, 0, 0)
                            .ok_or_else(|| JsError::new_from_str("Cannot convert date to date-time"))
                    })
                    .map(|naive| naive.and_utc())
            },

            _ => Err(JsError::new_from_str(format!("Parse error on timestamp {s}: {err}"))),
        }
    }
}