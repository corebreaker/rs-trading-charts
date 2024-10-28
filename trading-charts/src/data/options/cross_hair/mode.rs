use serde::{de::Error, Deserialize, Serialize, Deserializer, Serializer};

#[derive(Default, Copy, Clone)]
pub enum CrosshairMode {
    Normal = 0,

    #[default]
    Magnet = 1,

    Hidden = 2,
}

impl Serialize for CrosshairMode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for CrosshairMode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match u8::deserialize(deserializer)? {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Magnet),
            2 => Ok(Self::Hidden),
            _ => Err(Error::custom("invalid value for CrosshairMode")),
        }
    }
}
