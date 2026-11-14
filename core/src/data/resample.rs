use super::{Candlestick, OhlcvData, UTCTimestamp, ValueData};
use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};

/// Interval of the bars of a series, used to resample data to a larger interval.
///
/// The intervals are aligned in UTC: on the minute, the hour or the day for the intraday intervals, on Mondays for
/// the weeks, on the first day of the month for the months, and on January 1st for the years. Multiples are aligned
/// on the Unix epoch for the intraday intervals and the days, and on the year 0 for the months and years.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Interval {
    Minutes(u32),
    Hours(u32),
    Days(u32),
    Weeks(u32),
    Months(u32),
    Years(u32),
}

impl Interval {
    /// Start of the interval containing a time.
    pub fn start(&self, time: UTCTimestamp) -> UTCTimestamp {
        let datetime = time.into_datetime();
        let fixed = |seconds: i64| {
            let seconds = seconds.max(1);
            let timestamp = datetime.timestamp();

            UTCTimestamp::from_seconds((timestamp - timestamp.rem_euclid(seconds)) as f64)
        };
        let date = |date: NaiveDate| {
            UTCTimestamp::from_datetime(DateTime::<Utc>::from_naive_utc_and_offset(
                date.and_hms_opt(0, 0, 0).unwrap_or_default(),
                Utc,
            ))
        };

        match *self {
            Self::Minutes(count) => fixed(60 * count as i64),
            Self::Hours(count) => fixed(3_600 * count as i64),
            Self::Days(count) => fixed(86_400 * count as i64),
            Self::Weeks(count) => {
                let day = datetime.date_naive();
                let monday = day - Duration::days(day.weekday().num_days_from_monday() as i64);
                // 1970-01-05 is a Monday.
                let epoch_monday = NaiveDate::from_ymd_opt(1970, 1, 5).unwrap_or_default();
                let weeks = (monday - epoch_monday).num_days().div_euclid(7);
                let count = count.max(1) as i64;

                date(epoch_monday + Duration::weeks(weeks - weeks.rem_euclid(count)))
            }
            Self::Months(count) => {
                let months = datetime.year() as i64 * 12 + datetime.month0() as i64;
                let months = months - months.rem_euclid(count.max(1) as i64);

                date(NaiveDate::from_ymd_opt((months / 12) as i32, (months % 12) as u32 + 1, 1).unwrap_or_default())
            }
            Self::Years(count) => {
                let year = datetime.year();
                let year = year - year.rem_euclid(count.max(1) as i32);

                date(NaiveDate::from_ymd_opt(year, 1, 1).unwrap_or_default())
            }
        }
    }
}

/// How the values of an interval are combined by [`resample_values`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Aggregation {
    First,
    #[default]
    Last,
    Min,
    Max,
    Sum,
    Mean,
}

/// Group sorted items by interval, calling `merge` with the start of each interval and its items.
fn group<T, R>(
    items: &[T],
    interval: Interval,
    time: impl Fn(&T) -> UTCTimestamp,
    mut merge: impl FnMut(UTCTimestamp, &[T]) -> Option<R>,
) -> Vec<R> {
    let mut result = Vec::new();
    let mut begin = 0;

    while begin < items.len() {
        let start = interval.start(time(&items[begin]));
        let mut end = begin + 1;

        while end < items.len() && interval.start(time(&items[end])) == start {
            end += 1;
        }

        if let Some(item) = merge(start, &items[begin..end]) {
            result.push(item);
        }

        begin = end;
    }

    result
}

/// Resample OHLC items to a larger interval: each interval gets the open of its first item, the highest high, the
/// lowest low and the close of its last item. Whitespace items are ignored.
///
/// The items must be sorted by time. The resampled items are timed at the start of their interval.
pub fn resample_ohlc(items: &[Candlestick], interval: Interval) -> Vec<Candlestick> {
    let items: Vec<&Candlestick> = items.iter().filter(|item| !item.is_whitespace()).collect();

    group(
        &items,
        interval,
        |item| item.time(),
        |start, items| {
            let first = items.first()?;
            let last = items.last()?;
            let high = items.iter().filter_map(|item| item.high()).fold(f64::MIN, f64::max);
            let low = items.iter().filter_map(|item| item.low()).fold(f64::MAX, f64::min);

            Some(Candlestick::new(start, first.open()?, high, low, last.close()?))
        },
    )
}

