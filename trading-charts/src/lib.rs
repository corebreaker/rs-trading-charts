#[doc(hidden)]
mod bindings;
mod error;

pub mod chart;
pub mod data;
pub mod panel;
pub mod series;

pub use error::JsError;

pub const REFIT_EVENT_KIND: &str = "trading-charts/refit";
