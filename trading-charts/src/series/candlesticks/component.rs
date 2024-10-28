use super::CandlestickOptions;
use crate::{
    bindings::TradingChartBinding,
    data::{series::Series, Candlestick, Marker},
};

use leptos::{
    reactive::{
        signal::ReadSignal,
        traits::{Get, With},
        effect::Effect,
    },
    context::use_context,
    IntoView,
    component,
    view,
};

#[component(transparent)]
pub fn CandleStickSeries(
    #[prop(optional)] options: Option<ReadSignal<CandlestickOptions>>,
    data: ReadSignal<Vec<Candlestick>>,
    markers: ReadSignal<Vec<Marker>>,
) -> impl IntoView {
    let chart: Option<TradingChartBinding> = use_context();
    if let Some(chart) = chart {
        let series = {
            let mut series: Series<Candlestick, CandlestickOptions> = Series::new("candlestick");
            if let Some(options) = &options {
                series.set_options(options.get());
            }

            if let Err(err) = chart.add_series(&mut series) {
                err.log();

                return view! {};
            }

            series
        };

        if let Some(id) = series.id() {
            if let Some(options) = options {
                let id = id.clone();
                let chart = chart.clone();

                let _ = Effect::new(move || {
                    if let Err(err) = options.with(|options| chart.update_series_options(id.clone(), options)) {
                        err.log();
                    }
                });
            }

            let _ = Effect::new({
                let id = id.clone();
                let chart = chart.clone();

                move || {
                    if let Err(err) = data.with(|data| chart.update_data(id.clone(), data)) {
                        err.log();
                    }
                }
            });

            let _ = Effect::new({
                let id = id.clone();
                let chart = chart.clone();

                move || {
                    if let Err(err) = markers.with(|markers| chart.set_markers(id.clone(), markers)) {
                        err.log();
                    }
                }
            });
        }
    }

    view! {}
}
