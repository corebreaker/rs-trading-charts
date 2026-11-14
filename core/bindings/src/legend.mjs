import { formatDatePattern, timeToDate } from './formatters.mjs';
import { closedEye, createSvgIcon, openEye } from './icons.mjs';

const defaultLegendOptions = {
    visible: true,
    showOhlc: true,
    showPercent: true,
    showSeries: true,
    showVolume: true,
    toggleSeriesVisibility: true,
    text: '',
    textColor: '#0f172a',
    backgroundColor: 'rgba(0, 0, 0, 0)',
    fontSize: 12,
    fontFamily: "Avenir Next, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
    top: 10,
    left: 10,
    layout: 'series',
    position: 'topLeft',
    seriesId: null,
    timeFormat: 'yyyy-MM-dd HH:mm',
    locale: null,
    priceFontSize: null,
};

function positionStyle(options) {
    const horizontal = options.position === 'topRight' || options.position === 'bottomRight' ? 'right' : 'left';
    const vertical = options.position === 'bottomLeft' || options.position === 'bottomRight' ? 'bottom' : 'top';

    return {
        [horizontal]: `${options.left}px`,
        [vertical]: `${options.top}px`,
        alignItems: horizontal === 'right' ? 'flex-end' : 'flex-start',
    };
}

function getSeriesTitle(series) {
    const title = series.options.title;
    if (title && `${title}`.trim().length > 0) {
        return title;
    }

    const type = `${series.type || 'series'}`;
    return `${type.slice(0, 1).toUpperCase()}${type.slice(1)}`;
}

