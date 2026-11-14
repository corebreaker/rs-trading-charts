//! Leptos components for [Lightweight Charts](https://github.com/tradingview/lightweight-charts), built on
//! `trading-charts-core`.

pub mod chart;
pub mod overlay;
pub mod panel;
pub mod series;

pub use trading_charts_core::{data, series as series_data, ChartHandle, JsError, PanelId, SeriesHandle, Subscription};

/// Kind of the events which scale the time axis of a chart to fit its data, see the `refit` prop of `Chart`.
pub const REFIT_EVENT_KIND: &str = "trading-charts/refit";
