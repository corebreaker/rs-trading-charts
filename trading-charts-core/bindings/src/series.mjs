import {
    AreaSeries,
    BarSeries,
    BaselineSeries,
    CandlestickSeries,
    createSeriesMarkers,
    HistogramSeries,
    LineSeries,
} from 'lightweight-charts';
import { resolveMarker } from './markers.mjs';
import { shiftItem } from './timezone.mjs';
import { isDefined, sortByTime } from './utils.mjs';

const seriesTypes = {
    area: AreaSeries,
    baseline: BaselineSeries,
    bar: BarSeries,
    candlestick: CandlestickSeries,
    histogram: HistogramSeries,
    line: LineSeries,
};

function getSeriesType(type) {
    if (!(type in seriesTypes)) {
        throw new Error(`Series type "${type}" is not supported`);
    }

    return seriesTypes[type];
}

/**
 * State of a series managed by the binding.
 *
 * The entry outlives the Lightweight Charts series object (`chartApi`): it keeps the parameters, data, markers and
 * price lines so that the series can be recreated when the chart is bound again to a DOM node.
 *
 * The data and markers are kept with their real UTC times: they are converted to the displayed times of the time
 * zone of the owner chart when they are given to Lightweight Charts.
 */
export class SeriesEntry {
    constructor(owner, id, { type, data, options, panel }) {
        if (Array.isArray(data)) {
            sortByTime(data);
        }

        this.id = id;
        this._owner = owner;
        this.chartApi = null;
        this.markerApi = null;
        this.markerData = [];
        this.markersOptions = {};
        this.priceLines = [];
        this.priceLineApis = [];
        this.sorted = false;
        this.panel = panel ?? null;
        this.params = {
            type,
            data,
            options: options || {},
        };
    }

    get type() {
        return this.params.type;
    }

    get options() {
        return this.params.options || {};
    }

    get visible() {
        return this.params.options?.visible !== false;
    }

    getApi() {
        if (!this.chartApi) {
            throw new Error('Series is not bound to chart');
        }

        return this.chartApi;
    }

    /** Create the Lightweight Charts series on the given chart. */
    bind(chart) {
        const params = this.params;

        this.chartApi = chart.addSeries(getSeriesType(params.type), this._owner.resolveOptions(params.options));
        if (isDefined(this.panel)) {
            this.chartApi.moveToPane(this.panel);
        }

        if (params.data) {
            this.chartApi.setData(this._chartData(params.data));
            chart.timeScale().fitContent();
        }

        this.markerApi = createSeriesMarkers(this.chartApi, [], this.markersOptions);
        this.updateMarkers();
        this.rebuildPriceLines();
    }

    /** Forget the Lightweight Charts objects, after the chart has been removed. */
    unbind() {
        this.chartApi = null;
        this.markerApi = null;
    }

    /** Remove the Lightweight Charts objects of the series from the chart. */
    detach(chart) {
        if (!this.chartApi) {
            return;
        }

        this.markerData = [];
        this.updateMarkers();
        for (const priceLine of this.priceLineApis) {
            this.chartApi.removePriceLine(priceLine);
        }

        chart.removeSeries(this.chartApi);
    }

    applyOptions(options) {
        this.params.options = options;
        if (this.chartApi) {
            this.chartApi.applyOptions(this._owner.resolveOptions(options));
        }
    }

    setVisible(visible) {
        this.params.options = Object.assign({}, this.params.options || {}, { visible });
        if (this.chartApi) {
            this.chartApi.applyOptions({ visible });
        }
    }

    setData(data) {
        sortByTime(data);
        this.params.data = data;
        if (this.chartApi) {
            this.chartApi.setData(this._chartData(data));
        }
    }

    _chartData(data) {
        const shift = this._owner.timeShift;

        return data.map(item => shiftItem(item, shift));
    }

    /** Give the data and markers again to Lightweight Charts, after the time zone of the chart changed. */
    refreshTimes() {
        if (!this.chartApi) {
            return;
        }

        this.chartApi.setData(this._chartData(this.params.data || []));
        this.updateMarkers();
    }

    /** Return the last data item at or before a real UTC time, or `null`. */
    itemAt(time) {
        const data = this.params.data || [];
        let low = 0;
        let high = data.length - 1;
        let found = null;

        while (low <= high) {
            const middle = (low + high) >> 1;
            if (data[middle].time <= time) {
                found = data[middle];
                low = middle + 1;
            } else {
                high = middle - 1;
            }
        }

        return found;
    }

    updateDataPoint(dataPoint) {
        const data = this.params.data || [];
        this.params.data = data;

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

        if (this.chartApi) {
            if (canIncrementallyUpdate) {
                this.chartApi.update(shiftItem(dataPoint, this._owner.timeShift));
            } else {
                this.chartApi.setData(this._chartData(data));
            }
        }
    }

    updateMarkers() {
        if (!this.markerApi) {
            return;
        }

        if (!this.sorted) {
            this.sorted = true;
            this.markerData.sort((a, b) => a.time - b.time);
        }

        const markers = this.markerApi;
        const shift = this._owner.timeShift;
        const markerData = this.markerData.map(marker => shiftItem(marker, shift));

        markers.setMarkers(markerData);
        setTimeout(() => {
            markers.setMarkers(markerData);
        }, 1);
    }

    /**
     * Add, replace or remove a single marker.
     *
     * A marker with an identifier replaces the marker with the same identifier, other markers replace all the
     * markers without identifier at the same time.
     */
    setMarker(markerDesc) {
        const { time, id } = markerDesc;
        if (!time) {
            throw new Error('Marker time is required');
        }

        this.markerData = this.markerData.filter(m => (id ? m.id !== id : m.id || m.time !== time));

        const marker = resolveMarker(markerDesc);
        if (marker) {
            this.markerData.push(marker);
            this.sorted = false;
        }
    }

    /** Replace all the markers. Several markers can share the same time. */
    setMarkers(markers) {
        this.markerData = markers.map(resolveMarker).filter(marker => marker !== null);
        this.sorted = false;
    }

    applyMarkersOptions(options) {
        this.markersOptions = Object.assign({}, this.markersOptions, options || {});
        if (this.markerApi) {
            this.markerApi.applyOptions(this.markersOptions);
        }
    }

    setPriceLines(priceLines) {
        this.priceLines = Array.isArray(priceLines) ? priceLines.slice() : [];
        this.rebuildPriceLines();
    }

    rebuildPriceLines() {
        if (!this.chartApi) {
            this.priceLineApis = [];
            return;
        }

        for (const priceLine of this.priceLineApis) {
            this.chartApi.removePriceLine(priceLine);
        }

        this.priceLineApis = this.priceLines.map(priceLine => this.chartApi.createPriceLine(priceLine));
    }
}
