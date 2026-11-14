use crate::{
    data::{series::Series, Marker, PriceLineOptions},
    ChartHandle,
    PanelId,
    SeriesHandle,
};

use leptos::prelude::{on_cleanup, use_context, Effect, GetUntracked, Signal, With};
use serde::Serialize;

/// Reactive inputs of a series component.
pub(crate) struct SeriesInputs<Dat, Opt>
where
    Dat: Send + Sync + 'static,
    Opt: Send + Sync + 'static, {
    pub(crate) options:     Option<Signal<Opt>>,
    pub(crate) data:        Signal<Vec<Dat>>,
    pub(crate) markers:     Signal<Vec<Marker>>,
    pub(crate) price_lines: Signal<Vec<PriceLineOptions>>,
}

/// Add a series to the chart of the context, in the pane of the context, and keep it in sync with the inputs.
///
/// The series is removed from the chart when the component is cleaned up. Returns `None` when the component is not
/// rendered inside a `Chart` or when the series cannot be added.
pub(crate) fn mount_series<Dat, Opt>(
    series_type: &'static str,
    inputs: SeriesInputs<Dat, Opt>,
) -> Option<SeriesHandle<Dat, Opt>>
where
    Dat: Serialize + Clone + Send + Sync + 'static,
    Opt: Serialize + Clone + Send + Sync + 'static, {
    let chart = use_context::<ChartHandle>()?;
    let SeriesInputs {
        options,
        data,
        markers,
        price_lines,
    } = inputs;

    let mut series: Series<Dat, Opt> = Series::new(series_type);
    if let Some(panel_id) = use_context::<PanelId>() {
        series.set_panel(panel_id);
    }
    if let Some(options) = &options {
        series.set_options(options.get_untracked());
    }

    let handle = match chart.add_series(&series) {
        Ok(handle) => handle,
        Err(err) => {
            err.with_prefix("Failed to add series").log();

            return None;
        }
    };

    if let Some(options) = options {
        let handle = handle.clone();

        Effect::new(move || {
            options.with(|options| {
                if let Err(err) = handle.apply_options(options) {
                    err.with_prefix("Failed to update series options")
                        .with_serializable_data(options)
                        .log();
                }
            })
        });
    }

    Effect::new({
        let handle = handle.clone();

        move || {
            data.with(|data| {
                if let Err(err) = handle.set_data(data) {
                    err.with_prefix("Failed to update data")
                        .with_serializable_data(data)
                        .log();
                }
            })
        }
    });

    Effect::new({
        let handle = handle.clone();

        move || {
            markers.with(|markers| {
                if let Err(err) = handle.set_markers(markers) {
                    err.with_prefix("Failed to set markers")
                        .with_serializable_data(markers)
                        .log();
                }
            })
        }
    });

    Effect::new({
        let handle = handle.clone();

        move || {
            price_lines.with(|price_lines| {
                if let Err(err) = handle.set_price_lines(price_lines) {
                    err.with_prefix("Failed to set price lines")
                        .with_serializable_data(price_lines)
                        .log();
                }
            })
        }
    });

    on_cleanup({
        let handle = handle.clone();

        move || {
            if let Err(err) = handle.remove() {
                err.with_prefix("Failed to remove series").log();
            }
        }
    });

    Some(handle)
}
