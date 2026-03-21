use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Default, Copy, Clone)]
pub enum LastPriceAnimationMode {
    #[default]
    Disabled     = 0,

    Continuous   = 1,

    OnDataUpdate = 2,
}

impl Serialize for LastPriceAnimationMode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for LastPriceAnimationMode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match u8::deserialize(deserializer)? {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::Continuous),
            2 => Ok(Self::OnDataUpdate),
            _ => Err(Error::custom("invalid value for LastPriceAnimationMode")),
        }
    }
}
