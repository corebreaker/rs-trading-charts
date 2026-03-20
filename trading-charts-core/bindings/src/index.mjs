import uuidv4 from "@bundled-es-modules/uuid/v4.js";
import {
    createChart,
    createImageWatermark,
    createSeriesMarkers,
    createTextWatermark,
    LineSeries,
    AreaSeries,
    BarSeries,
    BaselineSeries,
    CandlestickSeries,
    HistogramSeries,
} from 'lightweight-charts';

const markerProps = {
    size: false,
    color: false,
    position: ['aboveBar', 'belowBar', 'inBar'],
    shape: ['arrowUp', 'arrowDown', 'circle', 'square', 'triangleUp', 'triangleDown']
};

const seriesTypes = {
    area: AreaSeries,
    baseline: BaselineSeries,
    bar: BarSeries,
    candlestick: CandlestickSeries,
    histogram: HistogramSeries,
    line: LineSeries,
}

function getSeriesType(type) {
    if (!(type in seriesTypes)) {
        throw new Error(`Series type "${type}" is not supported`);
    }

    return seriesTypes[type];
}

function makeMarkerProps(options) {
    const res = {};

    for (const name of Object.keys(markerProps)) {
        const value = options[name];

        if (value && (!markerProps[name] || markerProps[name].includes(value))) {
            res[name] = value;
        }
    }

    return res;
}

function sortByTime(data) {
    data.sort((a, b) => a.time - b.time);
}

export class TradingChart {
    constructor() {
        this._chart = null;
        this._node = null;
        this._series = {};
        this._watermarks = {};
        this._legend = {
            options: null,
            root: null,
            ohlcNode: null,
            volumeNode: null,
            rowsNode: null,
            rows: {},
            crosshairHandler: null,
        };
    }

    _getChartPriceScale(priceScaleId, paneIndex = undefined) {
        const chart = this._getChart();

        return chart.priceScale(priceScaleId, paneIndex);
    }

    _getChart() {
        if (!this._chart)
            throw new Error('Chart is not bound to DOM');

        return this._chart;
    }

    _getPane(paneIndex) {
        const chart = this._getChart();
        const panes = chart.panes();
        const pane = panes[paneIndex];

        if (!pane) {
            throw new Error(`Pane with index '${paneIndex}' not found`);
        }

        return pane;
    }

    _ensurePane(paneIndex) {
        const chart = this._getChart();
        while (chart.panes().length <= paneIndex) {
            chart.addPane(true);
        }

        return this._getPane(paneIndex);
    }

    _getSeries(seriesId) {
        const series = this._series[seriesId];
        if (!series) {
            throw new Error(`Series with id '${seriesId}' not found`);
        }

        return series;
    }

    _syncSeriesPanels() {
        for (const series of Object.values(this._series)) {
            if (series.chartApi) {
                series.panel = series.chartApi.getPane().paneIndex();
            }
        }
    }

    _getWatermark(watermarkId) {
        const watermark = this._watermarks[watermarkId];
        if (!watermark) {
            throw new Error(`Watermark with id '${watermarkId}' not found`);
        }

        return watermark;
    }

    _normalizeLegendOptions(options) {
        return Object.assign({
            visible: true,
            showOhlc: true,
            showPercent: true,
            showSeries: true,
            showVolume: true,
            toggleSeriesVisibility: true,
            textColor: '#0f172a',
            backgroundColor: 'rgba(255, 255, 255, 0.78)',
            fontSize: 12,
            fontFamily: "-apple-system, BlinkMacSystemFont, 'Trebuchet MS', Roboto, Ubuntu, sans-serif",
            top: 12,
            left: 12,
        }, options || {});
    }

    _getSeriesTitle(series) {
        const title = series.params?.options?.title;
        if (title && `${title}`.trim().length > 0) {
            return title;
        }

        const type = `${series.params?.type || 'series'}`;
        return `${type.slice(0, 1).toUpperCase()}${type.slice(1)}`;
    }

    _getSeriesVisible(series) {
        return series.params?.options?.visible !== false;
    }

    _extractSeriesPrice(dataPoint) {
        if (!dataPoint) {
            return null;
        }
        if (typeof dataPoint.value === 'number') {
            return dataPoint.value;
        }
        if (typeof dataPoint.close === 'number') {
            return dataPoint.close;
        }
        if (typeof dataPoint.price === 'number') {
            return dataPoint.price;
        }

        return null;
    }

