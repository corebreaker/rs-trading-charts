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

    getMarkers() {
        if (!this.markerApi) {
            throw new Error('Markers is not bound to chart');
        }

        return this.markerApi;
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

        if (this.markerApi) {
            this.updateMarkers();
        } else {
            this.markerApi = createSeriesMarkers(this.chartApi, this.markerData || []);
        }

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
        if (!this.sorted) {
            this.sorted = true;
            this.markerData.sort((a, b) => a.time - b.time);
        }

        const markers = this.getMarkers();
        const shift = this._owner.timeShift;
        const markerData = this.markerData.map(marker => shiftItem(marker, shift));

        markers.setMarkers(markerData);
        setTimeout(() => {
            markers.setMarkers(markerData);
        }, 1);
    }

    setMarker(markerDesc) {
        const { time } = markerDesc;
        if (!time) {
            throw new Error('Marker time is required');
        }

        const idx = this.markerData.findIndex(m => m.time === time);
        if (idx >= 0) {
            this.markerData.splice(idx, 1);
        }

        const marker = resolveMarker(markerDesc);
        if (marker) {
            this.markerData.push(marker);
            this.sorted = false;
        }
    }

    setMarkers(markers) {
        this.markerData = [];
        for (const marker of markers) {
            this.setMarker(marker);
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
