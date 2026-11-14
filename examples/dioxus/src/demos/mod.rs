pub mod customization;
pub mod multi_charts;
pub mod panes;
pub mod price_volume;
pub mod scales;
pub mod series;
pub mod tooltips;

use charts::{
    data::options::{layout::LayoutOptions, ChartOptions},
    ChartHandle,
    JsError,
};
use dioxus::prelude::*;

/// Chart options shared by the demos: the chart follows the size of its container.
pub fn base_options() -> ChartOptions {
    ChartOptions::new()
        .with_auto_size(true)
        .with_layout(LayoutOptions::new().with_attribution_logo(false))
}

/// Run an action on the chart handle kept from the `on_ready` event of a chart, logging its error.
pub fn with_chart(chart: Signal<Option<ChartHandle>>, action: impl FnOnce(&ChartHandle) -> Result<(), JsError>) {
    if let Some(chart) = chart.peek().as_ref() {
        if let Err(err) = action(chart) {
            err.log();
        }
    }
}

/// Toolbar of buttons selecting one value among several.
#[component]
pub fn Choice<T: Clone + PartialEq + 'static>(
    label: &'static str,
    options: Vec<(T, &'static str)>,
    mut value: Signal<T>,
) -> Element {
    let buttons = options.into_iter().map(move |(option, text)| {
        let active = *value.read() == option;

        rsx! {
            button {
                key: "{text}",
                class: if active { "active" },
                onclick: move |_| value.set(option.clone()),
                "{text}"
            }
        }
    });

    rsx! {
        div { class: "toolbar",
            span { class: "label", "{label}" }
            {buttons}
        }
    }
}

/// Checkbox bound to a boolean signal.
#[component]
pub fn Toggle(label: &'static str, mut value: Signal<bool>) -> Element {
    rsx! {
        label { class: "toggle",
            input { r#type: "checkbox", checked: value(), onchange: move |_| value.toggle() }
            "{label}"
        }
    }
}
