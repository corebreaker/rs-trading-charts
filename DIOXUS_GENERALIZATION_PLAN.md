# Plan to generalize `rs-trading-charts` for Dioxus

## Executive summary

Yes: the `rs-trading-charts` binding can support both Leptos and Dioxus from the same repository, and it can share almost all of the current JS/WASM and data-model code.

The part that should be shared is the chart runtime and `lightweight-charts` binding. The part that should not be shared directly is the UI/component layer, because the current public API is Leptos-specific and Dioxus has different render and mount semantics.

My recommendation is:

- Keep one repository and one shared JS binding pipeline.
- Extract a framework-agnostic core crate.
- Keep the current `trading-charts` crate as the Leptos adapter for backward compatibility.
- Add a new `trading-charts-dioxus` crate for Dioxus web support.
- Do not target Dioxus desktop/liveview in the first pass. Target `wasm32-unknown-unknown` and Dioxus `web` only.

## Short answer to the “same time” question

It is possible to make the binding work for Leptos and Dioxus at the same time, but there are two different meanings:

- Same repository/workspace: yes, this is the right target.
- Same shared Rust/JS core: yes, this is the main design goal.
- Same published crate with feature flags only: technically yes, but not the cleanest long-term API surface.
- Same end-user app running both frameworks together: technically possible in separate DOM roots, but not a useful design target for this library.

The important point is that the browser-facing binding can be shared, while the component shell should be framework-specific.

## Current state of the repo

The current workspace already has a natural split between framework-neutral code and Leptos-only code.

Framework-neutral today:

- `trading-charts/src/bindings/binding.rs`
- `trading-charts/src/bindings/js.rs`
- `trading-charts/src/data/**`
- `trading-charts/src/error.rs`
- `trading-charts/build.rs`
- `trading-charts/bindings/**`

Leptos-specific today:

- `trading-charts/src/chart.rs`
- `trading-charts/src/panel.rs`
- `trading-charts/src/series/candlesticks/component.rs`
- workspace dependencies on `leptos` and `emitix`
- `example/**`

Two structural observations matter for the Dioxus plan:

- The low-level binding already only depends on `wasm-bindgen`, `web-sys`, JS, and serde. That is reusable.
- The current public component API performs some imperative chart/series setup in a way that fits Leptos component setup, but Dioxus function components re-run on render. A direct line-by-line port would be fragile.

## Problem model

### Relevant entities

- The `lightweight-charts` JS module.
- The wasm-bindgen Rust bridge in `trading-charts/src/bindings/**`.
- Shared chart data types in `trading-charts/src/data/**`.
- A chart runtime/handle that owns the JS chart object.
- Framework adapters for Leptos and Dioxus.
- A DOM mount point represented as `web_sys::HtmlDivElement`.
- Optional panel and series handles.

### State variables

- Chart lifecycle state: created, bound to DOM, destroyed.
- Chart options state.
- Series registry state.
- Panel assignment state.
- Mounted DOM node availability.
- Framework reactive state for data/options/markers.

### Actions

- Create chart runtime.
- Bind chart runtime to a mounted DOM node.
- Apply chart options after bind.
- Allocate panel identifiers.
- Add or remove series.
- Update series options.
- Update series data.
- Update markers.
- Destroy chart and cleanup series on unmount.

### Constraints

- `lightweight-charts` requires a real browser DOM node.
- The shared binding depends on `wasm-bindgen` and `web-sys`, so this remains a web/WASM integration.
- Dioxus only matches this path on the `web` renderer.
- The current Leptos API should remain usable or have a clear compatibility path.
- The public API must avoid framework lifecycle assumptions leaking into the shared core.

## Recommended architecture

### Preferred structure

Use a core-plus-adapters workspace layout:

- `trading-charts-core`
  - Shared JS build pipeline.
  - Shared wasm-bindgen layer.
  - Shared `data`, `error`, and runtime handles.
- `trading-charts`
  - Leptos adapter crate.
  - Keeps or re-exports the current Leptos-facing API.
- `trading-charts-dioxus`
  - Dioxus web adapter crate.
  - Provides Dioxus components/hooks around the shared core.
- `wasmbind-macro`
  - Unchanged.
