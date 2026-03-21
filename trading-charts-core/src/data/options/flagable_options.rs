use serde::{
    de::{Visitor, Error},
    Deserialize,
    Serialize,
    Deserializer,
    Serializer,
};
use std::fmt::{Formatter, Result as FmtResult};

#[derive(Clone, Default)]
pub enum FlagableOptions<T: Serialize + for<'de> Deserialize<'de> + Clone + Default> {
    #[default]
    None,
    False,
    True,
    Options(T),
}

impl<T: Serialize + for<'de> Deserialize<'de> + Clone + Default> FlagableOptions<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_options(options: T) -> Self {
        Self::Options(options)
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl<T: Serialize + for<'de> Deserialize<'de> + Clone + Default> Serialize for FlagableOptions<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            FlagableOptions::None => serializer.serialize_none(),
            FlagableOptions::False => serializer.serialize_bool(false),
            FlagableOptions::True => serializer.serialize_bool(true),
            FlagableOptions::Options(options) => options.serialize(serializer),
        }
    }
}

impl<'de, T: Serialize + for<'x> Deserialize<'x> + Clone + Default> Deserialize<'de> for FlagableOptions<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(FlagableOptionsVisitor::<T> {
            _marker: std::marker::PhantomData,
        })
    }
}

struct FlagableOptionsVisitor<T: Serialize + for<'de> Deserialize<'de> + Clone + Default> {
    _marker: std::marker::PhantomData<T>,
}

impl<'de, T: Serialize + for<'x> Deserialize<'x> + Clone + Default> Visitor<'de> for FlagableOptionsVisitor<T> {
    type Value = FlagableOptions<T>;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a boolean or an object")
    }

    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(if v {
            FlagableOptions::True
        } else {
            FlagableOptions::False
        })
    }

    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        Ok(FlagableOptions::None)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        Ok(FlagableOptions::Options(T::deserialize(deserializer)?))
    }
}
