use super::AreaSeriesOptions;
use crate::{
    data::{Marker, PriceLineOptions, ValueData},
    series::mount::use_series,
};

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct AreaSeriesProps {
    data:        Vec<ValueData>,
    #[props(default)]
    markers:     Vec<Marker>,
    #[props(default)]
    price_lines: Vec<PriceLineOptions>,
    #[props(default)]
    options:     Option<AreaSeriesOptions>,
}

/// Add an area series to the enclosing `Chart`, in the enclosing `ChartPanel` if any.
///
/// The series is updated when `options`, `data`, `markers` or `price_lines` change, and is removed from the chart
/// when the component is dropped.
#[allow(non_snake_case)]
pub fn AreaSeries(props: AreaSeriesProps) -> Element {
    use_series("area", props.options, props.data, props.markers, props.price_lines);

    rsx! {}
}
