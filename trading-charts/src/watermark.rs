use crate::{
    data::{ImageWatermarkOptions, TextWatermarkOptions},
    ChartHandle,
    PanelId,
};

use leptos::prelude::*;

/// Text watermark drawn in the enclosing `ChartPanel`, or in the main pane outside of a panel.
///
/// The watermark follows the reactive options, and is removed when the component is unmounted.
#[component]
pub fn TextWatermark(#[prop(into)] options: Signal<TextWatermarkOptions>) -> impl IntoView {
    let Some(chart) = use_context::<ChartHandle>() else {
        return;
    };

    let panel = use_context::<PanelId>().unwrap_or(0);
    let id = match options.with_untracked(|options| chart.add_text_watermark(panel, options)) {
        Ok(id) => id,
        Err(err) => {
            err.with_prefix("Failed to add the text watermark").log();
            return;
        }
    };

    Effect::new({
        let chart = chart.clone();
        let id = id.clone();

        move || {
            options.with(|options| {
                if let Err(err) = chart.update_text_watermark(&id, options) {
                    err.with_prefix("Failed to update the text watermark").log();
                }
            })
        }
    });

    on_cleanup(move || {
        if let Err(err) = chart.remove_watermark(&id) {
            err.with_prefix("Failed to remove the text watermark").log();
        }
    });
}

/// Image watermark drawn in the enclosing `ChartPanel`, or in the main pane outside of a panel.
///
/// The watermark follows the reactive URL and options, and is removed when the component is unmounted.
#[component]
pub fn ImageWatermark(
    /// URL of the image, which can be a data URL.
    #[prop(into)]
    url: Signal<String>,
    #[prop(optional, into)] options: Signal<ImageWatermarkOptions>,
) -> impl IntoView {
    let Some(chart) = use_context::<ChartHandle>() else {
        return;
    };

    let panel = use_context::<PanelId>().unwrap_or(0);
    let current = StoredValue::new(None::<(String, String)>);

    Effect::new({
        let chart = chart.clone();

        move || {
            let url = url.get();
            let res = options.with(|options| match current.get_value() {
                Some((current_url, id)) if current_url == url => chart.update_image_watermark(&id, options),
                previous => {
                    if let Some((_, id)) = previous {
                        chart.remove_watermark(&id)?;
                    }

                    let id = chart.add_image_watermark(panel, &url, options)?;
                    current.set_value(Some((url, id)));

                    Ok(())
                }
            });

            if let Err(err) = res {
                err.with_prefix("Failed to update the image watermark").log();
            }
        }
    });

    on_cleanup(move || {
        if let Some((_, id)) = current.get_value() {
            if let Err(err) = chart.remove_watermark(&id) {
                err.with_prefix("Failed to remove the image watermark").log();
            }
        }
    });
}
