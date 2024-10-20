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

            chart.add_series(&mut series).unwrap();
            series
        };

        if let Some(id) = series.id() {
            if let Some(options) = options {
                let id = id.clone();
                let chart = chart.clone();

                let _ = Effect::new(move || {
                    options
                        .with(|options| chart.update_series_options(id.clone(), options))
                        .unwrap();
                });
            }

            let _ = Effect::new({
                let id = id.clone();
                let chart = chart.clone();

                move || {
                    data.with(|data| chart.update_data(id.clone(), data)).unwrap();
                }
            });

            let _ = Effect::new({
                let id = id.clone();
                let chart = chart.clone();

                move || {
                    markers.with(|markers| chart.set_markers(id.clone(), markers)).unwrap();
                }
            });
        }
    }

    view! {}
}
