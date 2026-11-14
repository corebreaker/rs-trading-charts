use super::{MarkerPosition, MarkerShape, MarkerType, UTCTimestamp};
use serde::{Deserialize, Serialize};

/// Marker drawn on a series at a time: a shape with an optional text.
///
/// The [type](MarkerType) of the marker gives its style, and the properties set on the marker override the style.
/// A marker without type must give its own properties, the missing ones defaulting to a blue circle above the bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Marker {
    time:     UTCTimestamp,
    #[serde(rename = "type", default)]
    kind:     MarkerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    id:       Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    text:     Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    position: Option<MarkerPosition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    shape:    Option<MarkerShape>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    color:    Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    size:     Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price:    Option<f64>,
}

impl Marker {
    pub fn new(time: UTCTimestamp, kind: MarkerType) -> Self {
        Self {
            time,
            kind,
            id: None,
            text: None,
            position: None,
            shape: None,
            color: None,
            size: None,
            price: None,
        }
    }

    /// Buy signal marker.
    pub fn buy(time: UTCTimestamp) -> Self {
        Self::new(time, MarkerType::Buy)
    }

    /// Sell signal marker.
    pub fn sell(time: UTCTimestamp) -> Self {
        Self::new(time, MarkerType::Sell)
    }

    /// Removal request of the markers at a time, for [`ChartHandle::set_marker`](crate::ChartHandle::set_marker).
    pub fn remove(time: UTCTimestamp) -> Self {
        Self::new(time, MarkerType::Remove)
    }

    /// Marker with the style registered under `style_id` with [`register_marker_style`](super::register_marker_style).
    pub fn custom(time: UTCTimestamp, style_id: impl Into<String>) -> Self {
        Self::new(time, MarkerType::Custom(style_id.into()))
    }

    /// Marker without type, described by its shape, position and color.
    pub fn styled(time: UTCTimestamp, shape: MarkerShape, position: MarkerPosition, color: impl Into<String>) -> Self {
        Self::new(time, MarkerType::None)
            .with_shape(shape)
            .with_position(position)
            .with_color(color)
    }

    /// Identifier of the marker: setting a marker with [`ChartHandle::set_marker`](crate::ChartHandle::set_marker)
    /// replaces the marker with the same identifier, instead of the markers without identifier at the same time.
    pub fn with_id(self, id: impl Into<String>) -> Self {
        Self {
            id: Some(id.into()),
            ..self
        }
    }

    pub fn with_text(self, text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            ..self
        }
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

    /// Price of the marker, required by the `AtPrice*` positions.
    pub fn with_price(self, price: f64) -> Self {
        Self {
            price: Some(price),
            ..self
        }
    }

    /// Place the marker at a price, with one of the `AtPrice*` positions.
    pub fn at_price(self, price: f64, position: MarkerPosition) -> Self {
        self.with_price(price).with_position(position)
    }

    pub fn time(&self) -> UTCTimestamp {
        self.time
    }

    pub fn kind(&self) -> &MarkerType {
        &self.kind
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
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

    pub fn price(&self) -> Option<f64> {
        self.price
    }
}
