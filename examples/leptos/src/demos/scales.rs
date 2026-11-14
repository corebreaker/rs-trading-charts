use super::{base_options, Choice, Toggle};
use crate::data::{closes, daily_ohlcv};

use charts::{
    chart::Chart,
    data::options::{PriceScaleMode, PriceScaleOptions, PriceScaleSide, TimeScaleOptions},
    series::lines::{LineSeries, LineSeriesOptions},
};
use leptos::prelude::*;

#[component]
pub fn ScalesDemo() -> impl IntoView {
    let acme = Signal::stored(closes(&daily_ohlcv(3, 365, 100.0)));
    let globex = Signal::stored(closes(&daily_ohlcv(11, 365, 2_500.0)));
    let mode = RwSignal::new(PriceScaleMode::Normal);
    let left_visible = RwSignal::new(true);
    let inverted = RwSignal::new(false);
    let time_visible = RwSignal::new(true);

    let options = Signal::derive(move || {
        base_options()
            .with_left_price_scale(
                PriceScaleOptions::new()
                    .with_visible(left_visible.get())
                    .with_mode(mode.get())
                    .with_border_color("#2563eb"),
            )
            .with_right_price_scale(
                PriceScaleOptions::new()
                    .with_mode(mode.get())
                    .with_invert_scale(inverted.get())
                    .with_border_color("#dc2626"),
            )
            .with_time_scale(TimeScaleOptions::new().with_visible(time_visible.get()))
    });

    view! {
        <Choice
            label="Price scale mode"
            value=mode
            options=vec![
                (PriceScaleMode::Normal, "Normal"),
                (PriceScaleMode::Logarithmic, "Logarithmic"),
                (PriceScaleMode::Percentage, "Percentage"),
                (PriceScaleMode::IndexedTo100, "Indexed to 100"),
            ]
        />
        <div class="toolbar">
            <Toggle label="Left price scale" value=left_visible />
            <Toggle label="Invert the right price scale" value=inverted />
            <Toggle label="Time scale" value=time_visible />
        </div>
        <Chart options=options class="chart">
            <LineSeries
                data=acme
                options=LineSeriesOptions::new()
                    .with_title("ACME (left)")
                    .with_color("#2563eb")
                    .with_price_scale_id(PriceScaleSide::Left)
            />
            <LineSeries
                data=globex
                options=LineSeriesOptions::new()
                    .with_title("GLOBEX (right)")
                    .with_color("#dc2626")
                    .with_price_scale_id(PriceScaleSide::Right)
            />
        </Chart>
    }
}
