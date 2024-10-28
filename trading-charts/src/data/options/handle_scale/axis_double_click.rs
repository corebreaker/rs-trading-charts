use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AxisDoubleClickOptions {
    #[serde(default = "defaults::time")]
    time: bool,

    #[serde(default = "defaults::price")]
    price: bool,
}

impl AxisDoubleClickOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_params(time: bool, price: bool) -> Self {
        Self {
            time,
            price,
        }
    }

    pub fn with_time(self, time: bool) -> Self {
        Self {
            time,
            ..self
        }
    }

    pub fn with_price(self, price: bool) -> Self {
        Self {
            price,
            ..self
        }
    }

    pub fn time(&self) -> bool {
        self.time
    }

    pub fn set_time(&mut self, time: bool) {
        self.time = time;
    }

    pub fn price(&self) -> bool {
        self.price
    }

    pub fn set_price(&mut self, price: bool) {
        self.price = price;
    }
}

impl Default for AxisDoubleClickOptions {
    fn default() -> Self {
        Self {
            time:  defaults::time(),
            price: defaults::price(),
        }
    }
}

mod defaults {
    pub(super) fn time() -> bool {
        true
    }

    pub(super) fn price() -> bool {
        true
    }
}