    _formatSeriesPrice(series, price) {
        if (!Number.isFinite(price)) {
            return '';
        }

        if (series.chartApi) {
            try {
                return series.getApi().priceFormatter().format(price);
            } catch (_error) {
            }
        }

        const precision = series.params?.options?.priceFormat?.precision;
        if (typeof precision === 'number') {
            return price.toFixed(precision);
        }

        if (Math.abs(price) >= 1000) {
            return price.toFixed(2);
        }

        return `${price}`;
    }

    _getSeriesColor(series, dataPoint = undefined) {
        const options = series.params?.options || {};

        if (
            dataPoint &&
            typeof dataPoint.open === 'number' &&
            typeof dataPoint.close === 'number' &&
            options.upColor &&
            options.downColor
        ) {
            return dataPoint.close >= dataPoint.open ? options.upColor : options.downColor;
        }

        if (series.chartApi) {
            try {
                const lastValue = series.getApi().lastValueData(true);
                if (lastValue && lastValue.color) {
                    return lastValue.color;
                }
            } catch (_error) {
            }
        }

        return options.color
            || options.lineColor
            || options.topLineColor
            || options.bottomLineColor
            || options.upColor
            || options.borderUpColor
            || options.priceLineColor
            || '#334155';
    }

    _getSeriesLegendData(series, param = null) {
        if (param && param.seriesData && series.chartApi) {
            const hovered = param.seriesData.get(series.chartApi);
            if (hovered) {
                return hovered;
            }
        }

        const data = series.params?.data;
        if (!Array.isArray(data) || data.length === 0) {
            return null;
        }

        return data[data.length - 1];
    }

    _getOhlcSeries() {
        return Object.values(this._series).find(series => ['candlestick', 'bar'].includes(series.params?.type));
    }

    _getVolumeSeries() {
        return Object.values(this._series).find(series => (
            series.params?.type === 'histogram'
            && series.params?.options?.priceFormat?.type === 'volume'
        ));
    }

    _setSeriesVisible(seriesId, visible) {
        const series = this._getSeries(seriesId);
        series.params.options = Object.assign({}, series.params.options || {}, { visible });
        if (series.chartApi) {
            series.getApi().applyOptions({ visible });
        }
        this._refreshLegend();
    }

    _clearLegend() {
        if (this._chart && this._legend.crosshairHandler) {
            this._chart.unsubscribeCrosshairMove(this._legend.crosshairHandler);
        }

        if (this._legend.root && this._legend.root.parentNode) {
            this._legend.root.parentNode.removeChild(this._legend.root);
        }

        this._legend.root = null;
        this._legend.ohlcNode = null;
        this._legend.volumeNode = null;
        this._legend.rowsNode = null;
        this._legend.rows = {};
        this._legend.crosshairHandler = null;
    }

    _rebuildLegendRows() {
        const options = this._legend.options;
        const rowsNode = this._legend.rowsNode;
        if (!options || !rowsNode) {
            return;
        }

        rowsNode.replaceChildren();
        this._legend.rows = {};

        if (!options.showSeries) {
            return;
        }

        for (const [seriesId, series] of Object.entries(this._series)) {
            const row = document.createElement('div');
            row.style.display = 'flex';
            row.style.alignItems = 'center';
            row.style.gap = '8px';
            row.style.pointerEvents = 'none';

            const swatch = document.createElement('span');
            swatch.style.display = 'inline-block';
            swatch.style.width = '10px';
            swatch.style.height = '10px';
            swatch.style.borderRadius = '999px';
            swatch.style.flex = '0 0 10px';

            const label = document.createElement('span');
            label.textContent = this._getSeriesTitle(series);
            label.style.minWidth = '64px';

            const value = document.createElement('span');
            value.style.fontVariantNumeric = 'tabular-nums';
            value.style.opacity = '0.92';

            row.appendChild(swatch);
            row.appendChild(label);
            row.appendChild(value);

            let toggle = null;
            if (options.toggleSeriesVisibility) {
                toggle = document.createElement('button');
                toggle.type = 'button';
                toggle.style.pointerEvents = 'auto';
                toggle.style.border = '1px solid rgba(15, 23, 42, 0.16)';
                toggle.style.background = 'rgba(255, 255, 255, 0.65)';
                toggle.style.borderRadius = '999px';
                toggle.style.padding = '2px 8px';
                toggle.style.fontSize = '11px';
                toggle.style.lineHeight = '1.2';
                toggle.style.cursor = 'pointer';
                toggle.addEventListener('click', event => {
                    event.preventDefault();
                    event.stopPropagation();
                    this._setSeriesVisible(seriesId, !this._getSeriesVisible(series));
                });
                row.appendChild(toggle);
            }

            rowsNode.appendChild(row);
            this._legend.rows[seriesId] = { row, swatch, label, value, toggle };
        }
    }

