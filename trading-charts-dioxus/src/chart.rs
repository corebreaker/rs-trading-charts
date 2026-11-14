use crate::{
    data::{options::ChartOptions, LegendOptions, LogicalRange, MouseEventParams, TimeRange},
    ChartHandle,
    JsError,
    Subscription,
};

use dioxus::{core::Runtime, prelude::*};
use dioxus_web::WebEventExt;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;

/// Crosshair state of a chart, provided to the children of [`Chart`].
#[derive(Clone, Copy)]
struct CrosshairState(Signal<Option<MouseEventParams>>);

/// Event handlers of a chart, updated at each render so that the subscriptions call the latest ones.
#[derive(Clone, Copy, Default)]
struct ChartEventHandlers {
    crosshair_move:               Option<EventHandler<MouseEventParams>>,
    click:                        Option<EventHandler<MouseEventParams>>,
    dbl_click:                    Option<EventHandler<MouseEventParams>>,
    visible_time_range_change:    Option<EventHandler<Option<TimeRange>>>,
    visible_logical_range_change: Option<EventHandler<Option<LogicalRange>>>,
}

#[derive(Clone, Props)]
pub struct ChartProps {
    /// Options of the chart, applied again when they change.
    #[props(default)]
    options:                         Option<ChartOptions>,
    /// Options of the built-in legend, which is removed with `None`.
    #[props(default)]
    legend:                          Option<LegendOptions>,
    /// IANA time zone in which the times are displayed, such as `Europe/Paris`; UTC by default.
    #[props(default, into)]
    time_zone:                       Option<String>,
    #[props(default, into)]
    style:                           Option<String>,
    #[props(default, into)]
    class:                           Option<String>,
    #[props(default)]
    on_crosshair_move:               Option<EventHandler<MouseEventParams>>,
    #[props(default)]
    on_click:                        Option<EventHandler<MouseEventParams>>,
    #[props(default)]
    on_dbl_click:                    Option<EventHandler<MouseEventParams>>,
    #[props(default)]
    on_visible_time_range_change:    Option<EventHandler<Option<TimeRange>>>,
    #[props(default)]
    on_visible_logical_range_change: Option<EventHandler<Option<LogicalRange>>>,
    #[props(default)]
    children:                        Element,
}

impl PartialEq for ChartProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

/// Run a subscription callback in the scope of the component, as it is called from JS.
fn in_component_scope<T: 'static>(mut callback: impl FnMut(T) + 'static) -> impl FnMut(T) + 'static {
    let runtime = Runtime::current();
    let scope = runtime.current_scope_id();

    move |value| runtime.in_scope(scope, || callback(value))
}

fn keep_subscription(subscription: Result<Subscription, JsError>, event: &str) -> Option<Rc<Subscription>> {
    match subscription {
        Ok(subscription) => Some(Rc::new(subscription)),
        Err(err) => {
            err.with_prefix(format!("Failed to subscribe to the {event} event"))
                .log();
            None
        }
    }
}

/// Chart, which renders its series, panes, tooltips and overlays given as children.
///
/// The chart fills a `div` sized by the `style` and `class` props, which is also the positioning context of the
/// overlays: give it a size, for instance with `style: "width:100%;height:400px"`, and set the `autoSize` chart
/// option to follow it.
///
/// The children can access the chart with [`use_chart`] and the crosshair state with [`use_crosshair`].
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

    let mut crosshair = use_signal(|| None::<MouseEventParams>);
    use_context_provider(|| CrosshairState(crosshair));

    let mut handlers = use_signal(ChartEventHandlers::default);
    handlers.set(ChartEventHandlers {
        crosshair_move:               props.on_crosshair_move,
        click:                        props.on_click,
        dbl_click:                    props.on_dbl_click,
        visible_time_range_change:    props.on_visible_time_range_change,
        visible_logical_range_change: props.on_visible_logical_range_change,
    });

    use_hook({
        let chart = chart.clone();

        move || {
            let handlers = move || *handlers.peek();

            [
                keep_subscription(
                    chart.on_crosshair_move(in_component_scope(move |params: MouseEventParams| {
                        if let Some(handler) = handlers().crosshair_move {
                            handler.call(params.clone());
                        }

                        crosshair.set(Some(params));
                    })),
                    "crosshair move",
                ),
                keep_subscription(
                    chart.on_click(in_component_scope(move |params| {
                        if let Some(handler) = handlers().click {
                            handler.call(params);
                        }
                    })),
                    "click",
                ),
                keep_subscription(
                    chart.on_dbl_click(in_component_scope(move |params| {
                        if let Some(handler) = handlers().dbl_click {
                            handler.call(params);
                        }
                    })),
                    "double click",
                ),
                keep_subscription(
                    chart.on_visible_time_range_change(in_component_scope(move |range| {
                        if let Some(handler) = handlers().visible_time_range_change {
                            handler.call(range);
                        }
                    })),
                    "visible time range change",
                ),
                keep_subscription(
                    chart.on_visible_logical_range_change(in_component_scope(move |range| {
                        if let Some(handler) = handlers().visible_logical_range_change {
                            handler.call(range);
                        }
                    })),
                    "visible logical range change",
                ),
            ]
        }
    });

    let mut node = use_signal(|| None::<HtmlDivElement>);

    use_effect({
        let chart = chart.clone();

        move || {
            if let Some(div) = node() {
                if let Err(err) = chart.bind(div) {
                    err.with_prefix("Failed to bind chart").log();
                }
            }
        }
    });

    use_effect(use_reactive((&props.options,), {
        let chart = chart.clone();

        move |(options,)| {
            if let Some(options) = options.as_ref() {
                if let Err(err) = chart.apply_chart_options(options) {
                    err.with_prefix("Failed to apply chart options").log();
                }
            }
        }
    }));

    use_effect(use_reactive((&props.time_zone,), {
        let chart = chart.clone();

        move |(time_zone,)| {
            if let Err(err) = chart.set_time_zone(time_zone.as_deref()) {
                err.with_prefix("Failed to set the time zone").log();
            }
        }
    }));

    use_effect(use_reactive((&props.legend,), {
        let chart = chart.clone();

        move |(legend,)| {
            let res = match legend.as_ref() {
                Some(legend) => chart.apply_legend_options(legend),
                None => chart.remove_legend(),
            };

            if let Err(err) = res {
                err.with_prefix("Failed to update the legend").log();
            }
        }
    }));

    rsx! {
        div {
            style: format!("position:relative;{}", props.style.unwrap_or_default()),
            class: props.class.unwrap_or_default(),
            div {
                style: "position:absolute;inset:0",
                onmounted: move |event| {
                    let element = event.as_web_event();
                    match element.dyn_into::<HtmlDivElement>() {
                        Ok(div) => node.set(Some(div)),
                        Err(element) => {
                            JsError::from_displayable(format!("Mounted element is not a div: {:?}", element)).log()
                        }
                    }
                },
            }
            {props.children}
        }
    }
}

/// The chart handle of the enclosing [`Chart`].
///
/// # Panics
///
/// Panics when called outside of a `Chart`.
pub fn use_chart() -> ChartHandle {
    try_use_context::<ChartHandle>().expect("Chart context not found. Ensure the component is rendered inside <Chart>.")
}

/// Reactive state of the crosshair of the enclosing [`Chart`], updated each time the crosshair moves.
///
/// It is `None` before the first move, and always `None` outside of a `Chart`.
pub fn use_crosshair() -> ReadSignal<Option<MouseEventParams>> {
    match try_use_context::<CrosshairState>() {
        Some(state) => state.0.into(),
        None => use_signal(|| None).into(),
    }
}
