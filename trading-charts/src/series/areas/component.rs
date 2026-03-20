use super::AreaSeriesOptions;
use crate::{
    ChartHandle, PanelId,
    data::{Marker, ValueData, series::Series},
};

use leptos::{
    reactive::{
        effect::Effect,
        traits::{Get, With},
        wrappers::read::Signal,
    },
    context::use_context,
    IntoView, component, view,
};

#[component(transparent)]
pub fn AreaSeries(
    #[prop(optional, into)] options: Option<Signal<AreaSeriesOptions>>,
    #[prop(into)] data: Signal<Vec<ValueData>>,
    #[prop(into)] markers: Signal<Vec<Marker>>,
) -> impl IntoView {
    let chart: Option<ChartHandle> = use_context();
    if let Some(chart) = chart {
        let series = {
            let mut series: Series<ValueData, AreaSeriesOptions> = Series::new("area");
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
        }
    }

    view!()
}
