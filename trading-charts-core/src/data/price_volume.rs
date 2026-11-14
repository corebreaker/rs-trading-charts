use super::{
    options::{PriceFormatOptions, PriceScaleMargins, PriceScaleOptions},
    Candlestick,
    HistogramData,
    UTCTimestamp,
    ValueData,
};
use crate::series::{
    areas::AreaSeriesOptions,
    bars::BarSeriesOptions,
    candlesticks::CandlestickOptions,
    histograms::HistogramSeriesOptions,
    lines::LineSeriesOptions,
};
use serde::{Deserialize, Serialize};

/// Identifier of the overlay price scale of the volume series of a price and volume chart.
pub const VOLUME_PRICE_SCALE_ID: &str = "volume";

/// Data item with the prices and the volume of a period.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct OhlcvData {
    pub time:   UTCTimestamp,
    pub open:   f64,
    pub high:   f64,
    pub low:    f64,
    pub close:  f64,
    pub volume: f64,
}

impl OhlcvData {
    pub fn new(time: UTCTimestamp, open: f64, high: f64, low: f64, close: f64, volume: f64) -> Self {
        Self {
            time,
            open,
            high,
            low,
            close,
            volume,
        }
    }

    /// Return `true` when the close price is greater than or equal to the open price.
    pub fn is_up(&self) -> bool {
        self.close >= self.open
    }

    /// The OHLC part of the item.
    pub fn candlestick(&self) -> Candlestick {
        Candlestick::new(self.time, self.open, self.high, self.low, self.close)
    }
}

/// Style of the price series of a price and volume chart, with its options.
///
/// The OHLC styles (candlestick and bar) show the four prices, the single-value styles (line and area) show the close
/// price.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PriceSeriesStyle {
    Candlestick(CandlestickOptions),
    Bar(BarSeriesOptions),
    Line(LineSeriesOptions),
    Area(AreaSeriesOptions),
}

impl Default for PriceSeriesStyle {
    fn default() -> Self {
        Self::Candlestick(CandlestickOptions::default())
    }
}

impl PriceSeriesStyle {
    /// Lightweight Charts type of the price series.
    pub fn series_type(&self) -> &'static str {
        match self {
            Self::Candlestick(_) => "candlestick",
            Self::Bar(_) => "bar",
            Self::Line(_) => "line",
            Self::Area(_) => "area",
        }
    }

    /// Return `true` for the styles showing the four prices.
    pub fn is_ohlc(&self) -> bool {
        matches!(self, Self::Candlestick(_) | Self::Bar(_))
    }
}

/// Data item of the price series of a price and volume chart: OHLC prices or a single value, depending on the
/// [`PriceSeriesStyle`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PriceData {
    Ohlc(Candlestick),
    Value(ValueData),
}

/// Colors of the volume columns, depending on the direction of the period.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VolumeColors {
    pub up:   String,
    pub down: String,
}

impl Default for VolumeColors {
    fn default() -> Self {
        Self {
            up:   String::from("rgba(38, 166, 154, 0.5)"),
            down: String::from("rgba(239, 83, 80, 0.5)"),
        }
    }
}

/// Helpers to build a chart showing prices and volumes, with the volumes as a histogram at the bottom of the pane of
/// the prices, on an overlay price scale.
pub struct PriceVolume;

impl PriceVolume {
    /// Data of the price series for a style.
    pub fn price_data(data: &[OhlcvData], style: &PriceSeriesStyle) -> Vec<PriceData> {
        data.iter()
            .map(|item| {
                if style.is_ohlc() {
                    PriceData::Ohlc(item.candlestick())
                } else {
                    PriceData::Value(ValueData::new(item.time, item.close))
                }
            })
            .collect()
    }

    /// Data of the volume series, colored by direction of the periods.
    pub fn volume_data(data: &[OhlcvData], colors: &VolumeColors) -> Vec<HistogramData> {
        data.iter()
            .map(|item| {
                let color = if item.is_up() { &colors.up } else { &colors.down };

                HistogramData::new(item.time, item.volume).with_color(color.as_str())
            })
            .collect()
    }

    /// Options of the volume series, on the volume overlay price scale.
    pub fn volume_options(options: Option<HistogramSeriesOptions>) -> HistogramSeriesOptions {
        let options = options.unwrap_or_default();
        let options = match options.price_format() {
            Some(_) => options,
            None => options.with_price_format(PriceFormatOptions::volume()),
        };

        let last_value_visible = options.last_value_visible().unwrap_or(false);
        let price_line_visible = options.price_line_visible().unwrap_or(false);

        options
            .with_price_scale_id(VOLUME_PRICE_SCALE_ID)
            .with_last_value_visible(last_value_visible)
            .with_price_line_visible(price_line_visible)
    }

    /// Margins of the price scales: the volumes take `volume_height` (a ratio of the pane height, from 0 to 1) at the
    /// bottom of the pane, and the prices take the rest, with a small top margin.
    ///
    /// Returns the options of the price scale of the prices, then of the volumes.
    pub fn price_scale_options(volume_height: f64) -> (PriceScaleOptions, PriceScaleOptions) {
        let volume_height = volume_height.clamp(0.0, 0.9);
        let margins = |top: f64, bottom: f64| PriceScaleMargins::new().with_top(top).with_bottom(bottom);

        (
            PriceScaleOptions::new().with_scale_margins(margins(0.1, volume_height)),
            PriceScaleOptions::new().with_scale_margins(margins(1.0 - volume_height, 0.0)),
        )
    }
}
