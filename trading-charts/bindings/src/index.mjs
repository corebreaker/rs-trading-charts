import uuidv4 from "@bundled-es-modules/uuid/v4.js";
import {
    createChart,
    createSeriesMarkers,
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

export class TradingChart {
    constructor() {
        this._chart = null;
        this._series = {};
        this._panelGenerator = 0;
        this._currentPanelId = null;
    }

    _getChart() {
        if (!this._chart)
            throw new Error('Chart is not bound to DOM');

        return this._chart;
    }

    _getSeries(seriesId) {
        const series = this._series[seriesId];
        if (!series) {
            throw new Error(`Series with id '${seriesId}' not found`);
        }

        return series;
    }

    _bindSeries(seriesId) {
        const chart = this._chart;
        if (!chart)
            return;

        const series = this._getSeries(seriesId);
        const params = series.params;
        if (!params)
            return;

        delete series.params;
        series.chartApi = chart.addSeries(getSeriesType(params.type), params.options);
        if (series.panel !== null) {
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
    }

    destroy() {
        for (const seriesId of Object.keys(this._series)) {
            this.removeSeries(seriesId);
        }

        if (this._chart) {
            this._chart.remove();
            this._chart = null;
        }
    }

    applyChartOptions(options) {
        const chart = this._getChart();

        chart.applyOptions(options);
    }

    bindChart(node, options = null) {
        if (this._chart)
            this.destroy();

        this._chart = createChart(node, options || {});
        for (const seriesId of Object.keys(this._series)) {
            this._bindSeries(seriesId);
        }
    }

    addPanel() {
        if (this._currentPanelId !== null) {
            throw new Error('Panel already exists, panels cannot be nested, remove it before adding a new one');
        }

        this._currentPanelId = this._panelGenerator++;
    }

    removePanel() {
        this._currentPanelId = null;
    }

    addSeries(seriesDesc) {
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

        if (data || Array.isArray(data)) {
            data.sort((a, b) => a.time - b.time);
        }

        const id = uuidv4();
        this._series[id] = {
            id,
            chartApi: null,
            markerApi: null,
            markerData: [],
            sorted: false,
            panel: this._currentPanelId,
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

        return id;
    }

    removeSeries(seriesId) {
        if (!this._chart)
            return false;

        const series = this._series[seriesId];
        if (!series)
            return false;

        delete this._series[seriesId];

        const api = series.getApi();

        series.markerData = [];
        series.updateMarkers();
        this._chart.removeSeries(api);

        return true;
    }

    updateSeriesOptions(seriesId, options) {
        this._getSeries(seriesId).getApi().applyOptions(options);
    }

    updateData(seriesId, data) {
        const chart = this._getChart();

        data.sort((a, b) => a.time - b.time);
        this._getSeries(seriesId).getApi().setData(data);
        chart.timeScale().fitContent();
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
}
