use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Default, Copy, Clone)]
pub enum LineType {
    #[default]
    Simple    = 0,

    WithSteps = 1,

    Curved    = 2,
}

impl Serialize for LineType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for LineType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match u8::deserialize(deserializer)? {
            0 => Ok(Self::Simple),
            1 => Ok(Self::WithSteps),
            2 => Ok(Self::Curved),
            _ => Err(Error::custom("invalid value for LineType")),
        }
    }
}
