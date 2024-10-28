use serde::{de::Error, Deserialize, Serialize, Deserializer, Serializer};

#[derive(Default, Copy, Clone)]
pub enum OverlayPriceScaleMode {
    #[default]
    Normal       = 0,

    Logarithmic  = 1,

    Percentage   = 2,

    IndexedTo100 = 3,
}

impl Serialize for OverlayPriceScaleMode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for OverlayPriceScaleMode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match u8::deserialize(deserializer)? {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Logarithmic),
            2 => Ok(Self::Percentage),
            3 => Ok(Self::IndexedTo100),
            _ => Err(Error::custom("invalid value for OverlayPriceScaleMode")),
        }
    }
}
