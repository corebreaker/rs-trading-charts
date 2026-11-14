use super::{
    handle_scale::{AxisDoubleClickOptions, AxisPressedMouseMoveOptions, HandleScaleOptions},
    HandleScrollOptions,
};
use serde::{Deserialize, Serialize};

/// Option which is either a flag enabling or disabling a whole feature, or the detailed options of the feature.
///
/// It converts from a `bool` and from the detailed options, so that builders accept both.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FlagableOptions<T> {
    /// Enable (`true`) or disable (`false`) the whole feature.
    Flag(bool),

    /// Detailed options of the feature.
    Options(T),
}

impl<T> FlagableOptions<T> {
    pub fn enabled() -> Self {
        Self::Flag(true)
    }

    pub fn disabled() -> Self {
        Self::Flag(false)
    }

    pub fn options(&self) -> Option<&T> {
        match self {
            Self::Flag(_) => None,
            Self::Options(options) => Some(options),
        }
    }
}

impl<T> From<bool> for FlagableOptions<T> {
    fn from(flag: bool) -> Self {
        Self::Flag(flag)
    }
}

macro_rules! from_options {
    ($($options:ty),+) => {
        $(
            impl From<$options> for FlagableOptions<$options> {
                fn from(options: $options) -> Self {
                    Self::Options(options)
                }
            }
        )+
    };
}

from_options!(
    HandleScrollOptions,
    HandleScaleOptions,
    AxisPressedMouseMoveOptions,
    AxisDoubleClickOptions
);
