use super::TickMarkType;
use crate::{bindings::js, data::UTCTimestamp, JsError};
use wasm_bindgen::closure::Closure;

const PRICE: &str = "price";
const TIME: &str = "time";
const TICK_MARK: &str = "tickMark";

/// Register a Rust function formatting prices, usable with
/// [`PriceFormatter::custom`](super::PriceFormatter::custom) under `name`.
///
/// Registering another function with the same name replaces the previous one, for the charts and series configured
/// afterwards. The function is called from the JS binding each time Lightweight Charts formats a price: it should
/// be fast.
pub fn register_price_formatter(name: &str, formatter: impl Fn(f64) -> String + 'static) -> Result<(), JsError> {
    let closure = Closure::<dyn Fn(f64) -> String>::new(formatter).into_js_value();

    Ok(js::registerCustomFunction(PRICE, name, &closure)?)
}

/// Register a Rust function formatting the crosshair label of the time scale, usable with
/// [`TimeFormatter::custom`](super::TimeFormatter::custom) under `name`.
///
/// The function receives the real UTC time, even when the chart has a time zone.
pub fn register_time_formatter(
    name: &str,
    formatter: impl Fn(UTCTimestamp) -> String + 'static,
) -> Result<(), JsError> {
    let closure =
        Closure::<dyn Fn(f64) -> String>::new(move |seconds: f64| formatter(UTCTimestamp::from_seconds(seconds)))
            .into_js_value();

    Ok(js::registerCustomFunction(TIME, name, &closure)?)
}

/// Register a Rust function formatting the tick marks of the time scale, usable with
/// [`TickMarkFormatter::custom`](super::TickMarkFormatter::custom) under `name`.
///
/// The function receives the real UTC time, even when the chart has a time zone, and the kind of tick mark. It
/// returns `None` to keep the default label of Lightweight Charts.
pub fn register_tick_mark_formatter(
    name: &str,
    formatter: impl Fn(UTCTimestamp, TickMarkType) -> Option<String> + 'static,
) -> Result<(), JsError> {
    let closure = Closure::<dyn Fn(f64, u8) -> Option<String>>::new(move |seconds: f64, kind: u8| {
        TickMarkType::from_index(kind).and_then(|kind| formatter(UTCTimestamp::from_seconds(seconds), kind))
    })
    .into_js_value();

    Ok(js::registerCustomFunction(TICK_MARK, name, &closure)?)
}

/// Unregister the price formatter registered under `name`. Returns `true` when there was one.
pub fn unregister_price_formatter(name: &str) -> bool {
    js::unregisterCustomFunction(PRICE, name)
}

/// Unregister the time formatter registered under `name`. Returns `true` when there was one.
pub fn unregister_time_formatter(name: &str) -> bool {
    js::unregisterCustomFunction(TIME, name)
}

/// Unregister the tick mark formatter registered under `name`. Returns `true` when there was one.
pub fn unregister_tick_mark_formatter(name: &str) -> bool {
    js::unregisterCustomFunction(TICK_MARK, name)
}
