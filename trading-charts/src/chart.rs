use super::data::options::ChartOptions;
use crate::bindings::TradingChartBinding;
use leptos::{
    tachys::{
        html::{
            attribute::global::{StyleAttribute, ClassAttribute},
            node_ref::NodeRefAttribute,
        },
        view::any_view::IntoAny,
        reactive_graph::node_ref::NodeRef,
    },
    reactive::{
        effect::Effect,
        traits::{Get, With, WithUntracked},
        signal::ReadSignal,
    },
    context::provide_context,
    children::Children,
    html::Div,
    IntoView,
    component,
    view,
};

#[component]
pub fn Chart<'a, 'b>(
    #[prop(optional)] options: Option<ReadSignal<ChartOptions>>,

    #[prop(optional)] style: Option<&'a str>,

    #[prop(optional)] class: Option<&'b str>,

    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let chart = match options {
        None => TradingChartBinding::new(None).unwrap(),
        Some(options) => {
            let chart = {
                options
                    .with_untracked(|options| TradingChartBinding::new(Some(options)))
                    .unwrap()
            };

            let _ = Effect::new({
                let chart = chart.clone();

                move || {
                    options.with(|options| {
                        // TODO: handle error
                        chart.apply_chart_options(options).unwrap();
                    })
                }
            });

            chart
        }
    };

    let node_ref = NodeRef::<Div>::new();
    let _ = Effect::new({
        let chart = chart.clone();

        move || {
            if let Some(node) = node_ref.get() {
                chart.bind_chart(node).unwrap();
            }
        }
    });

    provide_context(chart);

    let style = style.map_or_else(String::new, |s| s.to_string());
    let class = class.map_or_else(String::new, |s| s.to_string());
    let children = match children {
        Some(children) => children().into_any(),
        None => (view! {<></>}).into_any(),
    };

    view! {
        <>
            <div style=style class=class node_ref={node_ref}/>
            {children}
        </>
    }
}
