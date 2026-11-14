use crate::{
    bindings::{js, optional_value},
    JsError,
};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use std::collections::BTreeMap;

/// Position of a marker.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarkerPosition {
    /// Above the bar.
    AboveBar,

    /// Below the bar.
    BelowBar,

    /// Over the bar.
    InBar,

    /// Above the price of the marker, which must be set.
    AtPriceTop,

    /// Below the price of the marker, which must be set.
    AtPriceBottom,

    /// Centered on the price of the marker, which must be set.
    AtPriceMiddle,
}

/// Shape of a marker.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarkerShape {
    Circle,
    Square,
    ArrowUp,
    ArrowDown,
}

/// Style of a marker: the properties a marker gets from its type.
///
/// The styles are registered with [`register_marker_style`] and used by the markers of type
/// [`MarkerType::Custom`](super::MarkerType::Custom). The properties set on a marker take precedence over its
/// style; the missing properties default to a blue circle above the bar.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkerStyle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    position: Option<MarkerPosition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    shape:    Option<MarkerShape>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    color:    Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    size:     Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    text:     Option<String>,
}

impl MarkerStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_position(self, position: MarkerPosition) -> Self {
        Self {
            position: Some(position),
            ..self
        }
    }

    pub fn with_shape(self, shape: MarkerShape) -> Self {
        Self {
            shape: Some(shape),
            ..self
        }
    }

    /// CSS color of the marker.
    pub fn with_color(self, color: impl Into<String>) -> Self {
        Self {
            color: Some(color.into()),
            ..self
        }
    }

    /// Size multiplier of the marker. Default: `1`.
    pub fn with_size(self, size: f64) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    /// Text shown next to the markers which have no text.
    pub fn with_text(self, text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            ..self
        }
    }

    pub fn position(&self) -> Option<MarkerPosition> {
        self.position
    }

    pub fn shape(&self) -> Option<MarkerShape> {
        self.shape
    }

    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    pub fn size(&self) -> Option<f64> {
        self.size
    }

    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Set or reset (with `None`) the `position` property.
    pub fn set_position(&mut self, position: Option<MarkerPosition>) {
        self.position = position;
    }

    /// Set or reset (with `None`) the `shape` property.
    pub fn set_shape(&mut self, shape: Option<MarkerShape>) {
        self.shape = shape;
    }

    /// Set or reset (with `None`) the `color` property.
    pub fn set_color(&mut self, color: Option<String>) {
        self.color = color;
    }

    /// Set or reset (with `None`) the `size` property.
    pub fn set_size(&mut self, size: Option<f64>) {
        self.size = size;
    }

    /// Set or reset (with `None`) the `text` property.
    pub fn set_text(&mut self, text: Option<String>) {
        self.text = text;
    }
}

/// Register a marker style under an identifier, used by the markers of type `MarkerType::Custom(id)`.
///
/// Registering a style with an existing identifier replaces it; the `buy` and `sell` styles can be overridden this
/// way. The styles are global to the page, and applied when the markers are set on a series: prefixing their
/// identifiers (`"my-chart:flag"`) groups them by chart or by feature, to list or unregister them by prefix.
pub fn register_marker_style(id: &str, style: &MarkerStyle) -> Result<(), JsError> {
    Ok(js::registerMarkerStyle(id, to_value(style)?)?)
}

/// Unregister a marker style. Unregistering the `buy` or `sell` style restores its default. Returns `true` when the
/// style was registered.
pub fn unregister_marker_style(id: &str) -> bool {
    js::unregisterMarkerStyle(id)
}

/// Identifiers of all the registered marker styles, sorted, including the built-in `buy` and `sell` styles.
pub fn marker_style_ids() -> Vec<String> {
    js::listMarkerStyleIds("")
}

/// Identifiers of the registered marker styles starting with `prefix`, sorted.
///
/// The styles are global to the page: prefixing their identifiers (`"my-chart:flag"`) groups them by chart or by
/// feature.
pub fn marker_style_ids_with_prefix(prefix: &str) -> Vec<String> {
    js::listMarkerStyleIds(prefix)
}

/// The marker style registered under `id`, `None` when there is none.
pub fn marker_style(id: &str) -> Result<Option<MarkerStyle>, JsError> {
    optional_value(js::getMarkerStyle(id))
}

/// The marker styles whose identifier starts with `prefix`, indexed by identifier. An empty prefix returns all the
/// styles.
pub fn marker_styles_with_prefix(prefix: &str) -> Result<BTreeMap<String, MarkerStyle>, JsError> {
    Ok(from_value(js::getMarkerStyles(prefix))?)
}

/// Unregister the marker styles whose identifier starts with `prefix`, which must not be empty, and return how many
/// were removed. The built-in `buy` and `sell` styles matching the prefix are restored to their default, and counted
/// when they were overridden.
pub fn unregister_marker_styles_with_prefix(prefix: &str) -> Result<usize, JsError> {
    Ok(js::unregisterMarkerStylesWithPrefix(prefix)? as usize)
}

/// Unregister all the custom marker styles and restore the default `buy` and `sell` styles.
pub fn clear_marker_styles() {
    js::clearMarkerStyles()
}

/// Order in which the markers are drawn, relatively to the series.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarkerZOrder {
    /// Above everything, including the other series.
    Top,

    /// Above the series of the markers.
    AboveSeries,

    /// Below the series of the markers.
    #[default]
    Normal,
}

/// Options of the markers of a series.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesMarkersOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    auto_scale: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    z_order:    Option<MarkerZOrder>,
}

impl SeriesMarkersOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Include the markers in the automatic scaling of the price scale. Default: `true`.
    pub fn with_auto_scale(self, auto_scale: bool) -> Self {
        Self {
            auto_scale: Some(auto_scale),
            ..self
        }
    }

    /// Drawing order of the markers. Default: [`MarkerZOrder::Normal`].
    pub fn with_z_order(self, z_order: MarkerZOrder) -> Self {
        Self {
            z_order: Some(z_order),
            ..self
        }
    }

    pub fn auto_scale(&self) -> Option<bool> {
        self.auto_scale
    }

    pub fn z_order(&self) -> Option<MarkerZOrder> {
        self.z_order
    }

    /// Set or reset (with `None`) the `autoScale` option.
    pub fn set_auto_scale(&mut self, auto_scale: Option<bool>) {
        self.auto_scale = auto_scale;
    }

    /// Set or reset (with `None`) the `zOrder` option.
    pub fn set_z_order(&mut self, z_order: Option<MarkerZOrder>) {
        self.z_order = z_order;
    }
}
