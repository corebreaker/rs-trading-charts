use super::BarSeriesOptions;
use crate::{
    data::{Marker, PriceLineOptions, Candlestick},
    series::mount::use_series,
};

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct BarSeriesProps {
    data:        Vec<Candlestick>,
    #[props(default)]
    markers:     Vec<Marker>,
    #[props(default)]
    price_lines: Vec<PriceLineOptions>,
    #[props(default)]
    options:     Option<BarSeriesOptions>,
}

/// Add a bar (OHLC) series to the enclosing `Chart`, in the enclosing `ChartPanel` if any.
///
/// The series is updated when `options`, `data`, `markers` or `price_lines` change, and is removed from the chart
/// when the component is dropped.
#[allow(non_snake_case)]
pub fn BarSeries(props: BarSeriesProps) -> Element {
    use_series("bar", props.options, props.data, props.markers, props.price_lines);

    rsx! {}
}
