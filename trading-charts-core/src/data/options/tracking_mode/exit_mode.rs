use serde::{de::Error, Deserialize, Serialize, Deserializer, Serializer};

#[derive(Default, Copy, Clone)]
pub enum TrackingModeExitMode {
    #[default]
    OnTouchEnd = 0,

    OnNextTap  = 1,
}

impl Serialize for TrackingModeExitMode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for TrackingModeExitMode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match u8::deserialize(deserializer)? {
            0 => Ok(Self::OnTouchEnd),
            1 => Ok(Self::OnNextTap),
            _ => Err(Error::custom("invalid value for TrackingModeExitMode")),
        }
    }
}
