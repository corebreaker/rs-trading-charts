use crate::bindings::TradingChartBinding;
use leptos::{tachys::view::any_view::IntoAny, children::Children, context::use_context, IntoView, component, view};

#[component]
pub fn ChartPanel(#[prop(optional)] children: Option<Children>) -> impl IntoView {
    match children {
        None => view!(<></>).into_any(),
        Some(children) => {
            let chart: Option<TradingChartBinding> = use_context();
            if let Some(chart) = chart.clone() {
                if let Err(err) = chart.add_panel() {
                    err.with_prefix("Failed to add panel").log();
                }
            };

            let res = children().into_any();
            if let Some(chart) = chart {
                if let Err(err) = chart.remove_panel() {
                    err.with_prefix("Failed to remove panel").log();
                }
            };

            res
        }
    }
}
