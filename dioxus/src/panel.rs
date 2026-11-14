use crate::{chart::use_chart, data::PaneOptions};
use dioxus::prelude::*;

#[derive(Clone, Props)]
pub struct ChartPanelProps {
    /// Settings of the pane (stretch factor, height, preservation when empty), applied again when they change.
    #[props(default)]
    options:  Option<PaneOptions>,
    children: Element,
}

impl PartialEq for ChartPanelProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

/// Pane of the enclosing `Chart`: the series and watermarks given as children are drawn in this pane.
///
/// The panes are allocated in rendering order: the first `ChartPanel` is the main pane (index 0), the next ones are
/// stacked below it. The series which are not in a `ChartPanel` are drawn in the main pane.
#[allow(non_snake_case)]
pub fn ChartPanel(props: ChartPanelProps) -> Element {
    let chart = use_chart();
    let panel_id = use_hook({
        let chart = chart.clone();

        move || chart.allocate_panel()
    });

    use_context_provider(|| panel_id);
    use_effect(use_reactive((&props.options,), move |(options,)| {
        if let Some(options) = options {
            if let Err(err) = chart.configure_pane(panel_id, &options) {
                err.with_prefix("Failed to configure the pane").log();
            }
        }
    }));

    props.children
}
