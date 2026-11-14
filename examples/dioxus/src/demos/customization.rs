use super::Choice;
use crate::data::intraday;

use charts::{
    chart::Chart,
    data::{
        format::{
            load_web_font,
            register_price_formatter,
            CurrencyDisplay,
            DateFormat,
            FontFamily,
            PriceFormatter,
            TimeFormatter,
        },
        options::{
            crosshair::{CrosshairMode, CrosshairOptions},
            layout::LayoutOptions,
            ChartOptions,
            LocalizationOptions,
            TimeScaleOptions,
        },
    },
    series::candlesticks::{CandleStickSeries, CandlestickOptions},
};
use dioxus::prelude::*;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq)]
enum Prices {
    Default,
    Dollars,
    Euros,
    Compact,
    Cents,
}

impl Prices {
    fn formatter(self, locale: &str) -> PriceFormatter {
        match self {
            Prices::Default => PriceFormatter::Default,
            Prices::Dollars => PriceFormatter::currency_in_locale("USD", locale),
            Prices::Euros => {
                PriceFormatter::currency_in_locale("EUR", locale).with_currency_display(CurrencyDisplay::Code)
            }
            Prices::Compact => PriceFormatter::Compact {
                locale:                  Some(locale.to_string()),
                minimum_fraction_digits: None,
                maximum_fraction_digits: Some(1),
            },
            // Rust closure registered below.
            Prices::Cents => PriceFormatter::custom("cents"),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Font {
    System,
    Monospace,
    Serif,
    Inter,
}

impl Font {
    fn family(&self) -> FontFamily {
        match self {
            Font::System => FontFamily::System,
            Font::Monospace => FontFamily::Monospace,
            Font::Serif => FontFamily::Serif,
            Font::Inter => FontFamily::Inter,
        }
    }
}

#[component]
pub fn CustomizationDemo() -> Element {
    let data = use_hook(|| {
        let _ = register_price_formatter("cents", |price| format!("{:.0}¢", price * 100.0));

        Rc::new(intraday())
    });
    let time_zone = use_signal(|| "America/New_York");
    let locale = use_signal(|| "en-US");
    let prices = use_signal(|| Prices::Default);
    let date_format = use_signal(|| DateFormat::Default);
    let font = use_signal(|| Font::System);
    let crosshair = use_signal(|| CrosshairMode::Magnet);

    // The Inter font is loaded from the web the first time it is selected; the chart is redrawn once it is loaded.
    let mut inter_loaded = use_signal(|| false);
    use_effect(move || {
        if font() == Font::Inter && !*inter_loaded.peek() {
            inter_loaded.set(true);
            if let Err(err) = load_web_font("Inter", "https://rsms.me/inter/font-files/InterVariable.woff2") {
                err.log();
            }
        }
    });

    let options = ChartOptions::new()
        .with_auto_size(true)
        .with_layout(
            LayoutOptions::new()
                .with_attribution_logo(false)
                .with_font_family(font().family())
                .with_font_size(13),
        )
        .with_localization(
            LocalizationOptions::new()
                .with_locale(locale())
                .with_date_format(date_format())
                .with_price_formatter(prices().formatter(locale()))
                .with_time_formatter(TimeFormatter::pattern_in_locale("EEE d MMM yyyy HH:mm", locale())),
        )
        .with_time_scale(TimeScaleOptions::new().with_time_visible(true))
        .with_crosshair(CrosshairOptions::new().with_mode(crosshair()));

    rsx! {
        Choice {
            label: "Time zone",
            value: time_zone,
            options: vec![
                ("UTC", "UTC"),
                ("America/New_York", "New York"),
                ("Europe/Paris", "Paris"),
                ("Asia/Tokyo", "Tokyo"),
            ],
        }
        Choice {
            label: "Locale",
            value: locale,
            options: vec![
                ("en-US", "English (US)"),
                ("fr-FR", "Français"),
                ("de-DE", "Deutsch"),
                ("ja-JP", "日本語"),
            ],
        }
        Choice {
            label: "Prices",
            value: prices,
            options: vec![
                (Prices::Default, "Default"),
                (Prices::Dollars, "Dollars"),
                (Prices::Euros, "Euros"),
                (Prices::Compact, "Compact"),
                (Prices::Cents, "Cents (Rust closure)"),
            ],
        }
        Choice {
            label: "Date format",
            value: date_format,
            options: vec![
                (DateFormat::Default, "Default"),
                (DateFormat::Iso, "ISO"),
                (DateFormat::DayMonthYear, "dd/MM/yyyy"),
                (DateFormat::Long, "Long"),
            ],
        }
        Choice {
            label: "Font",
            value: font,
            options: vec![
                (Font::System, "System"),
                (Font::Monospace, "Monospace"),
                (Font::Serif, "Serif"),
                (Font::Inter, "Inter (web font)"),
            ],
        }
        Choice {
            label: "Crosshair",
            value: crosshair,
            options: vec![
                (CrosshairMode::Normal, "Normal"),
                (CrosshairMode::Magnet, "Magnet"),
                (CrosshairMode::MagnetOHLC, "Magnet OHLC"),
                (CrosshairMode::Hidden, "Hidden"),
            ],
        }
        Chart { options, time_zone: time_zone().to_string(), class: "chart",
            CandleStickSeries { data: data.to_vec(), options: CandlestickOptions::new().with_title("5 minutes") }
        }
    }
}
