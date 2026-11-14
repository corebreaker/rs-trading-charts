// Entry point of the JS binding, bundled by esbuild into a single ES module inlined in the wasm module.
//
// Every export of this file must have a matching `extern "C"` declaration in `src/bindings/js.rs`.

export { TradingChart } from './chart.mjs';
export { loadWebFont } from './fonts.mjs';
export { registerCustomFunction, unregisterCustomFunction } from './formatters.mjs';
export { registerMarkerStyle, unregisterMarkerStyle } from './markers.mjs';
