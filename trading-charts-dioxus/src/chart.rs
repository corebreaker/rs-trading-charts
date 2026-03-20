use dioxus::prelude::*;
use dioxus_web::WebEventExt;
use trading_charts_core::{
    ChartHandle, JsError,
    data::{LegendOptions, options::ChartOptions},
};
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;

#[derive(Clone, Props)]
pub struct ChartProps {
    #[props(default)]
    options: Option<ChartOptions>,
    #[props(default)]
    legend: Option<LegendOptions>,
    #[props(default, into)]
    style: Option<String>,
    #[props(default, into)]
    class: Option<String>,
    #[props(default)]
    children: Element,
}

impl PartialEq for ChartProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[allow(non_snake_case)]
pub fn Chart(props: ChartProps) -> Element {
    let initial_options = props.options.clone();
    let chart = use_hook(move || {
        ChartHandle::new(initial_options.as_ref()).map_err(|err| err.with_prefix("Failed to create chart"))
    });
    let chart = match chart {
        Ok(chart) => chart,
        Err(err) => {
            err.log();
            return rsx! {};
        }
    };

    use_context_provider(|| chart.clone());

    let mut node = use_signal(|| None::<HtmlDivElement>);

    {
        let chart = chart.clone();
        use_effect(move || {
            if let Some(div) = node() {
                if let Err(err) = chart.bind(div) {
                    err.with_prefix("Failed to bind chart").log();
                }
            }
        });
    }

    {
        let chart = chart.clone();
        let options = props.options.clone();
        use_effect(move || {
            if let Some(options) = options.as_ref() {
                if let Err(err) = chart.apply_chart_options(options) {
                    err.with_prefix("Failed to apply chart options").log();
                }
            }
        });
    }

    {
        let chart = chart.clone();
        let legend = props.legend.clone();
        use_effect(move || match legend.as_ref() {
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
        });
    }

    rsx! {
        div {
            style: props.style.unwrap_or_default(),
            class: props.class.unwrap_or_default(),
            onmounted: move |event| {
                let element = event.as_web_event();
                match element.dyn_into::<HtmlDivElement>() {
                    Ok(div) => node.set(Some(div)),
                    Err(element) => JsError::from_displayable(format!("Mounted element is not a div: {:?}", element)).log(),
                }
            },
        }
        {props.children}
    }
}

pub fn use_chart() -> ChartHandle {
    try_use_context::<ChartHandle>().expect("Chart context not found. Ensure the component is rendered inside <Chart>.")
}
