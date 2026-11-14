use super::{base_options, keep, Choice};
use crate::data::{candlesticks, daily_ohlcv, rsi};

use charts::{
    chart::Chart,
    data::{
        options::{LineStyle, VertAlign},
        HistogramData,
        ImageFormat,
        ImageWatermarkOptions,
        PaneOptions,
        PriceLineOptions,
        ScreenshotOptions,
        TextWatermarkLineOptions,
        TextWatermarkOptions,
    },
    panel::ChartPanel,
    series::{
        candlesticks::{CandleStickSeries, CandlestickOptions},
        histograms::{HistogramSeries, HistogramSeriesOptions},
        lines::{LineSeries, LineSeriesOptions},
    },
    watermark::{ImageWatermark, TextWatermark},
    ChartHandle,
};
use leptos::prelude::*;

const LOGO: &str = "<svg xmlns='http://www.w3.org/2000/svg' width='96' height='96' viewBox='0 0 24 24'>\
<path fill='none' stroke='%232563eb' stroke-width='2' d='M3 17l6-6 4 4 8-8'/></svg>";

#[component]
pub fn PanesDemo() -> impl IntoView {
    let data = daily_ohlcv(21, 200, 60.0);
    let candles = Signal::stored(candlesticks(&data));
    let volumes = Signal::stored(
        data.iter()
            .map(|item| {
                let color = if item.is_up() { "#26a69a" } else { "#ef5350" };

                HistogramData::new(item.time, item.volume).with_color(color)
            })
            .collect::<Vec<_>>(),
    );
    let rsi = Signal::stored(rsi(&data, 14));
    let rsi_levels = Signal::stored(vec![
        PriceLineOptions::new(70.0)
            .with_color("#dc2626")
            .with_line_style(LineStyle::Dashed)
            .with_title("70"),
        PriceLineOptions::new(30.0)
            .with_color("#16a34a")
            .with_line_style(LineStyle::Dashed)
            .with_title("30"),
    ]);
    let stretch = RwSignal::new(1.0);
    let chart = StoredValue::new(None::<ChartHandle>);
    let preview = RwSignal::new(None::<String>);

    let text_watermark = TextWatermarkOptions::new()
        .with_vert_align(VertAlign::Top)
        .with_lines(vec![
            TextWatermarkLineOptions::new("ACME")
                .with_font_size(48)
                .with_color("rgba(37, 99, 235, 0.15)"),
            TextWatermarkLineOptions::new("Daily")
                .with_font_size(20)
                .with_color("rgba(37, 99, 235, 0.15)"),
        ]);
    let image_watermark = ImageWatermarkOptions::new().with_max_height(48).with_alpha(0.3);

    let with_chart = move |action: &dyn Fn(&ChartHandle) -> Result<(), charts::JsError>| {
        if let Some(chart) = chart.get_value() {
            if let Err(err) = action(&chart) {
                err.log();
            }
        }
    };

    view! {
        <Choice
            label="Lower panes stretch factor"
            value=stretch
            options=vec![(0.5, "0.5"), (1.0, "1"), (2.0, "2")]
        />
        <div class="toolbar">
            <button on:click=move |_| with_chart(&|chart| chart.swap_panes(1, 2))>"Swap the lower panes"</button>
            <button on:click=move |_| {
                with_chart(&|chart| {
                    let options = ScreenshotOptions::new().with_top_layer(true);

                    chart.download_screenshot("chart.png", &options)
                })
            }>"Download a PNG screenshot"</button>
            <button on:click=move |_| {
                with_chart(&|chart| {
                    let options = ScreenshotOptions::new().with_format(ImageFormat::Jpeg(0.8));

                    chart.take_screenshot(&options).map(|url| preview.set(Some(url)))
                })
            }>"Preview a JPEG screenshot"</button>
        </div>
        <Chart options=base_options() class="chart tall" on_ready=keep(chart)>
            <ChartPanel options=PaneOptions::new().with_stretch_factor(3.0)>
                <CandleStickSeries data=candles options=CandlestickOptions::new().with_title("ACME") />
                <TextWatermark options=text_watermark />
            </ChartPanel>
            <ChartPanel options=Signal::derive(move || PaneOptions::new().with_stretch_factor(stretch.get()))>
                <HistogramSeries data=volumes options=HistogramSeriesOptions::new().with_title("Volume") />
            </ChartPanel>
            <ChartPanel options=Signal::derive(move || PaneOptions::new().with_stretch_factor(stretch.get()))>
                <LineSeries
                    data=rsi
                    price_lines=rsi_levels
                    options=LineSeriesOptions::new().with_title("RSI 14").with_color("#7c3aed")
                />
                <ImageWatermark url=format!("data:image/svg+xml;utf8,{LOGO}") options=image_watermark />
            </ChartPanel>
        </Chart>
        {move || preview.get().map(|url| view! { <img class="preview" src=url alt="Screenshot preview" /> })}
    }
}