- `example-leptos`
  - Current example moved or renamed.
- `example-dioxus-web`
  - New example.

### Why this is better than a single dual-feature crate

A single crate with `leptos` and `dioxus` features would work, but it has weaker separation:

- Docs become harder to navigate.
- Public modules tend to become feature-gated and noisy.
- Component lifecycle differences still force a runtime abstraction anyway.
- Backward compatibility is easier if `trading-charts` remains the Leptos crate users already know.

If you want one user-facing meta-crate later, add it after the core/adapters split is stable.

## Key design change: remove ambient panel state

This is the most important refactor for a clean Dioxus integration.

Today the JS bridge tracks `_currentPanelId` and `ChartPanel` temporarily mutates that state while children render. That works as a render-time DSL, but it is not a good shared abstraction for multiple frameworks.

Instead, make panel assignment explicit:

- Allocate a `PanelId` from the chart handle.
- Pass `PanelId` through framework context.
- Add series with an explicit `panel_id: Option<PanelId>`.

That lets both Leptos and Dioxus use the same runtime contract without relying on ambient mutable “current panel” state.

Suggested direction:

- Replace JS `addPanel()` and `removePanel()` with explicit pane assignment on `addSeries(...)`.
- Keep an internal counter for `PanelId` allocation in Rust or JS.
- Model series as explicit handles so unmount cleanup can call `removeSeries`.

## Proposed crate responsibilities

### `trading-charts-core`

Move these pieces into the new core crate:

- `bindings/**`
- `data/**`
- `error.rs`
- `build.rs`
- `bindings/` JS sources and npm build assets

Expose public runtime types such as:

- `ChartHandle`
- `PanelId`
- `SeriesHandle`
- shared option/data/marker types
- `JsError`

Add or formalize core methods such as:

- `ChartHandle::new`
- `ChartHandle::bind(HtmlDivElement)`
- `ChartHandle::apply_chart_options`
- `ChartHandle::allocate_panel`
- `ChartHandle::add_series`
- `ChartHandle::update_series_options`
- `ChartHandle::update_data`
- `ChartHandle::set_markers`
- `ChartHandle::refit_content`
- `ChartHandle::remove_series`

The core crate should have no dependency on Leptos, Dioxus, or `emitix`.

### `trading-charts` Leptos adapter

This crate should:

- Depend on `trading-charts-core`.
- Re-export core data types for convenience.
- Keep `Chart`, `ChartPanel`, and `CandleStickSeries` as Leptos components.
- Keep Leptos-specific conveniences such as `REFIT_EVENT_KIND` and any `emitix` integration.

Internally, it should stop depending on render-time ambient panel mutation and use explicit panel/series handles from the core.

### `trading-charts-dioxus`

This crate should:

- Depend on `trading-charts-core`.
- Depend on Dioxus web-facing features only.
- Use Dioxus `onmounted` to capture a mounted DOM node.
- Downcast mounted data to `web_sys::Element` and then `web_sys::HtmlDivElement`.
- Use `use_context_provider` / `use_context` to propagate the chart handle and panel id.
- Use `use_hook` or `use_hook_with_cleanup` to create series once.
- Use `use_effect` for reactive updates to options, data, and markers.
- Use `use_drop` cleanup to remove series and destroy resources cleanly.

## Dioxus-specific implementation notes

### DOM binding

The first pass should be web-only. The Dioxus component should bind only after mount:

- `div { onmounted: ... }`
- store `MountedData`
- obtain the underlying web element
- cast to `web_sys::HtmlDivElement`
- call the shared core `bind(...)`

This is the right match for the existing `web-sys` + `wasm-bindgen` stack.

### State and rerender safety

Dioxus function components re-run. Because of that:

- Do not call `add_series` directly in the render body.
- Create long-lived chart and series handles in hooks.
- Use effects only for updates.
- Use drop cleanup for `remove_series`.

This is the main reason a direct copy of the current Leptos component code is not enough.

### Suggested Dioxus dependency shape

Use a minimal Dioxus feature set for libraries:

- `dioxus` with `default-features = false`
- enable the equivalent of `web`, `hooks`, `signals`, `macro`, and `mounted`
- add `dioxus-web` if needed for `WebEventExt`

