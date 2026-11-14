use super::HistogramSeriesOptions;
use crate::{
    data::{Marker, PriceLineOptions, HistogramData},
    series::mount::use_series,
};

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct HistogramSeriesProps {
    data:        Vec<HistogramData>,
    #[props(default)]
    markers:     Vec<Marker>,
    #[props(default)]
    price_lines: Vec<PriceLineOptions>,
    #[props(default)]
    options:     Option<HistogramSeriesOptions>,
}

/// Add a histogram series to the enclosing `Chart`, in the enclosing `ChartPanel` if any.
///
/// The series is updated when `options`, `data`, `markers` or `price_lines` change, and is removed from the chart
/// when the component is dropped.
#[allow(non_snake_case)]
pub fn HistogramSeries(props: HistogramSeriesProps) -> Element {
    use_series("histogram", props.options, props.data, props.markers, props.price_lines);

    rsx! {}
}
