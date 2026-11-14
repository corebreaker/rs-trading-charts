use crate::{
    chart::use_chart,
    data::{ImageWatermarkOptions, TextWatermarkOptions},
    PanelId,
};

use dioxus::prelude::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, PartialEq, Props)]
pub struct TextWatermarkProps {
    options: TextWatermarkOptions,
}

/// Text watermark drawn in the enclosing `ChartPanel`, or in the main pane outside of a panel.
///
/// The watermark is updated when its options change, and removed when the component is dropped.
#[allow(non_snake_case)]
pub fn TextWatermark(props: TextWatermarkProps) -> Element {
    let chart = use_chart();
    let panel = try_use_context::<PanelId>().unwrap_or(0);
    let id = use_hook({
        let chart = chart.clone();
        let options = props.options.clone();

        move || match chart.add_text_watermark(panel, &options) {
            Ok(id) => Some(id),
            Err(err) => {
                err.with_prefix("Failed to add the text watermark").log();
                None
            }
        }
    });

    use_effect(use_reactive((&props.options,), {
        let chart = chart.clone();
        let id = id.clone();

        move |(options,)| {
            if let Some(id) = &id {
                if let Err(err) = chart.update_text_watermark(id, &options) {
                    err.with_prefix("Failed to update the text watermark").log();
                }
            }
        }
    }));

    use_drop(move || {
        if let Some(id) = id {
            if let Err(err) = chart.remove_watermark(&id) {
                err.with_prefix("Failed to remove the text watermark").log();
            }
        }
    });

    rsx! {}
}

#[derive(Clone, PartialEq, Props)]
pub struct ImageWatermarkProps {
    /// URL of the image, which can be a data URL.
    #[props(into)]
    url:     String,
    #[props(default)]
    options: ImageWatermarkOptions,
}

/// Image watermark drawn in the enclosing `ChartPanel`, or in the main pane outside of a panel.
///
/// The watermark is updated when its URL or options change, and removed when the component is dropped.
#[allow(non_snake_case)]
pub fn ImageWatermark(props: ImageWatermarkProps) -> Element {
    let chart = use_chart();
    let panel = try_use_context::<PanelId>().unwrap_or(0);
    let current = use_hook(|| Rc::new(RefCell::new(None::<(String, String)>)));

    use_effect(use_reactive((&props.url, &props.options), {
        let chart = chart.clone();
        let current = current.clone();

        move |(url, options)| {
            let previous = current.borrow_mut().take();
            let res = match previous {
                Some((current_url, id)) if current_url == url => {
                    let res = chart.update_image_watermark(&id, &options);
                    current.replace(Some((current_url, id)));
                    res
                }
                previous => {
                    if let Some((_, id)) = previous {
                        if let Err(err) = chart.remove_watermark(&id) {
                            err.with_prefix("Failed to remove the image watermark").log();
                        }
                    }

                    chart.add_image_watermark(panel, &url, &options).map(|id| {
                        current.replace(Some((url, id)));
                    })
                }
            };

            if let Err(err) = res {
                err.with_prefix("Failed to update the image watermark").log();
            }
        }
    }));

    use_drop(move || {
        if let Some((_, id)) = current.borrow_mut().take() {
            if let Err(err) = chart.remove_watermark(&id) {
                err.with_prefix("Failed to remove the image watermark").log();
            }
        }
    });

    rsx! {}
}