/// Resample OHLCV items to a larger interval, like [`resample_ohlc`], the volumes of an interval being summed.
///
/// The items must be sorted by time. The resampled items are timed at the start of their interval.
pub fn resample_ohlcv(items: &[OhlcvData], interval: Interval) -> Vec<OhlcvData> {
    group(
        items,
        interval,
        |item| item.time,
        |start, items| {
            let first = items.first()?;
            let last = items.last()?;

            Some(OhlcvData::new(
                start,
                first.open,
                items.iter().map(|item| item.high).fold(f64::MIN, f64::max),
                items.iter().map(|item| item.low).fold(f64::MAX, f64::min),
                last.close,
                items.iter().map(|item| item.volume).sum(),
            ))
        },
    )
}

/// Resample single-value items to a larger interval, combining the values of each interval with `aggregation`.
/// Whitespace items are ignored.
///
/// The items must be sorted by time. The resampled items are timed at the start of their interval.
pub fn resample_values(items: &[ValueData], interval: Interval, aggregation: Aggregation) -> Vec<ValueData> {
    let items: Vec<(UTCTimestamp, f64)> = items
        .iter()
        .filter_map(|item| item.value().map(|value| (item.time(), value)))
        .collect();

    group(
        &items,
        interval,
        |item| item.0,
        |start, items| {
            let values = items.iter().map(|item| item.1);
            let value = match aggregation {
                Aggregation::First => items.first()?.1,
                Aggregation::Last => items.last()?.1,
                Aggregation::Min => values.fold(f64::MAX, f64::min),
                Aggregation::Max => values.fold(f64::MIN, f64::max),
                Aggregation::Sum => values.sum(),
                Aggregation::Mean => values.sum::<f64>() / items.len() as f64,
            };

            Some(ValueData::new(start, value))
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn time(s: &str) -> UTCTimestamp {
        s.parse().unwrap()
    }

    #[test]
    fn interval_start() {
        let t = time("2024-03-14T13:47:12Z");

        assert_eq!(Interval::Minutes(15).start(t), time("2024-03-14T13:45:00Z"));
        assert_eq!(Interval::Hours(4).start(t), time("2024-03-14T12:00:00Z"));
        assert_eq!(Interval::Days(1).start(t), time("2024-03-14"));
        assert_eq!(Interval::Weeks(1).start(t), time("2024-03-11"));
        assert_eq!(Interval::Months(1).start(t), time("2024-03-01"));
        assert_eq!(Interval::Months(3).start(t), time("2024-01-01"));
        assert_eq!(Interval::Years(1).start(t), time("2024-01-01"));
    }

    #[test]
    fn ohlc() {
        let items = vec![
            Candlestick::new(time("2024-03-11"), 10.0, 12.0, 9.0, 11.0),
            Candlestick::new(time("2024-03-12"), 11.0, 15.0, 10.0, 14.0),
            Candlestick::whitespace(time("2024-03-13")),
            Candlestick::new(time("2024-03-18"), 14.0, 14.5, 8.0, 9.0),
        ];
        let weeks = resample_ohlc(&items, Interval::Weeks(1));

        assert_eq!(
            weeks,
            vec![
                Candlestick::new(time("2024-03-11"), 10.0, 15.0, 9.0, 14.0),
                Candlestick::new(time("2024-03-18"), 14.0, 14.5, 8.0, 9.0),
            ]
        );
    }

    #[test]
    fn ohlcv() {
        let items = vec![
            OhlcvData::new(time("2024-01-31"), 1.0, 2.0, 0.5, 1.5, 100.0),
            OhlcvData::new(time("2024-02-01"), 1.5, 3.0, 1.0, 2.5, 200.0),
            OhlcvData::new(time("2024-02-29"), 2.5, 2.6, 2.0, 2.1, 50.0),
        ];
        let months = resample_ohlcv(&items, Interval::Months(1));

        assert_eq!(months.len(), 2);
        assert_eq!(months[1], OhlcvData::new(time("2024-02-01"), 1.5, 3.0, 1.0, 2.1, 250.0));
    }

    #[test]
    fn values() {
        let items = vec![
            ValueData::new(time("2024-03-14T10:00:00Z"), 1.0),
            ValueData::new(time("2024-03-14T11:00:00Z"), 3.0),
            ValueData::whitespace(time("2024-03-14T12:00:00Z")),
            ValueData::new(time("2024-03-15T10:00:00Z"), 5.0),
        ];

        let mean = resample_values(&items, Interval::Days(1), Aggregation::Mean);
        assert_eq!(
            mean,
            vec![
                ValueData::new(time("2024-03-14"), 2.0),
                ValueData::new(time("2024-03-15"), 5.0)
            ]
        );

        let sum = resample_values(&items, Interval::Days(1), Aggregation::Sum);
        assert_eq!(sum[0].value(), Some(4.0));
    }
}
