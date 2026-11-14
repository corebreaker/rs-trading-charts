import { createChart } from 'lightweight-charts';
import { trackChart, untrackChart } from './fonts.mjs';
import { resolveOptions } from './formatters.mjs';
import { eventKinds, toMouseEventParams } from './events.mjs';
import { Legend } from './legend.mjs';
import { SeriesEntry } from './series.mjs';
import { attachSync, joinSyncGroup, leaveSyncGroup } from './sync.mjs';
import { shiftRange, TimeZoneShift, utcShift } from './timezone.mjs';
import { newId } from './utils.mjs';
import { Watermarks } from './watermarks.mjs';

/**
 * Chart object driven from Rust through wasm-bindgen.
 *
 * It owns the Lightweight Charts chart and the registries of series, watermarks and the legend. Everything added
 * before the chart is bound to a DOM node is kept and created on binding.
 */
export class TradingChart {
    constructor() {
        this._chart = null;
        this._node = null;
        this._series = {};
        this._watermarks = new Watermarks(this);
        this._legend = new Legend(this);
        this._subscriptions = new Map();
        this._syncGroup = null;
        this._syncDetach = null;
        this._timeShift = utcShift;
        this._timeContext = {
            toUtc: time => this._timeShift.toUtc(time),
        };
    }

    /** Synchronization group of the chart (`{ name, options }`), or `null`. */
    get syncGroup() {
        return this._syncGroup;
    }

    /** Converter between the real UTC times and the displayed times of the time zone of the chart. */
    get timeShift() {
        return this._timeShift;
    }

    /** Replace the formatter descriptors of options sent by Rust by functions. */
    resolveOptions(options) {
        return resolveOptions(options || {}, this._timeContext);
    }

    /** The Lightweight Charts chart, or `null` when the chart is not bound. */
    get chart() {
        return this._chart;
    }

    /** The DOM node the chart is bound to, or `null`. */
    get node() {
        return this._node;
    }

    /** The series entries, indexed by identifier. */
    get series() {
        return this._series;
    }

    _getChart() {
        if (!this._chart) {
            throw new Error('Chart is not bound to DOM');
        }

        return this._chart;
    }

    _getChartPriceScale(priceScaleId, paneIndex = undefined) {
        return this._getChart().priceScale(priceScaleId, paneIndex);
    }

    _getPane(paneIndex) {
        const pane = this._getChart().panes()[paneIndex];
        if (!pane) {
            throw new Error(`Pane with index '${paneIndex}' not found`);
        }

        return pane;
    }

