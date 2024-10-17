use serde::{de::Error, Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkerType {
    #[default]
    None,
    Buy,
    Sell,
    Remove,
}

impl MarkerType {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl Serialize for MarkerType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str(""),
            Self::Buy => serializer.serialize_str("buy"),
            Self::Sell => serializer.serialize_str("sell"),
            Self::Remove => serializer.serialize_str("remove"),
        }
    }
}

impl<'de> Deserialize<'de> for MarkerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            "" => Ok(Self::None),
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            "remove" => Ok(Self::Remove),
            _ => Err(Error::custom("invalid marker type")),
        }
    }
}
