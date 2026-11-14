use super::chart::ChartInner;
use std::sync::Weak;
use wasm_bindgen::{closure::Closure, JsValue};

/// Subscription to an event of a chart, returned by the `ChartHandle::on_*` methods.
///
/// The callback is called until the subscription is dropped or [cancelled](Subscription::cancel). The subscription
/// does not keep the chart alive.
#[must_use = "the subscription is cancelled when it is dropped"]
pub struct Subscription {
    chart:    Weak<ChartInner>,
    id:       String,
    _closure: Closure<dyn FnMut(JsValue)>,
}

// SAFETY: the WebAssembly target the subscription is built for is single-threaded, as for `ChartHandle`.
unsafe impl Send for Subscription {}
unsafe impl Sync for Subscription {}

impl Subscription {
    pub(super) fn new(chart: Weak<ChartInner>, id: String, closure: Closure<dyn FnMut(JsValue)>) -> Self {
        Self {
            chart,
            id,
            _closure: closure,
        }
    }

    /// Cancel the subscription.
    pub fn cancel(self) {}
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if let Some(chart) = self.chart.upgrade() {
            chart.js.unsubscribe(&self.id);
        }
    }
}
