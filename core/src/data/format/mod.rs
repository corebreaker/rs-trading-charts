//! Formatting and customization of the texts of the charts.
//!
//! Lightweight Charts takes JS functions to format prices and times. The formatters of this module describe
//! predefined behaviors implemented by the JS binding, or refer by name to Rust functions registered with the
//! `register_*` functions.

mod date_format;
mod font;
mod price;
mod registry;
mod time;

pub use self::{
    date_format::DateFormat,
    font::{load_web_font, FontFamily},
    price::{CurrencyDisplay, PriceFormatter},
    registry::{
        register_price_formatter,
        register_tick_mark_formatter,
        register_time_formatter,
        unregister_price_formatter,
        unregister_tick_mark_formatter,
        unregister_time_formatter,
    },
    time::{DateTimeStyle, TickMarkFormatter, TickMarkType, TimeFormatter},
};
