use crate::demos::{
    customization::CustomizationDemo,
    multi_charts::MultiChartsDemo,
    panes::PanesDemo,
    price_volume::PriceVolumeDemo,
    scales::ScalesDemo,
    series::SeriesDemo,
    tooltips::TooltipsDemo,
};

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Demo {
    Series,
    PriceVolume,
    Scales,
    Tooltips,
    Customization,
    Panes,
    MultiCharts,
}

impl Demo {
    const ALL: [Demo; 7] = [
        Demo::Series,
        Demo::PriceVolume,
        Demo::Scales,
        Demo::Tooltips,
        Demo::Customization,
        Demo::Panes,
        Demo::MultiCharts,
    ];

    fn title(self) -> &'static str {
        match self {
            Demo::Series => "Series",
            Demo::PriceVolume => "Price & volume",
            Demo::Scales => "Scales",
            Demo::Tooltips => "Tooltips & legends",
            Demo::Customization => "Customization",
            Demo::Panes => "Panes & watermarks",
            Demo::MultiCharts => "Multi-charts",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Demo::Series => {
                "The six series types, with markers (built-in, custom and price-positioned), price lines, whitespace \
                 data (the empty slots of the time scale) and a moving average. Changing the type mounts another \
                 series component."
            }
            Demo::PriceVolume => {
                "Prices and volumes on the same chart, with the price style, the bar interval (resampled data) and the \
                 visible range chosen with the buttons."
            }
            Demo::Scales => {
                "Two visible price scales, price scale modes, inverted scale and hidden time scale, applied as \
                 reactive chart options."
            }
            Demo::Tooltips => {
                "Tooltips rendered by Dioxus components, in the tracking, floating and magnifier modes, with the \
                 built-in legends and a custom legend overlay."
            }
            Demo::Customization => {
                "Time zone, locale, date format, price formatters (including a Rust closure), font and crosshair mode \
                 on real 5-minute data."
            }
            Demo::Panes => {
                "Several panes with stretch factors, text and image watermarks, pane swapping and screenshots."
            }
            Demo::MultiCharts => "Two charts in a synchronization group, sharing their crosshair and visible range.",
        }
    }
}

#[component]
pub fn App() -> Element {
    let mut demo = use_signal(|| Demo::Series);

    rsx! {
        main {
            h1 { "Trading Charts for Dioxus" }
            nav {
                for item in Demo::ALL {
                    button {
                        key: "{item.title()}",
                        class: if demo() == item { "active" },
                        onclick: move |_| demo.set(item),
                        "{item.title()}"
                    }
                }
            }
            p { class: "description", "{demo().description()}" }
            match demo() {
                Demo::Series => rsx! { SeriesDemo {} },
                Demo::PriceVolume => rsx! { PriceVolumeDemo {} },
                Demo::Scales => rsx! { ScalesDemo {} },
                Demo::Tooltips => rsx! { TooltipsDemo {} },
                Demo::Customization => rsx! { CustomizationDemo {} },
                Demo::Panes => rsx! { PanesDemo {} },
                Demo::MultiCharts => rsx! { MultiChartsDemo {} },
            }
        }
    }
}