    _refreshLegend(param = null) {
        const options = this._legend.options;
        if (!options || !this._legend.root) {
            return;
        }

        if (this._legend.ohlcNode) {
            const series = this._getOhlcSeries();
            if (series && options.showOhlc) {
                const dataPoint = this._getSeriesLegendData(series, param);
                if (
                    dataPoint &&
                    typeof dataPoint.open === 'number' &&
                    typeof dataPoint.high === 'number' &&
                    typeof dataPoint.low === 'number' &&
                    typeof dataPoint.close === 'number'
                ) {
                    const parts = [
                        `O ${this._formatSeriesPrice(series, dataPoint.open)}`,
                        `H ${this._formatSeriesPrice(series, dataPoint.high)}`,
                        `L ${this._formatSeriesPrice(series, dataPoint.low)}`,
                        `C ${this._formatSeriesPrice(series, dataPoint.close)}`,
                    ];

                    if (options.showPercent && dataPoint.open !== 0) {
                        const percent = ((dataPoint.close - dataPoint.open) / dataPoint.open) * 100;
                        const prefix = percent >= 0 ? '+' : '';
                        parts.push(`${prefix}${percent.toFixed(2)}%`);
                    }

                    this._legend.ohlcNode.textContent = `${this._getSeriesTitle(series)}  ${parts.join('  ')}`;
                    this._legend.ohlcNode.style.display = '';
                } else {
                    this._legend.ohlcNode.style.display = 'none';
                }
            } else {
                this._legend.ohlcNode.style.display = 'none';
            }
        }

        if (this._legend.volumeNode) {
            const volumeSeries = this._getVolumeSeries();
            if (volumeSeries && options.showVolume) {
                const dataPoint = this._getSeriesLegendData(volumeSeries, param);
                const price = this._extractSeriesPrice(dataPoint);
                if (Number.isFinite(price)) {
                    this._legend.volumeNode.textContent = `${this._getSeriesTitle(volumeSeries)}  ${this._formatSeriesPrice(volumeSeries, price)}`;
                    this._legend.volumeNode.style.display = '';
                } else {
                    this._legend.volumeNode.style.display = 'none';
                }
            } else {
                this._legend.volumeNode.style.display = 'none';
            }
        }

        for (const [seriesId, row] of Object.entries(this._legend.rows)) {
            const series = this._series[seriesId];
            if (!series) {
                continue;
            }

            const dataPoint = this._getSeriesLegendData(series, param);
            const price = this._extractSeriesPrice(dataPoint);
            const visible = this._getSeriesVisible(series);

            row.label.textContent = this._getSeriesTitle(series);
            row.swatch.style.background = this._getSeriesColor(series, dataPoint);
            row.value.textContent = Number.isFinite(price) ? this._formatSeriesPrice(series, price) : '';
            row.row.style.opacity = visible ? '1' : '0.52';
            if (row.toggle) {
                row.toggle.textContent = visible ? 'hide' : 'show';
            }
        }
    }

