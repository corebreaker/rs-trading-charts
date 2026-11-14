use crate::demos::{
    customization::CustomizationDemo,
    multi_charts::MultiChartsDemo,
    panes::PanesDemo,
    price_volume::PriceVolumeDemo,
    scales::ScalesDemo,
    series::SeriesDemo,
    tooltips::TooltipsDemo,
};

use leptos::prelude::*;

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
                 data (the gap of the line) and a moving average. Changing the type mounts another series component."
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
                "Tooltips rendered by Leptos components, in the tracking, floating and magnifier modes, with the \
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
pub fn App() -> impl IntoView {
    let demo = RwSignal::new(Demo::Series);

    view! {
        <main>
            <h1>"Trading Charts for Leptos"</h1>
            <nav>
                {Demo::ALL
                    .into_iter()
                    .map(|item| {
                        view! {
                            <button class:active=move || demo.get() == item on:click=move |_| demo.set(item)>
                                {item.title()}
                            </button>
                        }
                    })
                    .collect_view()}
            </nav>
            <p class="description">{move || demo.get().description()}</p>
            {move || match demo.get() {
                Demo::Series => view! { <SeriesDemo /> }.into_any(),
                Demo::PriceVolume => view! { <PriceVolumeDemo /> }.into_any(),
                Demo::Scales => view! { <ScalesDemo /> }.into_any(),
                Demo::Tooltips => view! { <TooltipsDemo /> }.into_any(),
                Demo::Customization => view! { <CustomizationDemo /> }.into_any(),
                Demo::Panes => view! { <PanesDemo /> }.into_any(),
                Demo::MultiCharts => view! { <MultiChartsDemo /> }.into_any(),
            }}
        </main>
    }
}
