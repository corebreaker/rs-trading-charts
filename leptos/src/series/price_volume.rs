use crate::{
    data::{Marker, OhlcvData, PriceLineOptions, PriceSeriesStyle, PriceVolume, SeriesMarkersOptions, VolumeColors},
    series::mount::{mount_series, SeriesInputs},
    series_data::histograms::HistogramSeriesOptions,
};

use leptos::prelude::*;

/// Add prices and volumes to the enclosing `Chart`, in the enclosing `ChartPanel` if any.
///
/// The prices are drawn with the `style` (candlesticks by default), and the volumes as a histogram at the bottom of
/// the pane, on an overlay price scale, taking `volume_height` of the pane height (`0.25` by default). The volume
/// columns are colored by direction of the periods. The markers and price lines belong to the price series.
///
/// The type of the price series is chosen when the component is mounted: changing the style to another type has no
/// effect until the component is mounted again.
#[component(transparent)]
pub fn PriceVolumeSeries(
    #[prop(into)] data: Signal<Vec<OhlcvData>>,
    #[prop(optional, into)] style: Option<Signal<PriceSeriesStyle>>,
    #[prop(optional, into)] volume_options: Option<Signal<HistogramSeriesOptions>>,
    #[prop(optional, into)] volume_colors: Option<Signal<VolumeColors>>,
    /// Ratio of the pane height taken by the volumes, from 0 to 1.
    #[prop(optional, into)]
    volume_height: Option<Signal<f64>>,
    #[prop(optional, into)] markers: Signal<Vec<Marker>>,
    #[prop(optional, into)] markers_options: Option<Signal<SeriesMarkersOptions>>,
    #[prop(optional, into)] price_lines: Signal<Vec<PriceLineOptions>>,
) -> impl IntoView {
    let style = style.unwrap_or_else(|| Signal::stored(PriceSeriesStyle::default()));
    let colors = volume_colors.unwrap_or_else(|| Signal::stored(VolumeColors::default()));
    let series_type = style.with_untracked(|style| style.series_type());

    let price = mount_series(
        series_type,
        SeriesInputs {
            options: Some(style),
            data: Signal::derive(move || data.with(|data| style.with(|style| PriceVolume::price_data(data, style)))),
            markers,
            markers_options,
            price_lines,
        },
    );

    let volume = mount_series(
        "histogram",
        SeriesInputs {
            options:         Some(Signal::derive(move || {
                PriceVolume::volume_options(volume_options.map(|options| options.get()))
            })),
            data:            Signal::derive(move || {
                data.with(|data| colors.with(|colors| PriceVolume::volume_data(data, colors)))
            }),
            markers:         Signal::default(),
            markers_options: None,
            price_lines:     Signal::default(),
        },
    );

    if let (Some(price), Some(volume)) = (price, volume) {
        Effect::new(move || {
            let height = volume_height.map(|height| height.get()).unwrap_or(0.25);
            let (price_scale, volume_scale) = PriceVolume::price_scale_options(height);
            let res = price
                .apply_price_scale_options(&price_scale)
                .and_then(|_| volume.apply_price_scale_options(&volume_scale));

            if let Err(err) = res {
                err.with_prefix("Failed to apply the price and volume scale margins")
                    .log();
            }
        });
    }
}