    _mountLegend() {
        this._clearLegend();

        const options = this._legend.options;
        if (!options || !options.visible || !this._node) {
            return;
        }

        if (window.getComputedStyle(this._node).position === 'static') {
            this._node.style.position = 'relative';
        }

        const root = document.createElement('div');
        root.style.position = 'absolute';
        root.style.left = `${options.left}px`;
        root.style.top = `${options.top}px`;
        root.style.zIndex = '20';
        root.style.display = 'flex';
        root.style.flexDirection = 'column';
        root.style.gap = '6px';
        root.style.maxWidth = 'calc(100% - 24px)';
        root.style.padding = '8px 10px';
        root.style.borderRadius = '10px';
        root.style.background = options.backgroundColor;
        root.style.color = options.textColor;
        root.style.fontSize = `${options.fontSize}px`;
        root.style.fontFamily = options.fontFamily;
        root.style.lineHeight = '1.35';
        root.style.boxShadow = '0 6px 24px rgba(15, 23, 42, 0.08)';
        root.style.pointerEvents = 'none';

        const ohlcNode = document.createElement('div');
        ohlcNode.style.fontVariantNumeric = 'tabular-nums';
        ohlcNode.style.fontWeight = '600';
        root.appendChild(ohlcNode);

        const volumeNode = document.createElement('div');
        volumeNode.style.fontVariantNumeric = 'tabular-nums';
        volumeNode.style.opacity = '0.9';
        root.appendChild(volumeNode);

        const rowsNode = document.createElement('div');
        rowsNode.style.display = 'flex';
        rowsNode.style.flexDirection = 'column';
        rowsNode.style.gap = '4px';
        root.appendChild(rowsNode);

        this._node.appendChild(root);
        this._legend.root = root;
        this._legend.ohlcNode = ohlcNode;
        this._legend.volumeNode = volumeNode;
        this._legend.rowsNode = rowsNode;
        this._rebuildLegendRows();
        this._refreshLegend();

        if (this._chart) {
            this._legend.crosshairHandler = param => {
                this._refreshLegend(param && param.time !== undefined ? param : null);
            };
            this._chart.subscribeCrosshairMove(this._legend.crosshairHandler);
        }
    }

    _bindWatermark(watermarkId) {
        const watermark = this._getWatermark(watermarkId);
        const pane = this._ensurePane(watermark.pane);

        if (watermark.api) {
            watermark.api.detach();
        }

        watermark.api = watermark.kind === 'text'
            ? createTextWatermark(pane, watermark.options)
            : createImageWatermark(pane, watermark.imageUrl, watermark.options);
    }

    _reindexWatermarksAfterSwap(first, second) {
        for (const watermark of Object.values(this._watermarks)) {
            if (watermark.pane === first) {
                watermark.pane = second;
            } else if (watermark.pane === second) {
                watermark.pane = first;
            }
        }
    }

    _reindexWatermarksAfterMove(from, to) {
        if (from === to) {
            return;
        }

        for (const watermark of Object.values(this._watermarks)) {
            if (watermark.pane === from) {
                watermark.pane = to;
            } else if (from < to && watermark.pane > from && watermark.pane <= to) {
                watermark.pane -= 1;
            } else if (from > to && watermark.pane >= to && watermark.pane < from) {
                watermark.pane += 1;
            }
        }
    }

    _reindexWatermarksAfterRemoval(paneIndex) {
        for (const [watermarkId, watermark] of Object.entries(this._watermarks)) {
            if (watermark.pane === paneIndex) {
                if (watermark.api) {
                    watermark.api.detach();
                }
                delete this._watermarks[watermarkId];
            } else if (watermark.pane > paneIndex) {
                watermark.pane -= 1;
            }
        }
    }

    _bindSeries(seriesId) {
        const chart = this._chart;
        if (!chart)
            return;

        const series = this._getSeries(seriesId);
        const params = series.params;
        if (!params)
            return;

        series.chartApi = chart.addSeries(getSeriesType(params.type), params.options);
        if (series.panel !== null && series.panel !== undefined) {
            series.chartApi.moveToPane(series.panel);
        }

        if (params.data) {
            series.chartApi.setData(params.data);
            chart.timeScale().fitContent();
        }

        if (series.markerApi) {
            series.updateMarkers();
        } else {
            series.markerApi = createSeriesMarkers(series.chartApi, series.markerData || []);
        }

        series.rebuildPriceLines();
        this._rebuildLegendRows();
        this._refreshLegend();
    }

    destroy() {
        this._clearLegend();

        if (this._chart) {
            this._chart.remove();
            this._chart = null;
        }

        for (const series of Object.values(this._series)) {
            series.chartApi = null;
            series.markerApi = null;
        }

        for (const watermark of Object.values(this._watermarks)) {
            if (watermark.api) {
                watermark.api.detach();
                watermark.api = null;
            }
        }

        this._series = {};
        this._watermarks = {};
        this._node = null;
    }

    applyChartOptions(options) {
        const chart = this._getChart();

        chart.applyOptions(options);
    }

    applyTimeScaleOptions(options) {
        const chart = this._getChart();

        chart.timeScale().applyOptions(options);
    }

