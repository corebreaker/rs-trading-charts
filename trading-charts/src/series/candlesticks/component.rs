use super::CandlestickOptions;
use crate::{
    data::{Marker, PriceLineOptions, Candlestick},
    series::mount::{mount_series, SeriesInputs},
};

use leptos::{component, prelude::Signal, view, IntoView};

/// Add a candlestick series to the enclosing `Chart`, in the enclosing `ChartPanel` if any.
///
/// The series follows the reactive `options`, `data`, `markers` and `price_lines` inputs, and is removed from the
/// chart when the component is unmounted.
#[component(transparent)]
pub fn CandleStickSeries(
    #[prop(optional, into)] options: Option<Signal<CandlestickOptions>>,
    #[prop(into)] data: Signal<Vec<Candlestick>>,
    #[prop(optional, into)] markers: Signal<Vec<Marker>>,
    #[prop(optional, into)] price_lines: Signal<Vec<PriceLineOptions>>,
) -> impl IntoView {
    mount_series(
        "candlestick",
        SeriesInputs {
            options,
            data,
            markers,
            price_lines,
        },
    );

    view!()
}
