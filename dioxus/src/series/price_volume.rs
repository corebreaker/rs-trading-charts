use crate::{
    data::{Marker, OhlcvData, PriceLineOptions, PriceSeriesStyle, PriceVolume, SeriesMarkersOptions, VolumeColors},
    series::mount::use_series,
    series_data::histograms::HistogramSeriesOptions,
};

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct PriceVolumeSeriesProps {
    data:            Vec<OhlcvData>,
    #[props(default)]
    style:           PriceSeriesStyle,
    #[props(default)]
    volume_options:  Option<HistogramSeriesOptions>,
    #[props(default)]
    volume_colors:   VolumeColors,
    /// Ratio of the pane height taken by the volumes, from 0 to 1.
    #[props(default = 0.25)]
    volume_height:   f64,
    #[props(default)]
    markers:         Vec<Marker>,
    #[props(default)]
    markers_options: Option<SeriesMarkersOptions>,
    #[props(default)]
    price_lines:     Vec<PriceLineOptions>,
}

/// Add prices and volumes to the enclosing `Chart`, in the enclosing `ChartPanel` if any.
///
/// The prices are drawn with the `style` (candlesticks by default), and the volumes as a histogram at the bottom of
/// the pane, on an overlay price scale, taking `volume_height` of the pane height. The volume columns are colored by
/// direction of the periods. The markers and price lines belong to the price series.
///
/// The type of the price series is chosen when the component is mounted: changing the style to another type has no
/// effect until the component is mounted again.
#[allow(non_snake_case)]
pub fn PriceVolumeSeries(props: PriceVolumeSeriesProps) -> Element {
    let price_data = PriceVolume::price_data(&props.data, &props.style);
    let volume_data = PriceVolume::volume_data(&props.data, &props.volume_colors);
    let price = use_series(
        props.style.series_type(),
        Some(props.style),
        price_data,
        props.markers,
        props.markers_options,
        props.price_lines,
    );
    let volume = use_series(
        "histogram",
        Some(PriceVolume::volume_options(props.volume_options)),
        volume_data,
        Vec::new(),
        None,
        Vec::new(),
    );

    use_effect(use_reactive((&props.volume_height,), move |(volume_height,)| {
        if let (Some(price), Some(volume)) = (&price, &volume) {
            let (price_scale, volume_scale) = PriceVolume::price_scale_options(volume_height);
            let res = price
                .apply_price_scale_options(&price_scale)
                .and_then(|_| volume.apply_price_scale_options(&volume_scale));

            if let Err(err) = res {
                err.with_prefix("Failed to apply the price and volume scale margins")
                    .log();
            }
        }
    }));

    rsx! {}
}
