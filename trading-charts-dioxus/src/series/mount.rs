use crate::{
    chart::use_chart,
    data::{series::Series, Marker, PriceLineOptions, SeriesMarkersOptions},
    PanelId,
    SeriesHandle,
};

use dioxus::prelude::*;
use serde::Serialize;

/// Add a series to the chart of the context, in the pane of the context, and keep it in sync with the given values.
///
/// This is a hook: it must be called unconditionally from a component. The series is created once, updated when a
/// value changes, and removed from the chart when the component is dropped. Returns `None` when the series cannot be
/// added.
pub(crate) fn use_series<Dat, Opt>(
    series_type: &'static str,
    options: Option<Opt>,
    data: Vec<Dat>,
    markers: Vec<Marker>,
    markers_options: Option<SeriesMarkersOptions>,
    price_lines: Vec<PriceLineOptions>,
) -> Option<SeriesHandle<Dat, Opt>>
where
    Dat: Serialize + Clone + PartialEq + 'static,
    Opt: Serialize + Clone + PartialEq + 'static, {
    let chart = use_chart();
    let panel_id = try_use_context::<PanelId>();
    let handle = use_hook({
        let options = options.clone();

        move || {
            let mut series: Series<Dat, Opt> = Series::new(series_type);
            if let Some(panel_id) = panel_id {
                series.set_panel(panel_id);
            }
            if let Some(options) = options {
                series.set_options(options);
            }

            match chart.add_series(&series) {
                Ok(handle) => Some(handle),
                Err(err) => {
                    err.with_prefix("Failed to add series").log();
                    None
                }
            }
        }
    });

    use_effect(use_reactive((&options,), {
        let handle = handle.clone();

        move |(options,)| {
            if let (Some(handle), Some(options)) = (&handle, &options) {
                if let Err(err) = handle.apply_options(options) {
                    err.with_prefix("Failed to update series options")
                        .with_serializable_data(options)
                        .log();
                }
            }
        }
    }));

    use_effect(use_reactive((&data,), {
        let handle = handle.clone();

        move |(data,)| {
            if let Some(handle) = &handle {
                if let Err(err) = handle.set_data(&data) {
                    err.with_prefix("Failed to update data")
                        .with_serializable_data(&data)
                        .log();
                }
            }
        }
    }));

    use_effect(use_reactive((&markers,), {
        let handle = handle.clone();

        move |(markers,)| {
            if let Some(handle) = &handle {
                if let Err(err) = handle.set_markers(&markers) {
                    err.with_prefix("Failed to set markers")
                        .with_serializable_data(&markers)
                        .log();
                }
            }
        }
    }));

    use_effect(use_reactive((&markers_options,), {
        let handle = handle.clone();

        move |(markers_options,)| {
            if let (Some(handle), Some(options)) = (&handle, &markers_options) {
                if let Err(err) = handle.apply_markers_options(options) {
                    err.with_prefix("Failed to apply markers options").log();
                }
            }
        }
    }));

    use_effect(use_reactive((&price_lines,), {
        let handle = handle.clone();

        move |(price_lines,)| {
            if let Some(handle) = &handle {
                if let Err(err) = handle.set_price_lines(&price_lines) {
                    err.with_prefix("Failed to set price lines")
                        .with_serializable_data(&price_lines)
                        .log();
                }
            }
        }
    }));

    use_drop({
        let handle = handle.clone();

        move || {
            if let Some(handle) = handle {
                if let Err(err) = handle.remove() {
                    err.with_prefix("Failed to remove series").log();
                }
            }
        }
    });

    handle
}
