use super::{base_options, with_chart, Choice};
use crate::data::{candlesticks, daily_ohlcv, rsi};

use charts::{
    chart::Chart,
    data::{
        options::{LineStyle, VertAlign},
        Candlestick,
        HistogramData,
        ImageFormat,
        ImageWatermarkOptions,
        PaneOptions,
        PriceLineOptions,
        ScreenshotOptions,
        TextWatermarkLineOptions,
        TextWatermarkOptions,
        ValueData,
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
use dioxus::prelude::*;
use std::rc::Rc;

const LOGO: &str = "<svg xmlns='http://www.w3.org/2000/svg' width='96' height='96' viewBox='0 0 24 24'>\
<path fill='none' stroke='%232563eb' stroke-width='2' d='M3 17l6-6 4 4 8-8'/></svg>";

/// Data of the demo, computed once.
struct PanesData {
    candles:    Vec<Candlestick>,
    volumes:    Vec<HistogramData>,
    rsi:        Vec<ValueData>,
    rsi_levels: Vec<PriceLineOptions>,
}

impl PanesData {
    fn new() -> Self {
        let data = daily_ohlcv(21, 200, 60.0);

        Self {
            candles:    candlesticks(&data),
            volumes:    data
                .iter()
                .map(|item| {
                    let color = if item.is_up() { "#26a69a" } else { "#ef5350" };

                    HistogramData::new(item.time, item.volume).with_color(color)
                })
                .collect(),
            rsi:        rsi(&data, 14),
            rsi_levels: vec![
                PriceLineOptions::new(70.0)
                    .with_color("#dc2626")
                    .with_line_style(LineStyle::Dashed)
                    .with_title("70"),
                PriceLineOptions::new(30.0)
                    .with_color("#16a34a")
                    .with_line_style(LineStyle::Dashed)
                    .with_title("30"),
            ],
        }
    }
}

#[component]
pub fn PanesDemo() -> Element {
    let data = use_hook(|| Rc::new(PanesData::new()));
    let stretch = use_signal(|| 1.0);
    let mut chart = use_signal(|| None::<ChartHandle>);
    let mut preview = use_signal(|| None::<String>);

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
    let lower_pane = PaneOptions::new().with_stretch_factor(stretch());

    rsx! {
        Choice {
            label: "Lower panes stretch factor",
            value: stretch,
            options: vec![(0.5, "0.5"), (1.0, "1"), (2.0, "2")],
        }
        div { class: "toolbar",
            button { onclick: move |_| with_chart(chart, |chart| chart.swap_panes(1, 2)), "Swap the lower panes" }
            button {
                onclick: move |_| {
                    with_chart(chart, |chart| {
                        let options = ScreenshotOptions::new().with_top_layer(true);

                        chart.download_screenshot("chart.png", &options)
                    })
                },
                "Download a PNG screenshot"
            }
            button {
                onclick: move |_| {
                    with_chart(chart, |chart| {
                        let options = ScreenshotOptions::new().with_format(ImageFormat::Jpeg(0.8));

                        chart.take_screenshot(&options).map(|url| preview.set(Some(url)))
                    })
                },
                "Preview a JPEG screenshot"
            }
        }
        Chart {
            options: base_options(),
            class: "chart tall",
            on_ready: move |handle| chart.set(Some(handle)),
            ChartPanel { options: PaneOptions::new().with_stretch_factor(3.0),
                CandleStickSeries { data: data.candles.clone(), options: CandlestickOptions::new().with_title("ACME") }
                TextWatermark { options: text_watermark }
            }
            ChartPanel { options: lower_pane.clone(),
                HistogramSeries {
                    data: data.volumes.clone(),
                    options: HistogramSeriesOptions::new().with_title("Volume"),
                }
            }
            ChartPanel { options: lower_pane,
                LineSeries {
                    data: data.rsi.clone(),
                    price_lines: data.rsi_levels.clone(),
                    options: LineSeriesOptions::new().with_title("RSI 14").with_color("#7c3aed"),
                }
                ImageWatermark { url: format!("data:image/svg+xml;utf8,{LOGO}"), options: image_watermark }
            }
        }
        if let Some(url) = preview() {
            img { class: "preview", src: url, alt: "Screenshot preview" }
        }
    }
}
