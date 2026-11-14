use super::LineSeriesOptions;
use crate::{
    data::{Marker, PriceLineOptions, SeriesMarkersOptions, ValueData},
    series::mount::{mount_series, SeriesInputs},
};

use leptos::{component, prelude::Signal, IntoView};

/// Add a line series to the enclosing `Chart`, in the enclosing `ChartPanel` if any.
///
/// The series follows the reactive `options`, `data`, `markers`, `markers_options` and `price_lines` inputs,
/// and is removed from the chart when the component is unmounted.
#[component(transparent)]
pub fn LineSeries(
    #[prop(optional, into)] options: Option<Signal<LineSeriesOptions>>,
    #[prop(into)] data: Signal<Vec<ValueData>>,
    #[prop(optional, into)] markers: Signal<Vec<Marker>>,
    #[prop(optional, into)] markers_options: Option<Signal<SeriesMarkersOptions>>,
    #[prop(optional, into)] price_lines: Signal<Vec<PriceLineOptions>>,
) -> impl IntoView {
    mount_series(
        "line",
        SeriesInputs {
            options,
            data,
            markers,
            markers_options,
            price_lines,
        },
    );
}
