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

function sortByTime(data) {
    data.sort((a, b) => a.time - b.time);
}

export class TradingChart {
    constructor() {
        this._chart = null;
        this._series = {};
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
    }

    destroy() {
        if (this._chart) {
            this._chart.remove();
            this._chart = null;
        }

        for (const series of Object.values(this._series)) {
            series.chartApi = null;
            series.markerApi = null;
        }

        this._series = {};
    }

    applyChartOptions(options) {
        const chart = this._getChart();

        chart.applyOptions(options);
    }

    bindChart(node, options = null) {
        if (this._chart) {
            this._chart.remove();
            this._chart = null;
            for (const series of Object.values(this._series)) {
                series.chartApi = null;
                series.markerApi = null;
            }
        }

        this._chart = createChart(node, options || {});
        for (const seriesId of Object.keys(this._series)) {
            this._bindSeries(seriesId);
        }
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

    resize(width, height) {
        const chart = this._getChart();

        chart.resize(width, height);
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

        return true;
    }

    updateSeriesOptions(seriesId, options) {
        const series = this._getSeries(seriesId);
        series.params.options = options;
        if (series.chartApi) {
            series.getApi().applyOptions(options);
        }
    }

    updateData(seriesId, data) {
        const series = this._getSeries(seriesId);
        sortByTime(data);
        series.params.data = data;
        if (series.chartApi) {
            series.getApi().setData(data);
        }
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
