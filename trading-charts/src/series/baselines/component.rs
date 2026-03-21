use super::BaselineSeriesOptions;
use crate::{
    ChartHandle,
    PanelId,
    data::{Marker, PriceLineOptions, ValueData, series::Series},
};

use leptos::{
    IntoView,
    component,
    context::use_context,
    reactive::{
        effect::Effect,
        traits::{Get, With},
        wrappers::read::Signal,
    },
    view,
};

#[component(transparent)]
pub fn BaselineSeries(
    #[prop(optional, into)] options: Option<Signal<BaselineSeriesOptions>>,
    #[prop(into)] data: Signal<Vec<ValueData>>,
    #[prop(into)] markers: Signal<Vec<Marker>>,
    #[prop(optional, into)] price_lines: Signal<Vec<PriceLineOptions>>,
) -> impl IntoView {
    let chart: Option<ChartHandle> = use_context();
    if let Some(chart) = chart {
        let series = {
            let mut series: Series<ValueData, BaselineSeriesOptions> = Series::new("baseline");
            if let Some(panel_id) = use_context::<PanelId>() {
                series.set_panel(panel_id);
            }
            if let Some(options) = &options {
                series.set_options(options.get());
            }

            if let Err(err) = chart.add_series(&mut series) {
                err.with_prefix("Failed to add series").log();

                return view!();
            }

            series
        };

        if let Some(id) = series.id() {
            if let Some(options) = options {
                let id = id.clone();
                let chart = chart.clone();

                let _ = Effect::new(move || {
                    let res = options.with(|options| {
                        chart
                            .update_series_options(id.clone(), options)
                            .map_err(|err| err.with_serializable_data(options))
                    });

                    if let Err(err) = res {
                        err.with_prefix("Failed to update series options").log();
                    }
                });
            }

            let _ = Effect::new({
                let id = id.clone();
                let chart = chart.clone();

                move || {
                    let res = data.with(|data| {
                        chart
                            .update_data(id.clone(), data)
                            .map_err(|err| err.with_serializable_data(data))
                    });

                    if let Err(err) = res {
                        err.with_prefix("Failed to update data").log();
                    }
                }
            });

            let _ = Effect::new({
                let id = id.clone();
                let chart = chart.clone();

                move || {
                    let res = markers.with(|markers| {
                        chart
                            .set_markers(id.clone(), markers)
                            .map_err(|err| err.with_serializable_data(markers))
                    });

                    if let Err(err) = res {
                        err.with_prefix("Failed to set markers").log();
                    }
                }
            });

            let _ = Effect::new({
                let id = id.clone();
                let chart = chart.clone();

                move || {
                    let res = price_lines.with(|price_lines| {
                        chart
                            .set_price_lines(id.clone(), price_lines)
                            .map_err(|err| err.with_serializable_data(price_lines))
                    });

                    if let Err(err) = res {
                        err.with_prefix("Failed to set price lines").log();
                    }
                }
            });
        }
    }

    view!()
}
