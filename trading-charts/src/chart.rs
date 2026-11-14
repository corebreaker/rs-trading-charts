use crate::{
    data::{options::ChartOptions, LegendOptions, LogicalRange, MouseEventParams, SyncOptions, TimeRange},
    ChartHandle,
    JsError,
    Subscription,
    REFIT_EVENT_KIND,
};

use emitix::{leptos::LeptosEventChannels, EventManager};
use leptos::{html::Div, prelude::*};

/// Crosshair state of a chart, provided to the children of [`Chart`].
#[derive(Clone, Copy)]
struct CrosshairState(Signal<Option<MouseEventParams>>);

/// Reactive state of the crosshair of the enclosing [`Chart`], updated each time the crosshair moves.
///
/// It is `None` before the first move. Outside of a `Chart`, the returned signal is always `None`.
pub fn use_crosshair() -> Signal<Option<MouseEventParams>> {
    use_context::<CrosshairState>()
        .map(|state| state.0)
        .unwrap_or_else(|| Signal::stored(None))
}

/// The chart handle of the enclosing [`Chart`], `None` outside of a chart.
pub fn use_chart() -> Option<ChartHandle> {
    use_context::<ChartHandle>()
}

fn make_chart(options: Option<Signal<ChartOptions>>) -> Result<ChartHandle, JsError> {
    let Some(options) = options else {
        return ChartHandle::new(None);
    };

    let chart = options.with_untracked(|options| ChartHandle::new(Some(options)))?;
    Effect::new({
        let chart = chart.clone();

        move || {
            options.with(|options| {
                if let Err(err) = chart.apply_chart_options(options) {
                    err.with_prefix("Failed to apply chart options").log();
                }
            })
        }
    });

    Ok(chart)
}

fn keep_subscription(subscription: Result<Subscription, JsError>, event: &str) {
    match subscription {
        Ok(subscription) => on_cleanup(move || drop(subscription)),
        Err(err) => err
            .with_prefix(format!("Failed to subscribe to the {event} event"))
            .log(),
    }
}

/// Chart, which renders its series, panes, tooltips and overlays given as children.
///
/// The chart fills a `div` sized by the `style` and `class` props, which is also the positioning context of the
/// overlays: give it a size, for instance with `style="width:100%;height:400px"`, and set the `autoSize` chart
/// option to follow it.
///
/// The children can access the chart with [`use_chart`] and the crosshair state with [`use_crosshair`].
#[component]
pub fn Chart(
    /// Options of the chart, applied again when they change.
    #[prop(optional, into)]
    options: Option<Signal<ChartOptions>>,
    /// Options of the built-in legend, which is removed with `None`.
    #[prop(optional, into)]
    legend: Option<Signal<Option<LegendOptions>>>,
    /// IANA time zone in which the times are displayed, such as `Europe/Paris`; UTC by default.
    #[prop(optional, into)]
    time_zone: MaybeProp<String>,
    /// Synchronization group of the chart: the charts of a group share their crosshair and visible range.
    #[prop(optional, into)]
    sync_group: MaybeProp<String>,
    /// What is synchronized with the other charts of the synchronization group.
    #[prop(optional, into)]
    sync_options: Option<Signal<SyncOptions>>,
    #[prop(optional, into)] style: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    /// Event channels on which a `REFIT_EVENT_KIND` event scales the time axis to fit the data.
    #[prop(optional)]
    refit: Option<LeptosEventChannels>,
    #[prop(optional, into)] on_crosshair_move: Option<Callback<MouseEventParams>>,
    #[prop(optional, into)] on_click: Option<Callback<MouseEventParams>>,
    #[prop(optional, into)] on_dbl_click: Option<Callback<MouseEventParams>>,
    #[prop(optional, into)] on_visible_time_range_change: Option<Callback<Option<TimeRange>>>,
    #[prop(optional, into)] on_visible_logical_range_change: Option<Callback<Option<LogicalRange>>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let chart = match make_chart(options) {
        Ok(chart) => chart,
        Err(err) => {
            err.with_prefix("Failed to create chart").log();

            return ().into_any();
        }
    };

    let crosshair = RwSignal::new(None::<MouseEventParams>);
    keep_subscription(
        chart.on_crosshair_move(move |params| {
            if let Some(callback) = on_crosshair_move {
                callback.run(params.clone());
            }

            crosshair.set(Some(params));
        }),
        "crosshair move",
    );

    if let Some(callback) = on_click {
        keep_subscription(chart.on_click(move |params| callback.run(params)), "click");
    }

    if let Some(callback) = on_dbl_click {
        keep_subscription(chart.on_dbl_click(move |params| callback.run(params)), "double click");
    }

    if let Some(callback) = on_visible_time_range_change {
        keep_subscription(
            chart.on_visible_time_range_change(move |range| callback.run(range)),
            "visible time range change",
        );
    }

    if let Some(callback) = on_visible_logical_range_change {
        keep_subscription(
            chart.on_visible_logical_range_change(move |range| callback.run(range)),
            "visible logical range change",
        );
    }

    Effect::new({
        let chart = chart.clone();

        move || {
            if let Err(err) = chart.set_time_zone(time_zone.get().as_deref()) {
                err.with_prefix("Failed to set the time zone").log();
            }
        }
    });

    Effect::new({
        let chart = chart.clone();

        move || {
            let options = sync_options.map(|options| options.get()).unwrap_or_default();

            if let Err(err) = chart.set_sync_group(sync_group.get().as_deref(), &options) {
                err.with_prefix("Failed to set the synchronization group").log();
            }
        }
    });

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
    Effect::new({
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

        Effect::new(move || {
            legend.with(|legend| {
                let res = match legend {
                    Some(legend) => chart.apply_legend_options(legend),
                    None => chart.remove_legend(),
                };

                if let Err(err) = res {
                    err.with_prefix("Failed to update the legend").log();
                }
            })
        });
    }

    let style = format!("position:relative;{}", style.unwrap_or_default());
    let class = class.unwrap_or_default();

    provide_context(chart);
    provide_context(CrosshairState(crosshair.into()));

    view! {
        <div style=style class=class>
            <div style="position:absolute;inset:0" node_ref=node_ref />
            {children.map(|children| children())}
        </div>
    }
    .into_any()
}
