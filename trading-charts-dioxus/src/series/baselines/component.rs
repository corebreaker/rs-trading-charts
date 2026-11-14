use super::BaselineSeriesOptions;
use crate::{
    data::{Marker, PriceLineOptions, SeriesMarkersOptions, ValueData},
    series::mount::use_series,
};

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct BaselineSeriesProps {
    data:            Vec<ValueData>,
    #[props(default)]
    markers:         Vec<Marker>,
    #[props(default)]
    markers_options: Option<SeriesMarkersOptions>,
    #[props(default)]
    price_lines:     Vec<PriceLineOptions>,
    #[props(default)]
    options:         Option<BaselineSeriesOptions>,
}

/// Add a baseline series to the enclosing `Chart`, in the enclosing `ChartPanel` if any.
///
/// The series is updated when `options`, `data`, `markers`, `markers_options` or `price_lines` change,
/// and is removed from the chart when the component is dropped.
#[allow(non_snake_case)]
pub fn BaselineSeries(props: BaselineSeriesProps) -> Element {
    use_series(
        "baseline",
        props.options,
        props.data,
        props.markers,
        props.markers_options,
        props.price_lines,
    );

    rsx! {}
}
