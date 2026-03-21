use super::data::{LegendOptions, options::ChartOptions};
use crate::{ChartHandle, JsError, REFIT_EVENT_KIND};
use emitix::{leptos::LeptosEventChannels, EventManager};
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
        wrappers::read::Signal,
    },
    children::Children,
    context::Provider,
    html::Div,
    IntoView,
    component,
    view,
};

fn make_chart(options: Option<Signal<ChartOptions>>) -> Result<ChartHandle, JsError> {
    Ok(match options {
        None => ChartHandle::new(None)?,
        Some(options) => {
            let chart = options.with_untracked(|options| ChartHandle::new(Some(options)))?;
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
pub fn Chart(
    #[prop(optional, into)] options: Option<Signal<ChartOptions>>,
    #[prop(optional, into)] legend: Option<Signal<Option<LegendOptions>>>,
    #[prop(optional, into)] style: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] refit: Option<LeptosEventChannels>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let chart = match make_chart(options) {
        Ok(chart) => chart,
        Err(err) => {
            err.with_prefix("Failed to create chart").log();

            return view!().into_any();
        }
    };

    if let Some(refit) = refit.as_ref() {
        let chart = chart.clone();

        let res = refit.add_listener(REFIT_EVENT_KIND, move |_| {
            if let Err(err) = chart.refit_content() {
                err.with_prefix("Failed to refit chart content").log();
            }
        });

        if let Err(err) = res {
            JsError::from_displayable(err)
                .with_prefix("Failed to add refit listener")
                .log();
        }
    }

    let node_ref = NodeRef::<Div>::new();
    let _ = Effect::new({
        let chart = chart.clone();

        move || {
            if let Some(node) = node_ref.get() {
                if let Err(err) = chart.bind(node) {
                    err.with_prefix("Failed to bind chart").log();
                }
            }
        }
    });

    if let Some(legend) = legend {
        let chart = chart.clone();
        let _ = Effect::new(move || {
            legend.with(|legend| match legend {
                Some(legend) => {
                    if let Err(err) = chart.apply_legend_options(legend) {
                        err.with_prefix("Failed to apply legend options").log();
                    }
                }
                None => {
                    if let Err(err) = chart.remove_legend() {
                        err.with_prefix("Failed to remove legend").log();
                    }
                }
            })
        });
    }

    let style = style.map_or_else(String::new, |s| s.to_string());
    let class = class.map_or_else(String::new, |s| s.to_string());
    match children {
        Some(children) => view! {
            <Provider value=chart>
                <div style=style class=class node_ref={node_ref}/>
                {children()}
            </Provider>
        }
        .into_any(),
        None => view! {
            <Provider value=chart>
                <div style=style class=class node_ref={node_ref}/>
            </Provider>
        }
        .into_any(),
    }
}