function extractSeriesPrice(dataPoint) {
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

function formatSeriesPrice(series, price) {
    if (!Number.isFinite(price)) {
        return '';
    }

    if (series.chartApi) {
        try {
            return series.getApi().priceFormatter().format(price);
        } catch (_error) {
        }
    }

    const precision = series.options.priceFormat?.precision;
    if (typeof precision === 'number') {
        return price.toFixed(precision);
    }

    if (Math.abs(price) >= 1000) {
        return price.toFixed(2);
    }

    return `${price}`;
}

function getSeriesColor(series, dataPoint = undefined) {
    const options = series.options;

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

function getSeriesLegendData(series, param = null) {
    if (param && param.seriesData && series.chartApi) {
        const hovered = param.seriesData.get(series.chartApi);
        if (hovered) {
            return hovered;
        }
    }

    const data = series.params.data;
    if (!Array.isArray(data) || data.length === 0) {
        return null;
    }

    return data[data.length - 1];
}

function isOhlcSeries(series) {
    return ['candlestick', 'bar'].includes(series.type);
}

function isVolumeSeries(series) {
    return series.type === 'histogram' && series.options.priceFormat?.type === 'volume';
}

function getLegendEntryTokens(series, dataPoint) {
    const options = series.options;

    if (isOhlcSeries(series)) {
        return [
            {
                symbol: series.type === 'bar' ? '┌' : '⋰',
                color: options.upColor || options.borderUpColor || '#26a69a',
            },
            {
                symbol: series.type === 'bar' ? '└' : '⋱',
                color: options.downColor || options.borderDownColor || '#ef5350',
            },
        ];
    }

    if (series.type === 'area') {
        return [{ symbol: '◪', color: options.lineColor || getSeriesColor(series, dataPoint) }];
    }

    if (series.type === 'histogram') {
        return [{ symbol: isVolumeSeries(series) ? '▥' : '▨', color: getSeriesColor(series, dataPoint) }];
    }

    if (series.type === 'baseline') {
        return [{ symbol: '◫', color: getSeriesColor(series, dataPoint) }];
    }

    return [{ symbol: '―', color: getSeriesColor(series, dataPoint) }];
}

function renderLegendInfo(series, dataPoint, showPercent) {
    const title = getSeriesTitle(series);
    const tokens = getLegendEntryTokens(series, dataPoint);
    const tokenMarkup = tokens
        .map(token => `<span style="color: ${token.color};">${token.symbol}</span>`)
        .join(' ');

    if (
        isOhlcSeries(series)
        && dataPoint
        && typeof dataPoint.open === 'number'
        && typeof dataPoint.close === 'number'
    ) {
        const openPrice = formatSeriesPrice(series, dataPoint.open);
        const closePrice = formatSeriesPrice(series, dataPoint.close);
        const upColor = tokens[0]?.color;
        const downColor = tokens[1]?.color || upColor;
        let percentMarkup = '';

        if (showPercent && dataPoint.open !== 0) {
            const percent = ((dataPoint.close - dataPoint.open) / dataPoint.open) * 100;
            const prefix = percent >= 0 ? '+' : '';
            const color = percent >= 0 ? upColor : downColor;

            percentMarkup = `, <span style="color: ${color};">${prefix}${percent.toFixed(2)}%</span>`;
        }

        return `${tokenMarkup} ${title}: <span style="color: ${upColor};">O ${openPrice}</span>, `
            + `<span style="color: ${downColor};">C ${closePrice}</span>${percentMarkup}`;
    }

    const price = extractSeriesPrice(dataPoint);
    const value = Number.isFinite(price) ? formatSeriesPrice(series, price) : '-';

    return `${tokenMarkup} ${title}: ${value}`;
}

/**
 * Built-in HTML legend drawn over the chart.
 *
 * The owner is the `TradingChart` instance: the legend reads its DOM node, its Lightweight Charts object and its
 * series registry.
 */
export class Legend {
    constructor(owner) {
        this._owner = owner;
        this._options = null;
        this._root = null;
        this._textNode = null;
        this._rowsNode = null;
        this._rows = {};
        this._threeLines = null;
        this._crosshairHandler = null;
    }

    setOptions(options) {
        this._options = Object.assign({}, defaultLegendOptions, options || {});
        this.mount();
    }

    remove() {
        this._options = null;
        this.clear();
    }

    /** Remove the legend from the DOM, keeping its options so that it can be mounted again. */
    clear() {
        const chart = this._owner.chart;
        if (chart && this._crosshairHandler) {
            chart.unsubscribeCrosshairMove(this._crosshairHandler);
        }

        if (this._root && this._root.parentNode) {
            this._root.parentNode.removeChild(this._root);
        }

        this._root = null;
        this._textNode = null;
        this._rowsNode = null;
        this._rows = {};
        this._threeLines = null;
        this._crosshairHandler = null;
    }

    _mountThreeLines(root, options) {
        const line = (fontSize, fontWeight) => {
            const element = document.createElement('div');
            Object.assign(element.style, { fontSize, fontWeight, lineHeight: '1.3' });
            root.appendChild(element);

            return element;
        };

        this._threeLines = {
            title: line(`${options.fontSize}px`, '600'),
            price: line(`${options.priceFontSize || options.fontSize * 2}px`, '700'),
            time: line(`${options.fontSize}px`, '400'),
        };
    }

    _refreshThreeLines(param) {
        const options = this._options;
        const lines = this._threeLines;
        const series = (options.seriesId && this._owner.series[options.seriesId])
            || Object.values(this._owner.series)[0];

        if (!series) {
            lines.title.textContent = options.text || '';
            lines.price.textContent = '';
            lines.time.textContent = '';
            return;
        }

        const dataPoint = getSeriesLegendData(series, param);
        const price = extractSeriesPrice(dataPoint);
        let time = null;
        if (param) {
            time = param.time;
        } else if (dataPoint) {
            time = this._owner.timeShift.toChart(dataPoint.time);
        }

        lines.title.textContent = options.text || getSeriesTitle(series);
        lines.price.textContent = Number.isFinite(price) ? formatSeriesPrice(series, price) : '-';
        lines.time.textContent = time === null || time === undefined
            ? ''
            : formatDatePattern(timeToDate(time), options.timeFormat, options.locale || undefined);
    }

    mount() {
        this.clear();

        const options = this._options;
        const node = this._owner.node;
        if (!options || !options.visible || !node) {
            return;
        }

        if (window.getComputedStyle(node).position === 'static') {
            node.style.position = 'relative';
        }

        const root = document.createElement('div');
        Object.assign(root.style, positionStyle(options), {
            position: 'absolute',
            zIndex: '3000',
            display: 'flex',
            flexDirection: 'column',
            maxWidth: 'calc(100% - 24px)',
            padding: '0px',
            borderRadius: '0px',
            background: options.backgroundColor,
            color: options.textColor,
            fontSize: `${options.fontSize}px`,
            fontFamily: options.fontFamily,
            lineHeight: 'normal',
            boxShadow: 'none',
            pointerEvents: 'none',
        });

        if (options.layout === 'threeLines') {
            this._mountThreeLines(root, options);
        } else {
            const textNode = document.createElement('span');
            textNode.style.lineHeight = '1.8';
            root.appendChild(textNode);

            const rowsNode = document.createElement('div');
            rowsNode.style.display = 'flex';
            rowsNode.style.flexDirection = 'column';
            root.appendChild(rowsNode);

            this._textNode = textNode;
            this._rowsNode = rowsNode;
        }

        node.appendChild(root);
        this._root = root;
        this.rebuildRows();
        this.refresh();

        const chart = this._owner.chart;
        if (chart) {
            this._crosshairHandler = param => {
                this.refresh(param && param.time !== undefined ? param : null);
            };
            chart.subscribeCrosshairMove(this._crosshairHandler);
        }
    }

    rebuildRows() {
        const options = this._options;
        const rowsNode = this._rowsNode;
        if (!options || !rowsNode) {
            return;
        }

        rowsNode.replaceChildren();
        this._rows = {};

        if (!options.showSeries) {
            return;
        }

        for (const [seriesId, series] of Object.entries(this._owner.series)) {
            if (isOhlcSeries(series) && !options.showOhlc) {
                continue;
            }

            if (isVolumeSeries(series) && !options.showVolume) {
                continue;
            }

            const row = document.createElement('div');
            row.className = 'legend-series-row';
            Object.assign(row.style, {
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'flex-start',
                marginBottom: '4px',
                pointerEvents: 'auto',
            });

            const info = document.createElement('div');
            info.className = 'series-info';
            info.style.flex = '1';
            info.style.fontVariantNumeric = 'tabular-nums';

            let toggle = null;
            if (options.toggleSeriesVisibility) {
                toggle = document.createElement('div');
                toggle.className = 'legend-toggle-switch';
                toggle.setAttribute('role', 'button');
                toggle.setAttribute('aria-label', `Toggle visibility for ${getSeriesTitle(series)}`);
                Object.assign(toggle.style, {
                    cursor: 'pointer',
                    display: 'flex',
                    alignItems: 'center',
                    marginRight: '10px',
                    borderRadius: '4px',
                    color: options.textColor,
                    pointerEvents: 'auto',
                });
                toggle.addEventListener('click', event => {
                    event.preventDefault();
                    event.stopPropagation();
                    series.setVisible(!series.visible);
                    this.refresh();
                });
                row.appendChild(toggle);
            }

            row.appendChild(info);
            rowsNode.appendChild(row);
            this._rows[seriesId] = { row, info, toggle };
        }
    }

    refresh(param = null) {
        const options = this._options;
        if (!options || !this._root) {
            return;
        }

        if (this._threeLines) {
            this._refreshThreeLines(param);
            return;
        }

        if (this._textNode) {
            this._textNode.textContent = options.text || '';
            this._textNode.style.display = options.text ? '' : 'none';
        }

        for (const [seriesId, row] of Object.entries(this._rows)) {
            const series = this._owner.series[seriesId];
            if (!series) {
                continue;
            }

            const dataPoint = getSeriesLegendData(series, param);
            const visible = series.visible;

            row.info.innerHTML = renderLegendInfo(series, dataPoint, options.showPercent);
            row.row.style.opacity = visible ? '1' : '0.52';
            if (row.toggle) {
                row.toggle.setAttribute('aria-pressed', visible ? 'true' : 'false');
                row.toggle.replaceChildren(createSvgIcon(visible ? openEye : closedEye));
            }
        }
    }

    /** Rebuild the rows and refresh their content, after the series list or their options changed. */
    update() {
        this.rebuildRows();
        this.refresh();
    }
}
