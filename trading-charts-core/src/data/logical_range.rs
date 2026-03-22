use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct LogicalRange {
    from: f64,
    to:   f64,
}

impl LogicalRange {
    pub fn new(from: f64, to: f64) -> Self {
        Self {
            from,
            to,
        }
    }

    pub fn from(&self) -> f64 {
        self.from
    }

    pub fn to(&self) -> f64 {
        self.to
    }
}
