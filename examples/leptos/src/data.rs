//! Data of the demos: synthetic daily prices and volumes, and real intraday prices.

use charts::data::{Candlestick, OhlcvData, UTCTimestamp, ValueData};
use serde::Deserialize;

const DAY: f64 = 86_400.0;

/// Small deterministic pseudo-random generator, so that the demos show the same data on every load.
struct Random(u64);

impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);

        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }

    fn range(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.next()
    }
}

fn round(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

/// Daily prices and volumes, as a random walk starting at `start_price`.
pub fn daily_ohlcv(seed: u64, days: usize, start_price: f64) -> Vec<OhlcvData> {
    let mut random = Random(seed);
    // 2024-01-01T00:00:00Z
    let start = 1_704_067_200.0;
    let mut close = start_price;

    (0..days)
        .map(|day| {
            let open = close;
            let change = random.range(-0.035, 0.037) * open;

            close = (open + change).max(1.0);

            let high = open.max(close) * (1.0 + random.range(0.0, 0.02));
            let low = open.min(close) * (1.0 - random.range(0.0, 0.02));
            let volume = (random.range(0.5, 1.5) * 1_000_000.0 * (1.0 + change.abs() / open * 20.0)).round();

            OhlcvData::new(
                UTCTimestamp::from_seconds(start + day as f64 * DAY),
                round(open),
                round(high),
                round(low),
                round(close),
                volume,
            )
        })
        .collect()
}

pub fn candlesticks(data: &[OhlcvData]) -> Vec<Candlestick> {
    data.iter().map(OhlcvData::candlestick).collect()
}

pub fn closes(data: &[OhlcvData]) -> Vec<ValueData> {
    data.iter().map(|item| ValueData::new(item.time, item.close)).collect()
}

/// Simple moving average of the close prices; the first items are whitespace.
pub fn moving_average(data: &[OhlcvData], length: usize) -> Vec<ValueData> {
    data.iter()
        .enumerate()
        .map(|(index, item)| {
            if index + 1 < length {
                return ValueData::whitespace(item.time);
            }

            let sum: f64 = data[index + 1 - length..=index].iter().map(|item| item.close).sum();

            ValueData::new(item.time, round(sum / length as f64))
        })
        .collect()
}

#[derive(Deserialize)]
struct IntradayFile {
    data_up: Vec<Candlestick>,
}

/// Real 5-minute prices of a trading session (May 2025, US market hours).
pub fn intraday() -> Vec<Candlestick> {
    let mut data = serde_json::from_str::<IntradayFile>(include_str!("../data/data1.json"))
        .map(|file| file.data_up)
        .unwrap_or_default();

    data.sort_by_key(|item| item.time());
    data.dedup_by_key(|item| item.time());
    data
}

/// Relative strength index of the close prices; the first items are whitespace.
pub fn rsi(data: &[OhlcvData], length: usize) -> Vec<ValueData> {
    let mut gains = 0.0;
    let mut losses = 0.0;

    data.iter()
        .enumerate()
        .map(|(index, item)| {
            if index == 0 {
                return ValueData::whitespace(item.time);
            }

            let change = item.close - data[index - 1].close;
            let (gain, loss) = (change.max(0.0), (-change).max(0.0));

            if index <= length {
                gains += gain / length as f64;
                losses += loss / length as f64;
            } else {
                gains = (gains * (length - 1) as f64 + gain) / length as f64;
                losses = (losses * (length - 1) as f64 + loss) / length as f64;
            }

            if index < length {
                ValueData::whitespace(item.time)
            } else if losses == 0.0 {
                ValueData::new(item.time, 100.0)
            } else {
                ValueData::new(item.time, round(100.0 - 100.0 / (1.0 + gains / losses)))
            }
        })
        .collect()
}
