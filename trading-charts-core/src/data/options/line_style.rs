use serde::{de::Error, Deserialize, Serialize, Deserializer, Serializer};

#[derive(Default, Copy, Clone)]
pub enum LineStyle {
    #[default]
    Solid        = 0,

    Dotted       = 1,

    Dashed       = 2,

    LargeDashed  = 3,

    SparseDotted = 4,
}

impl Serialize for LineStyle {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for LineStyle {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match u8::deserialize(deserializer)? {
            0 => Ok(Self::Solid),
            1 => Ok(Self::Dotted),
            2 => Ok(Self::Dashed),
            3 => Ok(Self::LargeDashed),
            4 => Ok(Self::SparseDotted),
            _ => Err(Error::custom("invalid value for LineStyle")),
        }
    }
}
