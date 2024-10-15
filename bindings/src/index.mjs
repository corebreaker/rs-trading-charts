import {createChart} from 'lightweight-charts';
import uuidv4 from "@bundled-es-modules/uuid/v4.js";

const allowedSeriesTypes = ['area', 'baseline', 'bar', 'candlestick', 'histogram', 'line'];
const markerProps = {
    size: false,
    color: false,
    position: ['aboveBar', 'belowBar', 'inBar'],
    shape: ['arrowUp', 'arrowDown', 'circle', 'square', 'triangleUp', 'triangleDown']
};

function makeSeriesName(type) {
    if (!allowedSeriesTypes.includes(type)) {
        throw new Error(`Series type "${type}" is not supported`);
    }

    const title = type.charAt(0).toUpperCase() + type.slice(1);

    return `add${title}Series`;
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

export class TradingChart {
    constructor() {
        this._chart = null;
        this._series = {};
    }

    _checkChart() {
        if (!this._chart)
            throw new Error('Chart is not bound to DOM');
    }

    _getSeries(seriesId) {
        const series = this._series[seriesId];
        if (!series) {
            throw new Error(`Series with id '${seriesId}' not found`);
        }

        return series;
    }

    bindChart(node, options = null) {
        if (this._chart)
            this.destroy();

        this._chart = createChart(node, options || {});
    }

    destroy() {
        for (const seriesId of Object.keys(this._series)) {
            this.removeSeries(seriesId);
        }

        if (this._chart)
            this._chart.remove();
    }

    addSeries(seriesDesc) {
        this._checkChart();

        const optId = seriesDesc.id;
        const type = seriesDesc.type;
        const data = seriesDesc.data;
        const options = seriesDesc.options || {};

        if (optId && this._series[optId]) {
            throw new Error(`Series with id '${optId}' already exists`);
        }

        if (!type) {
            throw new Error('Series type is required');
        }

        const id = uuidv4();
        const series = this._chart[makeSeriesName(type)](options);
        if (data) {
            series.setData(data);
            this._chart.timeScale().fitContent();
        }

        this._series[id] = {
            id,
            api: series,
            markers: [],

            updateMarkers() {
                console.log('updateMarkers:', this.markers);
                this.api.setMarkers(this.markers);
            },

            setMarker(markerDesc) {
                console.log('setMarker:', markerDesc);
                const { time, text, options } = markerDesc;
                if (!time) {
                    throw new Error('Marker time is required');
                }

                const idx = this.markers.findIndex(m => m.time === time);
                if (idx >= 0) {
                    this.markers.splice(idx, 1);
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
                    }

                    if (!marker._bad)
                        this.markers.push(Object.assign(marker, makeMarkerProps(options || {})));
                }
            }
        };

        return id;
    }

    removeSeries(seriesId) {
        if (!this._chart)
            return false;

        const series = this._getSeries[seriesId];

        series.api.setMarkers([]);
        this._chart.removeSeries(series.api);
        delete this._series[seriesId.id];

        return true;
    }

    updateData(seriesDesc) {
        this._checkChart();

        const series = this._getSeries(seriesDesc.id);

        series.api.setData(seriesDesc.data);
        this._chart.timeScale().fitContent();
    }

    updateOptions(seriesDesc) {
        this._checkChart();

        const series = this._getSeries(seriesDesc.id);

        series.api.applyOptions(seriesDesc.options);
    }

    setMarker(seriesId, marker) {
        this._checkChart();

        const series = this._getSeries(seriesId)

        series.setMarker(marker);
        series.updateMarkers();
    }

    setMarkers(seriesId, markers) {
        this._checkChart();

        const series = this._getSeries(seriesId)

        series.markers = [];
        for (const marker of markers) {
            series.setMarker(marker);
        }

        series.updateMarkers();
    }
}