Do not add desktop/native/liveview support in the first iteration.

## Migration plan

### Phase 1: extract the core

Goal:

- Create a stable, framework-agnostic runtime crate.

Steps:

- Add `trading-charts-core` to the workspace.
- Move `bindings/**`, `data/**`, `error.rs`, `build.rs`, and JS build assets into the core crate.
- Make the runtime types public instead of `pub(crate)`.
- Add explicit `remove_series` support to the Rust side if it is not already exposed cleanly.
- Replace ambient panel mutation with explicit panel identifiers.

Exit criteria:

- A small wasm test/example can create a chart runtime, bind it to an `HtmlDivElement`, add a series, update it, and destroy it without Leptos.

### Phase 2: refactor the Leptos adapter onto the core

Goal:

- Preserve current behavior while switching implementation to the shared runtime.

Steps:

- Update the current `trading-charts` crate to depend on `trading-charts-core`.
- Keep the public Leptos component names stable where possible.
- Re-export the shared data types from the Leptos crate.
- Refactor `ChartPanel` to provide a `PanelId` context rather than mutating a shared “current panel”.
- Refactor series components to hold explicit series handles and clean them up on drop if possible.

Exit criteria:

- The existing Leptos example still works with the new core-backed implementation.

### Phase 3: add the Dioxus adapter

Goal:

- Provide a Dioxus web API with parity for the existing Leptos surface.

Steps:

- Add `trading-charts-dioxus` to the workspace.
- Implement `Chart`, `ChartPanel`, and `CandleStickSeries` in Dioxus terms.
- Bind the chart on `onmounted`.
- Provide chart and panel context with Dioxus context hooks.
- Create series once and update them through `use_effect`.
- Add a Dioxus web example with the same dataset flow as the Leptos example.

Exit criteria:

- A Dioxus web example can mount the chart, switch datasets, update markers, and refit content.

### Phase 4: documentation and publishability

Goal:

- Make the new structure understandable and publishable.

Steps:

- Update the root README to describe the workspace layout.
- Document that the shared binding works for Leptos and Dioxus web.
- Document that Dioxus desktop/native/liveview are out of scope initially.
- Add crate-level READMEs for `trading-charts-core` and `trading-charts-dioxus`.
- Decide whether `trading-charts` remains the Leptos crate name permanently or becomes a thin umbrella later.

Exit criteria:

- A user can tell which crate to use for Leptos vs Dioxus without reading source.

## Compatibility and naming strategy

The least disruptive naming strategy is:

- Keep `trading-charts` as the Leptos crate.
- Add `trading-charts-core`.
- Add `trading-charts-dioxus`.

That avoids an immediate breaking change for current users.

If you instead rename the current crate to `trading-charts-leptos`, the architecture is still sound, but you should treat that as a larger API migration.

## Testing and validation plan

At minimum, add these checks:

- `cargo check -p trading-charts-core --target wasm32-unknown-unknown`
- `cargo check -p trading-charts --target wasm32-unknown-unknown`
- `cargo check -p trading-charts-dioxus --target wasm32-unknown-unknown`
- run the Leptos example in a browser
- run the Dioxus web example in a browser

Behavior to validate in both frameworks:

- initial chart render
- data update
- marker update
- options update
- refit action
- component unmount/remount
- multiple series
- multiple panels

## Main risks

- The current render-time panel API is too implicit for a clean multi-framework design.
- Dioxus lifecycle mistakes will create duplicate series if series creation is done during render instead of hook initialization.
- If you try to support Dioxus desktop or liveview in the first pass, scope will expand sharply because the current binding is browser/WASM-centric.
- If the core crate exposes too much raw JS detail, the adapter crates will be harder to keep ergonomic.

## Recommended first implementation slice

Do this in the smallest useful order:

1. Extract `trading-charts-core`.
2. Replace ambient panel state with explicit `PanelId`.
3. Move the current Leptos crate onto the core.
4. Add a minimal Dioxus `Chart` plus one `CandleStickSeries`.
5. Add Dioxus panel support.
6. Expand docs and examples.

That order gives you a usable architecture quickly and prevents Dioxus-specific work from being built on top of Leptos-only assumptions.