    /** Return the pane at the given index, creating the missing panes. */
    ensurePane(paneIndex) {
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

    _bindSeries(seriesId) {
        if (!this._chart) {
            return;
        }

        this._getSeries(seriesId).bind(this._chart);
        this._legend.update();
    }

    _attachSubscription(subscription) {
        subscription.detach = eventKinds[subscription.kind](this, this._chart, subscription.callback);
    }

    _detachSubscription(subscription) {
        if (subscription.detach) {
            subscription.detach();
            subscription.detach = null;
        }
    }

    _detachSync() {
        if (this._syncDetach) {
            this._syncDetach();
            this._syncDetach = null;
        }
    }

    _unbindAll() {
        this._detachSync();
        this._legend.clear();
        for (const subscription of this._subscriptions.values()) {
            this._detachSubscription(subscription);
        }

        if (this._chart) {
            this._chart.remove();
            this._chart = null;
        }

        for (const series of Object.values(this._series)) {
            series.unbind();
        }
    }

    // -----------------------------------------------------------------------------------------------------------------
    // Lifecycle
    // -----------------------------------------------------------------------------------------------------------------

    destroy() {
        untrackChart(this);
        this.setSyncGroup(null);
        this._unbindAll();
        this._watermarks.destroy();
        this._subscriptions.clear();
        this._series = {};
        this._node = null;
    }

    bindChart(node, options = null) {
        this._unbindAll();

        this._node = node;
        this._chart = createChart(node, this.resolveOptions(options));
        trackChart(this);
        for (const seriesId of Object.keys(this._series)) {
            this._bindSeries(seriesId);
        }

        this._watermarks.bindAll();
        this._legend.mount();
        for (const subscription of this._subscriptions.values()) {
            this._attachSubscription(subscription);
        }

        if (this._syncGroup) {
            this._syncDetach = attachSync(this);
        }
    }

    // -----------------------------------------------------------------------------------------------------------------
    // Synchronization
    // -----------------------------------------------------------------------------------------------------------------

    /**
     * Join the synchronization group `name`, or leave the current group with `null`.
     *
     * The options tell what is synchronized: `crosshair`, `logicalRange` (visible logical range, for charts sharing
     * the same bars) and `timeRange` (visible time range).
     */
    setSyncGroup(name, options = null) {
        this._detachSync();
        if (this._syncGroup) {
            leaveSyncGroup(this, this._syncGroup.name);
        }

        this._syncGroup = name ? {
            name,
            options: Object.assign({ crosshair: true, logicalRange: true, timeRange: false }, options || {}),
        } : null;

        if (this._syncGroup) {
            joinSyncGroup(this, name);
            if (this._chart) {
                this._syncDetach = attachSync(this);
            }
        }
    }

    /** Move the crosshair to the data item at or before a real UTC time of the first series, or hide it (`null`). */
    syncCrosshair(time) {
        if (!this._chart) {
            return;
        }

        const series = Object.values(this._series).find(entry => entry.chartApi && entry.params.data?.length);
        const item = time === null || !series ? null : series.itemAt(time);
        const price = item ? (typeof item.value === 'number' ? item.value : item.close) : undefined;

        if (typeof price !== 'number') {
            this._chart.clearCrosshairPosition();
            this._emitSyntheticCrosshairMove({ seriesData: new Map() });
            return;
        }

        const chartTime = this._timeShift.toChart(item.time);
        this._chart.setCrosshairPosition(price, chartTime, series.chartApi);

        // Lightweight Charts does not fire the crosshair move event for a programmatic move: the subscribers of the
        // chart (legend, tooltips) are notified with parameters built like the ones of a real move.
        const seriesData = new Map();
        for (const entry of Object.values(this._series)) {
            const entryItem = entry.chartApi ? entry.itemAt(item.time) : null;
            if (entryItem && entryItem.time === item.time) {
                seriesData.set(entry.chartApi, Object.assign({}, entryItem, { time: chartTime }));
            }
        }

        const x = this._chart.timeScale().timeToCoordinate(chartTime);
        const y = series.chartApi.priceToCoordinate(price);
        this._emitSyntheticCrosshairMove({
            time: chartTime,
            logical: x === null ? undefined : this._chart.timeScale().coordinateToLogical(x),
            point: x === null || y === null ? undefined : { x, y },
            paneIndex: series.chartApi.getPane().paneIndex(),
            seriesData,
        });
    }

    _emitSyntheticCrosshairMove(param) {
        this._legend.refresh(param.time === undefined ? null : param);

        for (const subscription of this._subscriptions.values()) {
            if (subscription.kind === 'crosshairMove' && subscription.detach) {
                subscription.callback(toMouseEventParams(this, param));
            }
        }
    }

    // -----------------------------------------------------------------------------------------------------------------
    // Events
    // -----------------------------------------------------------------------------------------------------------------

    /**
     * Subscribe to an event of the chart (see `eventKinds`), and return the identifier of the subscription.
     *
     * Subscriptions made before the chart is bound are attached on binding, and they survive the rebinding.
     */
    subscribe(kind, callback) {
        if (!(kind in eventKinds)) {
            throw new Error(`Unknown event kind: '${kind}'`);
        }

        const id = newId();
        const subscription = { kind, callback, detach: null };

        this._subscriptions.set(id, subscription);
        if (this._chart) {
            this._attachSubscription(subscription);
        }

        return id;
    }

    /** Remove a subscription. Returns `true` when it existed. */
    unsubscribe(subscriptionId) {
        const subscription = this._subscriptions.get(subscriptionId);
        if (!subscription) {
            return false;
        }

        this._detachSubscription(subscription);
        this._subscriptions.delete(subscriptionId);

        return true;
    }

    // -----------------------------------------------------------------------------------------------------------------
    // Chart and scales
    // -----------------------------------------------------------------------------------------------------------------

    applyChartOptions(options) {
        this._getChart().applyOptions(this.resolveOptions(options));
    }

    applyTimeScaleOptions(options) {
        this._getChart().timeScale().applyOptions(this.resolveOptions(options));
    }

    /** Redraw the texts of the chart, after a web font has been loaded. */
    refreshFonts() {
        if (!this._chart) {
            return;
        }

        const fontFamily = this._chart.options().layout.fontFamily;
        this._chart.applyOptions({ layout: { fontFamily } });
        this._watermarks.bindAll();
    }

    applyPriceScaleOptions(priceScaleId, paneIndex, options) {
        this._getChartPriceScale(priceScaleId, paneIndex).applyOptions(options);
    }

    /** Set the time zone of the chart (an IANA name such as `Europe/Paris`), or remove it with `null`. */
    setTimeZone(timeZone) {
        const shift = timeZone ? new TimeZoneShift(timeZone) : utcShift;
        if (shift.timeZone === this._timeShift.timeZone) {
            return;
        }

        const visibleRange = this._chart ? this.getVisibleRange() : null;

        this._timeShift = shift;
        for (const series of Object.values(this._series)) {
            series.refreshTimes();
        }

        if (visibleRange) {
            this.setVisibleRange(visibleRange);
        }

        this._legend.refresh();
    }

    getTimeZone() {
        return this._timeShift.timeZone;
    }

    refitContent() {
        this._getChart().timeScale().fitContent();
    }

    getVisibleRange() {
        return shiftRange(this._getChart().timeScale().getVisibleRange(), time => this._timeShift.toUtc(time));
    }

    setVisibleRange(range) {
        this._getChart().timeScale().setVisibleRange(shiftRange(range, time => this._timeShift.toChart(time)));
    }

    getVisibleLogicalRange() {
        return this._getChart().timeScale().getVisibleLogicalRange();
    }

    setVisibleLogicalRange(range) {
        this._getChart().timeScale().setVisibleLogicalRange(range);
    }

    logicalToCoordinate(logical) {
        return this._getChart().timeScale().logicalToCoordinate(logical);
    }

    coordinateToLogical(coordinate) {
        return this._getChart().timeScale().coordinateToLogical(coordinate);
    }

    timeToCoordinate(time) {
        return this._getChart().timeScale().timeToCoordinate(this._timeShift.toChart(time));
    }

    coordinateToTime(coordinate) {
        const time = this._getChart().timeScale().coordinateToTime(coordinate);

        return time === null ? null : this._timeShift.toUtc(time);
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

    resize(width, height) {
        this._getChart().resize(width, height);
    }

    takeScreenshot() {
        return this._getChart().takeScreenshot().toDataURL();
    }

    // -----------------------------------------------------------------------------------------------------------------
    // Panes
    // -----------------------------------------------------------------------------------------------------------------

    getPaneCount() {
        return this._getChart().panes().length;
    }

    getPaneSize(paneIndex) {
        return this._getChart().paneSize(paneIndex);
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
        this._watermarks.onPaneMoved(paneIndex, targetIndex);
    }

    removePane(paneIndex) {
        this._getChart().removePane(paneIndex);
        this._syncSeriesPanels();
        this._watermarks.onPaneRemoved(paneIndex);
    }

    swapPanes(first, second) {
        this._getChart().swapPanes(first, second);
        this._syncSeriesPanels();
        this._watermarks.onPanesSwapped(first, second);
    }

    // -----------------------------------------------------------------------------------------------------------------
    // Legend and watermarks
    // -----------------------------------------------------------------------------------------------------------------

    setLegendOptions(options) {
        this._legend.setOptions(options);
    }

    removeLegend() {
        this._legend.remove();
    }

    addTextWatermark(paneIndex, options) {
        return this._watermarks.addText(paneIndex, options);
    }

    updateTextWatermark(watermarkId, options) {
        this._watermarks.update(watermarkId, options);
    }

    addImageWatermark(paneIndex, imageUrl, options) {
        return this._watermarks.addImage(paneIndex, imageUrl, options);
    }

    updateImageWatermark(watermarkId, options) {
        this._watermarks.update(watermarkId, options);
    }

    removeWatermark(watermarkId) {
        return this._watermarks.remove(watermarkId);
    }

    // -----------------------------------------------------------------------------------------------------------------
    // Series
    // -----------------------------------------------------------------------------------------------------------------

    addSeries(seriesDesc) {
        const optId = seriesDesc.id;

        if (optId && this._series[optId]) {
            throw new Error(`Series with id '${optId}' already exists`);
        }

        if (!seriesDesc.type) {
            throw new Error('Series type is required');
        }

        const id = newId();
        this._series[id] = new SeriesEntry(this, id, seriesDesc);
        this._bindSeries(id);
        this._legend.update();

        return id;
    }

    removeSeries(seriesId) {
        const series = this._series[seriesId];
        if (!series) {
            return false;
        }

        delete this._series[seriesId];
        if (this._chart) {
            series.detach(this._chart);
        }

        this._legend.update();

        return true;
    }

    updateSeriesOptions(seriesId, options) {
        this._getSeries(seriesId).applyOptions(options);
        this._legend.update();
    }

    applySeriesPriceScaleOptions(seriesId, options) {
        this._getSeries(seriesId).getApi().priceScale().applyOptions(options);
    }

    getSeriesPriceScaleVisibleRange(seriesId) {
        return this._getSeries(seriesId).getApi().priceScale().getVisibleRange();
    }

    setSeriesPriceScaleVisibleRange(seriesId, range) {
        this._getSeries(seriesId).getApi().priceScale().setVisibleRange(range);
    }

    setSeriesPriceScaleAutoScale(seriesId, on) {
        this._getSeries(seriesId).getApi().priceScale().setAutoScale(on);
    }

    getSeriesPriceScaleWidth(seriesId) {
        return this._getSeries(seriesId).getApi().priceScale().width();
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
        return this._getSeries(seriesId).getApi().seriesOrder();
    }

    setSeriesOrder(seriesId, order) {
        this._getSeries(seriesId).getApi().setSeriesOrder(order);
    }

    priceToCoordinate(seriesId, price) {
        return this._getSeries(seriesId).getApi().priceToCoordinate(price);
    }

    coordinateToPrice(seriesId, coordinate) {
        return this._getSeries(seriesId).getApi().coordinateToPrice(coordinate);
    }

    updateData(seriesId, data) {
        this._getSeries(seriesId).setData(data);
        this._legend.refresh();
    }

    updateDataPoint(seriesId, dataPoint) {
        this._getSeries(seriesId).updateDataPoint(dataPoint);
        this._legend.refresh();
    }

    setMarker(seriesId, marker) {
        const series = this._getSeries(seriesId);

        series.setMarker(marker);
        series.updateMarkers();
    }

    setMarkers(seriesId, markers) {
        const series = this._getSeries(seriesId);

        series.setMarkers(markers);
        series.updateMarkers();
    }

    applySeriesMarkersOptions(seriesId, options) {
        this._getSeries(seriesId).applyMarkersOptions(options);
    }

    setPriceLines(seriesId, priceLines) {
        this._getSeries(seriesId).setPriceLines(priceLines);
    }

    // -----------------------------------------------------------------------------------------------------------------
    // Crosshair
    // -----------------------------------------------------------------------------------------------------------------

    setCrosshairPosition(seriesId, price, horizontalPosition) {
        const series = this._getSeries(seriesId);

        this._getChart().setCrosshairPosition(price, this._timeShift.toChart(horizontalPosition), series.getApi());
    }

    clearCrosshairPosition() {
        this._getChart().clearCrosshairPosition();
    }
}