    applyPriceScaleOptions(priceScaleId, paneIndex, options) {
        this._getChartPriceScale(priceScaleId, paneIndex).applyOptions(options);
    }

    bindChart(node, options = null) {
        this._clearLegend();

        if (this._chart) {
            this._chart.remove();
            this._chart = null;
            for (const series of Object.values(this._series)) {
                series.chartApi = null;
                series.markerApi = null;
            }
        }

        this._node = node;
        this._chart = createChart(node, options || {});
        for (const seriesId of Object.keys(this._series)) {
            this._bindSeries(seriesId);
        }

        for (const watermarkId of Object.keys(this._watermarks)) {
            this._bindWatermark(watermarkId);
        }

        this._mountLegend();
    }

    refitContent() {
        const chart = this._getChart();

        chart.timeScale().fitContent();
    }

    getVisibleRange() {
        const chart = this._getChart();

        return chart.timeScale().getVisibleRange();
    }

    setVisibleRange(range) {
        const chart = this._getChart();

        chart.timeScale().setVisibleRange(range);
    }

    getVisibleLogicalRange() {
        const chart = this._getChart();

        return chart.timeScale().getVisibleLogicalRange();
    }

    setVisibleLogicalRange(range) {
        const chart = this._getChart();

        chart.timeScale().setVisibleLogicalRange(range);
    }

    logicalToCoordinate(logical) {
        const chart = this._getChart();

        return chart.timeScale().logicalToCoordinate(logical);
    }

    coordinateToLogical(coordinate) {
        const chart = this._getChart();

        return chart.timeScale().coordinateToLogical(coordinate);
    }

    timeToCoordinate(time) {
        const chart = this._getChart();

        return chart.timeScale().timeToCoordinate(time);
    }

    coordinateToTime(coordinate) {
        const chart = this._getChart();

        return chart.timeScale().coordinateToTime(coordinate);
    }

    getPriceScaleVisibleRange(priceScaleId, paneIndex) {
        return this._getChartPriceScale(priceScaleId, paneIndex).getVisibleRange();
    }

    setPriceScaleVisibleRange(priceScaleId, paneIndex, range) {
        this._getChartPriceScale(priceScaleId, paneIndex).setVisibleRange(range);
    }

    setPriceScaleAutoScale(priceScaleId, paneIndex, on) {
        this._getChartPriceScale(priceScaleId, paneIndex).setAutoScale(on);
    }

    getPriceScaleWidth(priceScaleId, paneIndex) {
        return this._getChartPriceScale(priceScaleId, paneIndex).width();
    }

    getPaneCount() {
        const chart = this._getChart();

        return chart.panes().length;
    }

    getPaneSize(paneIndex) {
        const chart = this._getChart();

        return chart.paneSize(paneIndex);
    }

    setPaneHeight(paneIndex, height) {
        this._getPane(paneIndex).setHeight(height);
    }

    getPaneStretchFactor(paneIndex) {
        return this._getPane(paneIndex).getStretchFactor();
    }

    setPaneStretchFactor(paneIndex, stretchFactor) {
        this._getPane(paneIndex).setStretchFactor(stretchFactor);
    }

    movePane(paneIndex, targetIndex) {
        this._getPane(paneIndex).moveTo(targetIndex);
        this._syncSeriesPanels();
        this._reindexWatermarksAfterMove(paneIndex, targetIndex);
    }

    removePane(paneIndex) {
        const chart = this._getChart();

        chart.removePane(paneIndex);
        this._syncSeriesPanels();
        this._reindexWatermarksAfterRemoval(paneIndex);
    }

    swapPanes(first, second) {
        const chart = this._getChart();

        chart.swapPanes(first, second);
        this._syncSeriesPanels();
        this._reindexWatermarksAfterSwap(first, second);
    }

    resize(width, height) {
        const chart = this._getChart();

        chart.resize(width, height);
    }

    takeScreenshot() {
        const chart = this._getChart();

        return chart.takeScreenshot().toDataURL();
    }

    setLegendOptions(options) {
        this._legend.options = this._normalizeLegendOptions(options);
        this._mountLegend();
    }

    removeLegend() {
        this._legend.options = null;
        this._clearLegend();
    }

    addTextWatermark(paneIndex, options) {
        const id = uuidv4();
        this._watermarks[id] = {
            id,
            kind: 'text',
            pane: paneIndex,
            options: options || {},
            api: null,
        };

        if (this._chart) {
            this._bindWatermark(id);
        }

        return id;
    }

