use crate::{
    chart::use_crosshair,
    data::{TooltipMode, TooltipPlacement},
};

use dioxus::prelude::*;

const TOOLTIP_STYLE: &str = "display:block;position:absolute;z-index:3100;pointer-events:none;";
const OVERLAY_STYLE: &str = "position:absolute;z-index:3050;pointer-events:none;";

#[derive(Clone, PartialEq, Props)]
pub struct TooltipProps {
    /// How the tooltip follows the crosshair.
    #[props(default)]
    mode:      TooltipMode,
    /// Series followed by a floating tooltip; the series under the cursor, or else the first series, by default.
    #[props(default, into)]
    series_id: Option<String>,
    /// Distance in pixels between the tooltip and the point it follows. Default: `12`.
    #[props(default = 12.0)]
    offset:    f64,
    #[props(default, into)]
    class:     Option<String>,
    #[props(default, into)]
    style:     Option<String>,
    children:  Element,
}

/// Tooltip following the crosshair of the enclosing `Chart`, which renders its children.
///
/// The children read the crosshair state with [`use_crosshair`](crate::chart::use_crosshair) to show the data under
/// the cursor. The tooltip is hidden when the cursor is outside of the data. It does not capture the mouse events,
/// unless its style sets `pointer-events:auto`.
#[allow(non_snake_case)]
pub fn Tooltip(props: TooltipProps) -> Element {
    let crosshair = use_crosshair();
    let placement = crosshair
        .read()
        .as_ref()
        .and_then(|params| TooltipPlacement::compute(props.mode, params, props.series_id.as_deref(), props.offset));
    let style = match placement {
        Some(placement) => format!("{TOOLTIP_STYLE}{};{}", placement.css(), props.style.unwrap_or_default()),
        None => String::from("display:none"),
    };

    rsx! {
        div {
            class: props.class.unwrap_or_default(),
            style,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ChartOverlayProps {
    #[props(default, into)]
    class:    Option<String>,
    #[props(default, into)]
    style:    Option<String>,
    children: Element,
}

/// Element drawn over the enclosing `Chart`, positioned with its style (for instance `top:8px;left:8px`).
///
/// It is the building block of custom legends: its children can read the crosshair state with
/// [`use_crosshair`](crate::chart::use_crosshair). It does not capture the mouse events, unless its style sets
/// `pointer-events:auto`.
#[allow(non_snake_case)]
pub fn ChartOverlay(props: ChartOverlayProps) -> Element {
    rsx! {
        div {
            class: props.class.unwrap_or_default(),
            style: format!("{OVERLAY_STYLE}{}", props.style.unwrap_or_default()),
            {props.children}
        }
    }
}
