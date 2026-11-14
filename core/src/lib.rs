#![doc = include_str!("../README.md")]

mod bindings;
mod error;

pub mod data;
pub mod series;

pub use bindings::{ChartHandle, PanelId, SeriesHandle, Subscription};
pub use error::JsError;