    updateTextWatermark(watermarkId, options) {
        const watermark = this._getWatermark(watermarkId);

        watermark.options = options || {};
        if (watermark.api) {
            watermark.api.applyOptions(watermark.options);
        }
    }

    addImageWatermark(paneIndex, imageUrl, options) {
        const id = uuidv4();
        this._watermarks[id] = {
            id,
            kind: 'image',
            pane: paneIndex,
            imageUrl,
            options: options || {},
            api: null,
        };

        if (this._chart) {
            this._bindWatermark(id);
        }

        return id;
    }

    updateImageWatermark(watermarkId, options) {
        const watermark = this._getWatermark(watermarkId);

        watermark.options = options || {};
        if (watermark.api) {
            watermark.api.applyOptions(watermark.options);
        }
    }

    removeWatermark(watermarkId) {
        const watermark = this._watermarks[watermarkId];
        if (!watermark) {
            return false;
        }

        if (watermark.api) {
            watermark.api.detach();
        }

        delete this._watermarks[watermarkId];
        return true;
    }

    addSeries(seriesDesc) {
        const optId = seriesDesc.id;
        const type = seriesDesc.type;
        const data = seriesDesc.data;
        const options = seriesDesc.options || {};
        const panel = seriesDesc.panel;

        if (optId && this._series[optId]) {
            throw new Error(`Series with id '${optId}' already exists`);
        }

        if (!type) {
            throw new Error('Series type is required');
        }

        if (Array.isArray(data)) {
            sortByTime(data);
        }

        const id = uuidv4();
        this._series[id] = {
            id,
            chartApi: null,
            markerApi: null,
            markerData: [],
            priceLines: [],
            priceLineApis: [],
            sorted: false,
            panel: panel ?? null,
            params: {
                type,
                data,
                options,
            },

            getApi() {
                if (!this.chartApi) {
                    throw new Error('Series is not bound to chart');
                }

                return this.chartApi;
            },

            getMarkers() {
                if (!this.markerApi) {
                    throw new Error('Markers is not bound to chart');
                }

                return this.markerApi;
            },

            updateMarkers() {
                if (!this.sorted) {
                    this.sorted = true;
                    this.markerData.sort((a, b) => a.time - b.time);
                }

                const markers = this.getMarkers();
                markers.setMarkers(this.markerData);
                setTimeout(() => {
                    markers.setMarkers(this.markerData);
                }, 1);
            },

            rebuildPriceLines() {
                if (!this.chartApi) {
                    this.priceLineApis = [];
                    return;
                }

                for (const priceLine of this.priceLineApis) {
                    this.chartApi.removePriceLine(priceLine);
                }

                this.priceLineApis = this.priceLines.map(priceLine => this.chartApi.createPriceLine(priceLine));
            },

            setMarker(markerDesc) {
                const { time, text, options } = markerDesc;
                if (!time) {
                    throw new Error('Marker time is required');
                }

                const idx = this.markerData.findIndex(m => m.time === time);
                if (idx >= 0) {
                    this.markerData.splice(idx, 1);
                }

                if (markerDesc.type && markerDesc.type !== 'remove') {
                    const marker = {time};
                    switch (markerDesc.type) {
                        case 'buy':
                            Object.assign(marker, {
                                text: text || 'B',
                                position: 'belowBar',
                                shape: 'arrowUp',
                                color: 'green',
                                size: 1,
                            });
                            break;

                        case 'sell':
                            Object.assign(marker, {
                                text: text || 'S',
                                position: 'aboveBar',
                                shape: 'arrowDown',
                                color: 'red',
                                size: 1,
                            });
                            break;

                        default:
                            Object.assign(marker, {_bad: true});
                            break;
                    }

                    if (!marker._bad) {
                        this.markerData.push(Object.assign(marker, makeMarkerProps(options || {})));
                        this.sorted = false;
                    }
                }
            }
        };

        this._bindSeries(id);
        this._rebuildLegendRows();
        this._refreshLegend();

        return id;
    }

    removeSeries(seriesId) {
        const series = this._series[seriesId];
        if (!series)
            return false;

        delete this._series[seriesId];

        if (this._chart && series.chartApi) {
            series.markerData = [];
            series.updateMarkers();
            for (const priceLine of series.priceLineApis) {
                series.chartApi.removePriceLine(priceLine);
            }
            this._chart.removeSeries(series.chartApi);
        }

        this._rebuildLegendRows();
        this._refreshLegend();

        return true;
    }

