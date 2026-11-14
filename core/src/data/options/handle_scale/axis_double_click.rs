use serde::{Deserialize, Serialize};

/// Represents options for how the time and price axes react to mouse double click.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisDoubleClickOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    time:  Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price: Option<bool>,
}

impl AxisDoubleClickOptions {
    /// Create options where every option keeps its default value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable resetting scaling the time axis by double-clicking the left mouse button.
    ///
    /// Default: `true`.
    pub fn with_time(self, time: bool) -> Self {
        Self {
            time: Some(time),
            ..self
        }
    }

    pub fn time(&self) -> Option<bool> {
        self.time
    }

    /// Set or reset (with `None`) the `time` option.
    pub fn set_time(&mut self, time: Option<bool>) {
        self.time = time;
    }

    /// Enable reseting scaling the price axis by by double-clicking the left mouse button.
    ///
    /// Default: `true`.
    pub fn with_price(self, price: bool) -> Self {
        Self {
            price: Some(price),
            ..self
        }
    }

    pub fn price(&self) -> Option<bool> {
        self.price
    }

    /// Set or reset (with `None`) the `price` option.
    pub fn set_price(&mut self, price: Option<bool>) {
        self.price = price;
    }
}
