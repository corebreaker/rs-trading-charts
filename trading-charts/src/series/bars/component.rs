use super::BarSeriesOptions;
use crate::{
    data::{Marker, PriceLineOptions, SeriesMarkersOptions, Candlestick},
    series::mount::{mount_series, SeriesInputs},
};

use leptos::{component, prelude::Signal, IntoView};

/// Add a bar (OHLC) series to the enclosing `Chart`, in the enclosing `ChartPanel` if any.
///
/// The series follows the reactive `options`, `data`, `markers`, `markers_options` and `price_lines` inputs,
/// and is removed from the chart when the component is unmounted.
#[component(transparent)]
pub fn BarSeries(
    #[prop(optional, into)] options: Option<Signal<BarSeriesOptions>>,
    #[prop(into)] data: Signal<Vec<Candlestick>>,
    #[prop(optional, into)] markers: Signal<Vec<Marker>>,
    #[prop(optional, into)] markers_options: Option<Signal<SeriesMarkersOptions>>,
    #[prop(optional, into)] price_lines: Signal<Vec<PriceLineOptions>>,
) -> impl IntoView {
    mount_series(
        "bar",
        SeriesInputs {
            options,
            data,
            markers,
            markers_options,
            price_lines,
        },
    );
}
