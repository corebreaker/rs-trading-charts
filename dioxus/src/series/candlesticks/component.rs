use super::CandlestickOptions;
use crate::{
    data::{Marker, PriceLineOptions, SeriesMarkersOptions, Candlestick},
    series::mount::use_series,
};

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CandleStickSeriesProps {
    data:            Vec<Candlestick>,
    #[props(default)]
    markers:         Vec<Marker>,
    #[props(default)]
    markers_options: Option<SeriesMarkersOptions>,
    #[props(default)]
    price_lines:     Vec<PriceLineOptions>,
    #[props(default)]
    options:         Option<CandlestickOptions>,
}

/// Add a candlestick series to the enclosing `Chart`, in the enclosing `ChartPanel` if any.
///
/// The series is updated when `options`, `data`, `markers`, `markers_options` or `price_lines` change,
/// and is removed from the chart when the component is dropped.
#[allow(non_snake_case)]
pub fn CandleStickSeries(props: CandleStickSeriesProps) -> Element {
    use_series(
        "candlestick",
        props.options,
        props.data,
        props.markers,
        props.markers_options,
        props.price_lines,
    );

    rsx! {}
}
