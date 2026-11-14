# Plan to generalize `rs-trading-charts` for Dioxus

## Status

The plan is implemented as of version 1.0. This document keeps the rationale of the core/adapter split,
and records how each part of the plan was carried out.

- `trading-charts-core`, in the `core` directory, holds the JS binding, the data types and the runtime handles:
  `ChartHandle`, `PanelId`, `SeriesHandle` and `Subscription`.
- `trading-charts` (Leptos) and `trading-charts-dioxus` (Dioxus web), in the `leptos` and `dioxus` directories,
  are thin adapters on the core.
  The panels are explicit `PanelId` contexts, the series are created once and removed when their component is
  unmounted, and the Dioxus series update the chart only when a prop value changes.
- `examples/leptos` and `examples/dioxus` are the same gallery of demos, and each crate has its own README.

Two items of the plan are only partly done, see [Open items](#open-items):
there is no automated wasm test, and the behavior is validated by hand with the examples.

## Summary

The binding supports Leptos and Dioxus from the same repository, sharing all of the JS/WASM and data-model code.

The shared part is the chart runtime and the `lightweight-charts` binding.
The UI/component layer is not shared: the Leptos API is framework-specific,
and Dioxus has different render and mount semantics.

The decisions were:

- Keep one repository and one shared JS binding pipeline.
- Extract a framework-agnostic core crate.
- Keep the `trading-charts` crate as the Leptos adapter, for backward compatibility.
- Add a `trading-charts-dioxus` crate for Dioxus web support.
- Do not target Dioxus desktop, mobile or liveview: only `wasm32-unknown-unknown` and the Dioxus `web` renderer.

## Meaning of "both frameworks at the same time"

Making the binding work for Leptos and Dioxus at the same time has several meanings:

- Same repository/workspace: yes, this is the chosen layout.
- Same shared Rust/JS core: yes, this was the main design goal.
- Same published crate with feature flags only: technically possible, but not the cleanest API surface.
- Same application running both frameworks together: possible in separate DOM roots,
  but not a design target for this library.

The browser-facing binding is shared, while the component shell is framework-specific.

## Repository layout

Before this work, a single `trading-charts` crate held everything, and its example was in `example`.
The framework-neutral code moved to the core crate, the Leptos-only code stayed in the Leptos crate:

| Before                                           | After                                                         |
|--------------------------------------------------|---------------------------------------------------------------|
| `trading-charts/src/bindings/binding.rs`         | `core/src/bindings/chart.rs`, `series.rs`, `subscription.rs`  |
| `trading-charts/src/bindings/js.rs`              | `core/src/bindings/js.rs`                                     |
| `trading-charts/src/data/**`                     | `core/src/data/**`, `core/src/series/*/options.rs`            |
| `trading-charts/src/error.rs`                    | `core/src/error.rs`                                           |
| `trading-charts/build.rs`                        | `core/build.rs`                                               |
| `trading-charts/bindings/**`                     | `core/bindings/**`                                            |
| `trading-charts/src/chart.rs`, `panel.rs`        | `leptos/src/chart.rs`, `panel.rs`                             |
| `trading-charts/src/series/*/component.rs`       | `leptos/src/series/*/component.rs`, `leptos/src/series/mount.rs` |
| `example/**`                                     | `examples/leptos/**`                                          |
| (none)                                           | `dioxus/**`, `examples/dioxus/**`                             |

The workspace members are `core`, `leptos`, `dioxus`, `wasmbind-macro` and the two examples;
`default-members` excludes the examples.

## Problem model

### Relevant entities

- The `lightweight-charts` JS module.
- The wasm-bindgen Rust bridge, in `core/src/bindings/**`.
- The shared chart data types, in `core/src/data/**`.
- The chart runtime that owns the JS chart object: `ChartHandle`, and the `TradingChart` class on the JS side.
- The framework adapters for Leptos and Dioxus.
- A DOM mount point, represented as `web_sys::HtmlDivElement`.
- The panel and series handles: `PanelId` and `SeriesHandle`.

### State variables

- Chart lifecycle state: created, bound to DOM, destroyed.
- Chart options state.
- Series registry state.
- Panel assignment state.
- Mounted DOM node availability.
- Framework reactive state for data, options and markers.

### Actions

- Create the chart runtime.
- Bind the chart runtime to a mounted DOM node.
- Apply the chart options, before or after binding.
- Allocate panel identifiers.
- Add or remove series.
- Update series options, data and markers.
- Destroy the chart and clean up the series on unmount.

### Constraints

- `lightweight-charts` requires a real browser DOM node.
- The shared binding depends on `wasm-bindgen` and `web-sys`, so it remains a web/WASM integration.
- Dioxus only matches this path on the `web` renderer.
- The Leptos API should remain usable, or have a clear compatibility path.
- The public API must not leak framework lifecycle assumptions into the shared core.

## Architecture

### Crates

The workspace uses a core-plus-adapters layout:

- `trading-charts-core` (`core`): the JS build pipeline, the wasm-bindgen layer, the data types and the handles.
- `trading-charts` (`leptos`): the Leptos adapter, which keeps the Leptos-facing API.
- `trading-charts-dioxus` (`dioxus`): the Dioxus web adapter, with components and hooks around the core.
- `wasmbind-js-file-macro` (`wasmbind-macro`): unchanged.
- `examples/leptos` and `examples/dioxus`: the same gallery of demos, written with each adapter.

### Why not a single dual-feature crate

A single crate with `leptos` and `dioxus` features would work, but it separates the frameworks less well:

- Docs are harder to navigate.
- Public modules become feature-gated and noisy.
- Component lifecycle differences still force a runtime abstraction.
- Backward compatibility is easier if `trading-charts` remains the Leptos crate users already know.

A user-facing meta-crate can still be added later, on top of the core and the adapters.

## Key design change: no ambient panel state

This was the most important refactor for a clean Dioxus integration.

The JS bridge tracked a `_currentPanelId`, and `ChartPanel` mutated that state while its children rendered.
That worked as a render-time DSL, but not as an abstraction shared by several frameworks.

Panel assignment is now explicit:

- `ChartHandle::allocate_panel` allocates a `PanelId` from a counter kept by the handle.
- `ChartPanel` passes its `PanelId` to its children through the framework context.
- A series is added with its panel, set in its `Series` description (`Series::set_panel`);
  the JS `addPanel()` and `removePanel()` methods are gone.
- Series are explicit handles, so that unmount cleanup removes them with `SeriesHandle::remove`.

## Crate responsibilities

### `trading-charts-core`

The core crate holds the bindings, the data types, the error type, the build script and the JS sources,
and has no dependency on Leptos, Dioxus or `emitix`.

It exposes the runtime types `ChartHandle`, `PanelId`, `SeriesHandle`, `Subscription` and `JsError`,
with the data, option and marker types. The methods planned on `ChartHandle` all exist:
`new`, `bind`, `apply_chart_options`, `allocate_panel`, `add_series`, `update_series_options`, `update_data`,
`set_markers`, `refit_content` and `remove_series`.
`SeriesHandle` offers the same series operations without the series identifier.

### `trading-charts` Leptos adapter

The Leptos crate:

- Depends on `trading-charts-core`, and re-exports its types (`data`, `series_data`, `ChartHandle`...).
- Keeps `Chart`, `ChartPanel` and the series components as Leptos components:
  `CandleStickSeries`, and also the bar, line, area, baseline, histogram and price/volume series.
- Keeps the Leptos-specific conveniences: the `refit` prop of `Chart`, with `REFIT_EVENT_KIND` and `emitix`.
- Uses explicit panel and series handles: `ChartPanel` provides a `PanelId` context,
  and the series are removed from the chart in `on_cleanup` (`leptos/src/series/mount.rs`).

### `trading-charts-dioxus`

The Dioxus crate:

- Depends on `trading-charts-core`, and re-exports it like the Leptos crate.
- Depends on Dioxus web features only: `dioxus` without default features, with `lib` and `web`,
  and `dioxus-web` with `mounted`.
- Binds the chart in the `onmounted` handler of its `div`, casting the web element to `HtmlDivElement`.
- Provides the chart handle and the panel identifier with `use_context_provider`.
- Creates the series once with `use_hook`, updates them with `use_effect` and `use_reactive`
  when a prop value changes, and removes them in `use_drop` (`dioxus/src/series/mount.rs`).

## Dioxus-specific implementation notes

### DOM binding

The Dioxus `Chart` binds only after mount: the `onmounted` handler gets the web element of the `div`,
casts it to `web_sys::HtmlDivElement`, and an effect calls `ChartHandle::bind` with it.

### State and rerender safety

Dioxus function components re-run on render. Because of that:

- `add_series` is never called in the render body.
- The chart and series handles are long-lived, created in hooks.
- Effects only perform updates.
- `use_drop` removes the series.

This is why the Leptos component code could not be copied line by line.

### JS callbacks

The JS callbacks (events, crosshair) run Rust closures outside of any Dioxus scope:
they run them in the component scope with `Runtime::in_scope`, so that signals and contexts work.

### Styles

Dioxus merges style declarations, so the styles toggling visibility set `display` explicitly.

## Migration plan

### Phase 1: extract the core — done

Steps:

- Add `trading-charts-core` to the workspace.
- Move the bindings, the data types, the error type, the build script and the JS build assets into the core crate.
- Make the runtime types public instead of `pub(crate)`.
- Add explicit `remove_series` support.
- Replace ambient panel mutation with explicit panel identifiers.

Exit criterion: a small wasm test or example creates a chart runtime, binds it to an `HtmlDivElement`,
adds a series, updates it and destroys it without Leptos.
The example of the core README does this, and is compiled by `cargo make test`; it is not run in a browser.

### Phase 2: refactor the Leptos adapter onto the core — done

Steps:

- Make the `trading-charts` crate depend on `trading-charts-core`.
- Keep the public Leptos component names.
- Re-export the shared data types from the Leptos crate.
- Make `ChartPanel` provide a `PanelId` context instead of mutating a shared "current panel".
- Make the series components hold explicit series handles, and clean them up on unmount.

Exit criterion: the Leptos example works with the core-backed implementation.

### Phase 3: add the Dioxus adapter — done

Steps:

- Add `trading-charts-dioxus` to the workspace.
- Implement `Chart`, `ChartPanel` and the series components in Dioxus terms.
- Bind the chart on `onmounted`.
- Provide the chart and panel contexts with Dioxus context hooks.
- Create the series once, and update them through `use_effect`.
- Add a Dioxus web example, with the same demos as the Leptos example.

Exit criterion: the Dioxus example mounts the chart, switches datasets (the interval selector of the price and volume
demo resamples the data), updates markers and refits the content (the "All" range preset).

The adapters also gained features beyond the plan, in both frameworks:
`PriceVolumeSeries`, `Tooltip` and `ChartOverlay`, `TextWatermark` and `ImageWatermark`.

### Phase 4: documentation and publishability — done

Steps:

- Update the root README to describe the workspace layout.
- Document that the shared binding works for Leptos and Dioxus web.
- Document that the Dioxus desktop, mobile and liveview renderers are not supported.
- Add READMEs for `trading-charts-core` and `trading-charts-dioxus`; the READMEs are also the crate docs.
- Decide whether `trading-charts` remains the Leptos crate name: it does.

Exit criterion: a user can tell which crate to use for Leptos or Dioxus without reading the source.

## Compatibility and naming

The least disruptive naming was chosen:

- `trading-charts` remains the Leptos crate.
- `trading-charts-core` is added.
- `trading-charts-dioxus` is added.

That avoids a breaking change for the users of `trading-charts`.
Renaming it to `trading-charts-leptos` would have been sound too, but as a larger API migration.

## Testing and validation

The checks run by `cargo make verify`:

- `cargo make check-wasm`: `cargo check --workspace --target wasm32-unknown-unknown`, which covers the three crates.
- `cargo make clippy`: clippy on the workspace, on the same target.
- `cargo make test`: the tests of the core crate, and the README examples of the three library crates.
- `cargo make build-examples`: Trunk builds of both examples.

Behavior to validate in both frameworks, by running the examples in a browser
(`cargo make serve-example` and `cargo make serve-example-dioxus`):

- initial chart render
- data update
- marker update
- options update
- refit action
- component unmount/remount (switching the demo tabs)
- multiple series
- multiple panels

## Open items

- **Automated wasm test**: no `wasm-bindgen-test` runs the core in a headless browser.
  Such a test would turn the exit criterion of phase 1 into an automated check.
- **Automated behavior validation**: the behavior listed above is validated by hand with the examples.
  Driving the examples with a headless browser would automate it.

## Risks

- The render-time panel API was too implicit for a multi-framework design: it was replaced by `PanelId` contexts.
- Creating series during render instead of in hook initialization creates duplicate series in Dioxus:
  the series are created in `use_hook`.
- Supporting Dioxus desktop or liveview would expand the scope sharply, the binding being browser/WASM-centric:
  they stay out of scope.
- If the core crate exposes too much raw JS detail, the adapter crates are harder to keep ergonomic:
  the core exposes Rust types and handles only, the JS declarations staying `pub(crate)`.
