use crate::ChartHandle;
use leptos::{
    IntoView,
    children::Children,
    component,
    context::{Provider, use_context},
    tachys::view::any_view::IntoAny,
    view,
};

#[component]
pub fn ChartPanel(#[prop(optional)] children: Option<Children>) -> impl IntoView {
    match children {
        None => view!(<></>).into_any(),
        Some(children) => {
            let chart: Option<ChartHandle> = use_context();
            match chart {
                None => children().into_any(),
                Some(chart) => {
                    let panel_id = chart.allocate_panel();

                    view! {
                        <Provider value=panel_id>
                            {children()}
                        </Provider>
                    }
                    .into_any()
                }
            }
        }
    }
}
