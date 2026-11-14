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
};
use leptos::prelude::*;

/// Chart options shared by the demos: the chart follows the size of its container.
pub fn base_options() -> ChartOptions {
    ChartOptions::new()
        .with_auto_size(true)
        .with_layout(LayoutOptions::new().with_attribution_logo(false))
}

/// `on_ready` callback of a chart, keeping its handle to control it from outside.
pub fn keep(chart: StoredValue<Option<ChartHandle>>) -> impl Fn(ChartHandle) + Send + Sync + 'static {
    move |handle| chart.set_value(Some(handle))
}

/// Toolbar of buttons selecting one value among several.
#[component]
pub fn Choice<T>(label: &'static str, options: Vec<(T, &'static str)>, value: RwSignal<T>) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static, {
    view! {
        <div class="toolbar">
            <span class="label">{label}</span>
            {options
                .into_iter()
                .map(|(option, text)| {
                    let selected = option.clone();

                    view! {
                        <button
                            class:active=move || value.with(|value| *value == selected)
                            on:click=move |_| value.set(option.clone())
                        >
                            {text}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Checkbox bound to a boolean signal.
#[component]
pub fn Toggle(label: &'static str, value: RwSignal<bool>) -> impl IntoView {
    view! {
        <label class="toggle">
            <input type="checkbox" prop:checked=move || value.get() on:change=move |_| value.update(|v| *v = !*v) />
            {label}
        </label>
    }
}