    updateSeriesOptions(seriesId, options) {
        const series = this._getSeries(seriesId);
        series.params.options = options;
        if (series.chartApi) {
            series.getApi().applyOptions(options);
        }
        this._rebuildLegendRows();
        this._refreshLegend();
    }

    applySeriesPriceScaleOptions(seriesId, options) {
        const series = this._getSeries(seriesId);

        series.getApi().priceScale().applyOptions(options);
    }

    getSeriesPriceScaleVisibleRange(seriesId) {
        const series = this._getSeries(seriesId);

        return series.getApi().priceScale().getVisibleRange();
    }

    setSeriesPriceScaleVisibleRange(seriesId, range) {
        const series = this._getSeries(seriesId);

        series.getApi().priceScale().setVisibleRange(range);
    }

    setSeriesPriceScaleAutoScale(seriesId, on) {
        const series = this._getSeries(seriesId);

        series.getApi().priceScale().setAutoScale(on);
    }

    getSeriesPriceScaleWidth(seriesId) {
        const series = this._getSeries(seriesId);

        return series.getApi().priceScale().width();
    }

    moveSeriesToPane(seriesId, paneIndex) {
        const series = this._getSeries(seriesId);

        series.panel = paneIndex;
        series.getApi().moveToPane(paneIndex);
        this._syncSeriesPanels();
    }

    getSeriesPaneIndex(seriesId) {
        const series = this._getSeries(seriesId);

        if (!series.chartApi) {
            return series.panel ?? 0;
        }

        return series.getApi().getPane().paneIndex();
    }

    getSeriesOrder(seriesId) {
        const series = this._getSeries(seriesId);

        return series.getApi().seriesOrder();
    }

    setSeriesOrder(seriesId, order) {
        const series = this._getSeries(seriesId);

        series.getApi().setSeriesOrder(order);
    }

    priceToCoordinate(seriesId, price) {
        const series = this._getSeries(seriesId);

        return series.getApi().priceToCoordinate(price);
    }

    coordinateToPrice(seriesId, coordinate) {
        const series = this._getSeries(seriesId);

        return series.getApi().coordinateToPrice(coordinate);
    }

    setCrosshairPosition(seriesId, price, horizontalPosition) {
        const chart = this._getChart();
        const series = this._getSeries(seriesId);

        chart.setCrosshairPosition(price, horizontalPosition, series.getApi());
    }

    clearCrosshairPosition() {
        const chart = this._getChart();

        chart.clearCrosshairPosition();
    }

    updateData(seriesId, data) {
        const series = this._getSeries(seriesId);
        sortByTime(data);
        series.params.data = data;
        if (series.chartApi) {
            series.getApi().setData(data);
        }
        this._refreshLegend();
    }

    updateDataPoint(seriesId, dataPoint) {
        const series = this._getSeries(seriesId);
        const data = series.params.data || [];
        series.params.data = data;

        const idx = data.findIndex(item => item.time === dataPoint.time);
        let canIncrementallyUpdate = true;

        if (idx >= 0) {
            data[idx] = dataPoint;
            canIncrementallyUpdate = idx === data.length - 1;
        } else if (data.length === 0 || data[data.length - 1].time < dataPoint.time) {
            data.push(dataPoint);
        } else {
            data.push(dataPoint);
            sortByTime(data);
            canIncrementallyUpdate = false;
        }

        if (series.chartApi) {
            if (canIncrementallyUpdate) {
                series.getApi().update(dataPoint);
            } else {
                series.getApi().setData(data);
            }
        }
        this._refreshLegend();
    }

    setMarker(seriesId, marker) {
        const series = this._getSeries(seriesId)

        series.setMarker(marker);
        series.updateMarkers();
    }

    setMarkers(seriesId, markers) {
        const series = this._getSeries(seriesId)

        series.markerData = [];
        for (const marker of markers) {
            series.setMarker(marker);
        }

        series.updateMarkers();
    }

    setPriceLines(seriesId, priceLines) {
        const series = this._getSeries(seriesId);

        series.priceLines = Array.isArray(priceLines) ? priceLines.slice() : [];
        series.rebuildPriceLines();
    }
}
