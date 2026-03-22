NPM ?= npm
TRUNK ?= trunk
CARGO ?= cargo
CARGO_FMT ?= cargo +nightly fmt
LIGHTWEIGHT_CHARTS_VERSION ?= latest
TRUNK_ENV := env NO_COLOR=false

CORE_BINDINGS_DIR := trading-charts-core/bindings
LEPTOS_BINDINGS_DIR := trading-charts/bindings
LEPTOS_EXAMPLE_DIR := example
DIOXUS_EXAMPLE_DIR := example-dioxus

.DEFAULT_GOAL := help

.PHONY: help
help:
	@printf '%s\n' \
		'help                      Show available targets.' \
		'format                    Alias for fmt.' \
		'fmt                       Format the workspace.' \
		'clean                     Remove Cargo and example build artifacts.' \
		'check                     Run cargo check for the workspace.' \
		'check-wasm                Run cargo check for wasm32.' \
		'build-example             Build the Leptos example with trunk.' \
		'build-example-dioxus      Build the Dioxus example with trunk.' \
		'build-examples            Build both examples with trunk.' \
		'serve-example             Serve the Leptos example on its Trunk.toml port.' \
		'serve-example-dioxus      Serve the Dioxus example on its Trunk.toml port.' \
		'clean-examples            Remove example dist and .trunk artifacts.' \
		'verify                    Run fmt, check, check-wasm, and build both examples.' \
		'update-lightweight-charts Update both JS binding packages to the requested upstream version.'

.PHONY: format
format: fmt

.PHONY: fmt
fmt:
	$(CARGO_FMT) --all

.PHONY: clean
clean: clean-examples
	$(CARGO) clean

.PHONY: check
check:
	$(CARGO) check --workspace

.PHONY: check-wasm
check-wasm:
	$(CARGO) check --workspace --target wasm32-unknown-unknown

.PHONY: build-example
build-example:
	cd $(LEPTOS_EXAMPLE_DIR) && $(TRUNK_ENV) $(TRUNK) build

.PHONY: build-example-dioxus
build-example-dioxus:
	cd $(DIOXUS_EXAMPLE_DIR) && $(TRUNK_ENV) $(TRUNK) build

.PHONY: build-examples
build-examples: build-example build-example-dioxus

.PHONY: serve-example
serve-example:
	cd $(LEPTOS_EXAMPLE_DIR) && $(TRUNK_ENV) $(TRUNK) serve

.PHONY: serve-example-dioxus
serve-example-dioxus:
	cd $(DIOXUS_EXAMPLE_DIR) && $(TRUNK_ENV) $(TRUNK) serve

.PHONY: clean-examples
clean-examples:
	rm -rf $(LEPTOS_EXAMPLE_DIR)/dist $(LEPTOS_EXAMPLE_DIR)/.trunk
	rm -rf $(DIOXUS_EXAMPLE_DIR)/dist $(DIOXUS_EXAMPLE_DIR)/.trunk

.PHONY: verify
verify: fmt check check-wasm build-examples

.PHONY: update-lightweight-charts
update-lightweight-charts:
	cd $(CORE_BINDINGS_DIR) && $(NPM) install --package-lock-only --ignore-scripts --save-exact lightweight-charts@$(LIGHTWEIGHT_CHARTS_VERSION)
	cd $(LEPTOS_BINDINGS_DIR) && $(NPM) install --package-lock-only --ignore-scripts --save-exact lightweight-charts@$(LIGHTWEIGHT_CHARTS_VERSION)
	@printf 'Updated bindings to Lightweight Charts %s\n' \
		"`node -p \"require('./$(CORE_BINDINGS_DIR)/package-lock.json').packages['node_modules/lightweight-charts'].version\"`"
