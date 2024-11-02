use serde::{de::Error, Deserialize, Serialize, Deserializer, Serializer};

#[derive(Default, Copy, Clone)]
pub enum LineWidth {
    #[default]
    W1 = 1,

    W2 = 2,

    W3 = 3,

    W4 = 4,
}

impl Serialize for LineWidth {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for LineWidth {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match u8::deserialize(deserializer)? {
            1 => Ok(Self::W1),
            2 => Ok(Self::W2),
            3 => Ok(Self::W3),
            4 => Ok(Self::W4),
            _ => Err(Error::custom("invalid value for LineWidth")),
        }
    }
}
