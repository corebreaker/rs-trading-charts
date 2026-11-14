use crate::{
    chart::use_crosshair,
    data::{TooltipMode, TooltipPlacement},
};

use leptos::prelude::*;

const TOOLTIP_STYLE: &str = "display:block;position:absolute;z-index:3100;pointer-events:none;";
const OVERLAY_STYLE: &str = "position:absolute;z-index:3050;pointer-events:none;";

/// Tooltip following the crosshair of the enclosing `Chart`, which renders its children.
///
/// The children read the crosshair state with [`use_crosshair`](crate::chart::use_crosshair) to show the data under
/// the cursor. The tooltip is hidden when the cursor is outside of the data. It does not capture the mouse events,
/// unless its style sets `pointer-events:auto`.
#[component]
pub fn Tooltip(
    /// How the tooltip follows the crosshair.
    #[prop(optional)]
    mode: TooltipMode,
    /// Series followed by a floating tooltip; the series under the cursor, or else the first series, by default.
    #[prop(optional, into)]
    series_id: MaybeProp<String>,
    /// Distance in pixels between the tooltip and the point it follows. Default: `12`.
    #[prop(optional, into)]
    offset: Option<f64>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let crosshair = use_crosshair();
    let offset = offset.unwrap_or(12.0);
    let user_style = style.unwrap_or_default();
    let style = move || {
        let placement = crosshair.with(|params| {
            params
                .as_ref()
                .and_then(|params| TooltipPlacement::compute(mode, params, series_id.get().as_deref(), offset))
        });

        match placement {
            Some(placement) => format!("{TOOLTIP_STYLE}{};{user_style}", placement.css()),
            None => String::from("display:none"),
        }
    };

    view! {
        <div class=class.unwrap_or_default() style=style>
            {children()}
        </div>
    }
}

/// Element drawn over the enclosing `Chart`, positioned with its style (for instance `top:8px;left:8px`).
///
/// It is the building block of custom legends: its children can read the crosshair state with
/// [`use_crosshair`](crate::chart::use_crosshair). It does not capture the mouse events, unless its style sets
/// `pointer-events:auto`.
#[component]
pub fn ChartOverlay(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let style = format!("{OVERLAY_STYLE}{}", style.unwrap_or_default());

    view! {
        <div class=class.unwrap_or_default() style=style>
            {children()}
        </div>
    }
}
