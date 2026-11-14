use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Type of a [`Marker`](super::Marker), which selects the style of the marker.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MarkerType {
    /// No style: the marker is fully described by its own properties.
    #[default]
    None,

    /// Buy signal: green arrow up below the bar, with the `B` text.
    Buy,

    /// Sell signal: red arrow down above the bar, with the `S` text.
    Sell,

    /// Removal request, used with [`ChartHandle::set_marker`](crate::ChartHandle::set_marker) to remove the markers
    /// at a time.
    Remove,

    /// Style registered with [`register_marker_style`](super::register_marker_style) under this identifier.
    Custom(String),
}

impl MarkerType {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Identifier of the type, as sent to the JS binding.
    pub fn id(&self) -> &str {
        match self {
            Self::None => "",
            Self::Buy => "buy",
            Self::Sell => "sell",
            Self::Remove => "remove",
            Self::Custom(id) => id,
        }
    }
}

impl From<&str> for MarkerType {
    fn from(id: &str) -> Self {
        match id {
            "" => Self::None,
            "buy" => Self::Buy,
            "sell" => Self::Sell,
            "remove" => Self::Remove,
            id => Self::Custom(id.to_string()),
        }
    }
}

impl Serialize for MarkerType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.id())
    }
}

impl<'de> Deserialize<'de> for MarkerType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(deserializer)?.as_str()))
    }
}
