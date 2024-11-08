use super::data::options::ChartOptions;
use crate::{bindings::TradingChartBinding, JsError};
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

fn make_chart(options: Option<ReadSignal<ChartOptions>>) -> Result<TradingChartBinding, JsError> {
    Ok(match options {
        None => TradingChartBinding::new(None)?,
        Some(options) => {
            let chart = options.with_untracked(|options| TradingChartBinding::new(Some(options)))?;
            let _ = Effect::new({
                let chart = chart.clone();

                move || {
                    options.with(|options| {
                        if let Err(err) = chart.apply_chart_options(options) {
                            err.with_prefix("Failed to apply chart options").log();
                        }
                    })
                }
            });

            chart
        }
    })
}

#[component]
pub fn Chart<'a, 'b>(
    #[prop(optional)] options: Option<ReadSignal<ChartOptions>>,

    #[prop(optional)] style: Option<&'a str>,

    #[prop(optional)] class: Option<&'b str>,

    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let chart = match make_chart(options) {
        Ok(chart) => chart,
        Err(err) => {
            err.with_prefix("Failed to create chart").log();

            return view!().into_any();
        }
    };

    let node_ref = NodeRef::<Div>::new();
    let _ = Effect::new({
        let chart = chart.clone();

        move || {
            if let Some(node) = node_ref.get() {
                if let Err(err) = chart.bind_chart(node) {
                    err.with_prefix("Failed to bind chart").log();
                }
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

    let res = view! {
        <>
            <div style=style class=class node_ref={node_ref}/>
            {children}
        </>
    };

    res.into_any()
}
