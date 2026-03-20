use super::CandlestickOptions;
use crate::{
    PanelId,
    chart::use_chart,
    data::{Candlestick, Marker, PriceLineOptions, series::Series},
};
use dioxus::prelude::*;

#[derive(Clone, Props)]
pub struct CandleStickSeriesProps {
    data: Vec<Candlestick>,
    markers: Vec<Marker>,
    #[props(default)]
    price_lines: Vec<PriceLineOptions>,
    #[props(default)]
    options: Option<CandlestickOptions>,
}

impl PartialEq for CandleStickSeriesProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[allow(non_snake_case)]
pub fn CandleStickSeries(props: CandleStickSeriesProps) -> Element {
    let chart = use_chart();
    let panel_id = try_use_context::<PanelId>();
    let series_id = use_hook({
        let chart = chart.clone();
        let options = props.options.clone();
        move || {
            let mut series: Series<Candlestick, CandlestickOptions> = Series::new("candlestick");
            if let Some(panel_id) = panel_id {
                series.set_panel(panel_id);
            }
            if let Some(options) = options.clone() {
                series.set_options(options);
            }

            match chart.add_series(&mut series) {
                Ok(()) => series.id().cloned(),
                Err(err) => {
                    err.with_prefix("Failed to add series").log();
                    None
                }
            }
        }
    });

    {
        let chart = chart.clone();
        let series_id = series_id.clone();
        let options = props.options.clone();
        use_effect(move || {
            if let (Some(series_id), Some(options)) = (series_id.clone(), options.as_ref()) {
                if let Err(err) = chart.update_series_options(series_id, options) {
                    err.with_prefix("Failed to update series options").log();
                }
            }
        });
    }

    {
        let chart = chart.clone();
        let series_id = series_id.clone();
        let data = props.data.clone();
        use_effect(move || {
            if let Some(series_id) = series_id.clone() {
                if let Err(err) = chart.update_data(series_id, &data) {
                    err.with_prefix("Failed to update data")
                        .with_serializable_data(&data)
                        .log();
                }
            }
        });
    }

    {
        let chart = chart.clone();
        let series_id = series_id.clone();
        let markers = props.markers.clone();
        use_effect(move || {
            if let Some(series_id) = series_id.clone() {
                if let Err(err) = chart.set_markers(series_id, &markers) {
                    err.with_prefix("Failed to set markers")
                        .with_serializable_data(&markers)
                        .log();
                }
            }
        });
    }

    {
        let chart = chart.clone();
        let series_id = series_id.clone();
        let price_lines = props.price_lines.clone();
        use_effect(move || {
            if let Some(series_id) = series_id.clone() {
                if let Err(err) = chart.set_price_lines(series_id, &price_lines) {
                    err.with_prefix("Failed to set price lines")
                        .with_serializable_data(&price_lines)
                        .log();
                }
            }
        });
    }

    rsx! {}
}
