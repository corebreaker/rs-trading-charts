use crate::{data::PaneOptions, ChartHandle};
use leptos::prelude::*;

/// Pane of the enclosing `Chart`: the series and watermarks given as children are drawn in this pane.
///
/// The panes are allocated in rendering order: the first `ChartPanel` is the main pane (index 0), the next ones are
/// stacked below it. The series which are not in a `ChartPanel` are drawn in the main pane.
#[component]
pub fn ChartPanel(
    /// Settings of the pane (stretch factor, height, preservation when empty), applied again when they change.
    #[prop(optional, into)]
    options: Option<Signal<PaneOptions>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let Some(chart) = use_context::<ChartHandle>() else {
        return children.map(|children| children()).into_any();
    };

    let panel_id = chart.allocate_panel();
    if let Some(options) = options {
        Effect::new(move || {
            options.with(|options| {
                if let Err(err) = chart.configure_pane(panel_id, options) {
                    err.with_prefix("Failed to configure the pane").log();
                }
            })
        });
    }

    provide_context(panel_id);
    children.map(|children| children()).into_any()
}
