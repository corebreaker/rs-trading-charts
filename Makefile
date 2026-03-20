NPM ?= npm
LIGHTWEIGHT_CHARTS_VERSION ?= latest
CORE_BINDINGS_DIR := trading-charts-core/bindings
LEPTOS_BINDINGS_DIR := trading-charts/bindings

.PHONY: update-lightweight-charts
update-lightweight-charts:
	cd $(CORE_BINDINGS_DIR) && $(NPM) install --package-lock-only --ignore-scripts --save-exact lightweight-charts@$(LIGHTWEIGHT_CHARTS_VERSION)
	cd $(LEPTOS_BINDINGS_DIR) && $(NPM) install --package-lock-only --ignore-scripts --save-exact lightweight-charts@$(LIGHTWEIGHT_CHARTS_VERSION)
	@printf 'Updated bindings to Lightweight Charts %s\n' \
		"`node -p \"require('./$(CORE_BINDINGS_DIR)/package-lock.json').packages['node_modules/lightweight-charts'].version\"`"
