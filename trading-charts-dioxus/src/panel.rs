use crate::chart::use_chart;
use dioxus::prelude::*;

#[derive(Clone, Props)]
pub struct ChartPanelProps {
    children: Element,
}

impl PartialEq for ChartPanelProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[allow(non_snake_case)]
pub fn ChartPanel(props: ChartPanelProps) -> Element {
    let chart = use_chart();
    let panel_id = use_hook(move || chart.allocate_panel());
    use_context_provider(|| panel_id);

    props.children
}
