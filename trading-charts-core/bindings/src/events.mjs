// Chart events sent to Rust.
//
// The Lightweight Charts event parameters refer to series objects and use displayed times: they are converted to
// plain objects referring to the series identifiers of the binding, with real UTC times.

function seriesPrice(data) {
    if (!data) {
        return null;
    }
    if (typeof data.value === 'number') {
        return data.value;
    }
    if (typeof data.close === 'number') {
        return data.close;
    }

    return null;
}

function paneOffsets(owner) {
    const chart = owner.chart;
    const node = owner.node;
    if (!chart || !node) {
        return { left: 0, tops: [] };
    }

    const nodeTop = node.getBoundingClientRect().top;
    const tops = chart.panes().map(pane => {
        const element = pane.getHTMLElement();

        return element ? element.getBoundingClientRect().top - nodeTop : 0;
    });

    let left = 0;
    try {
        left = chart.priceScale('left').width();
    } catch (_error) {
    }

    return { left, tops };
}

/**
 * Convert Lightweight Charts mouse event parameters for the Rust side.
 *
 * Lightweight Charts gives the positions relatively to the drawing area of the pane under the cursor: they are
 * converted to positions relative to the top-left corner of the chart, which is what overlays need.
 */
export function toMouseEventParams(owner, param) {
    const shift = owner.timeShift;
    const offsets = paneOffsets(owner);
    const paneTop = index => offsets.tops[index] ?? 0;
    const entries = Object.values(owner.series);
    const findEntry = api => entries.find(entry => entry.chartApi === api);
    const seriesData = [];

    if (param.seriesData) {
        for (const [api, data] of param.seriesData.entries()) {
            const entry = findEntry(api);
            if (!entry) {
                continue;
            }

            const paneIndex = api.getPane().paneIndex();
            const price = seriesPrice(data);
            let coordinate = null;
            if (price !== null) {
                const y = api.priceToCoordinate(price);
                if (y !== null) {
                    coordinate = paneTop(paneIndex) + y;
                }
            }

            seriesData.push({
                seriesId: entry.id,
                seriesType: entry.type,
                title: entry.options.title || null,
                paneIndex,
                data: Object.assign({}, data, { time: shift.toUtc(data.time) }),
                coordinate,
            });
        }
    }

    const hoveredEntry = param.hoveredSeries ? findEntry(param.hoveredSeries) : null;
    const node = owner.node;

    return {
        time: param.time === undefined ? null : shift.toUtc(param.time),
        logical: param.logical ?? null,
        point: param.point ? {
            x: offsets.left + param.point.x,
            y: paneTop(param.paneIndex ?? 0) + param.point.y,
        } : null,
        paneIndex: param.paneIndex ?? null,
        hoveredSeriesId: hoveredEntry ? hoveredEntry.id : null,
        hoveredObjectId: param.hoveredObjectId === undefined ? null : `${param.hoveredObjectId}`,
        seriesData,
        chartWidth: node ? node.clientWidth : 0,
        chartHeight: node ? node.clientHeight : 0,
    };
}

/**
 * Event kinds which can be subscribed to from Rust.
 *
 * Each kind gives how to attach a handler to a Lightweight Charts chart, and returns the function detaching it.
 */
export const eventKinds = {
    crosshairMove(owner, chart, callback) {
        const handler = param => callback(toMouseEventParams(owner, param));
        chart.subscribeCrosshairMove(handler);

        return () => chart.unsubscribeCrosshairMove(handler);
    },

    click(owner, chart, callback) {
        const handler = param => callback(toMouseEventParams(owner, param));
        chart.subscribeClick(handler);

        return () => chart.unsubscribeClick(handler);
    },

    dblClick(owner, chart, callback) {
        const handler = param => callback(toMouseEventParams(owner, param));
        chart.subscribeDblClick(handler);

        return () => chart.unsubscribeDblClick(handler);
    },

    visibleTimeRangeChange(owner, chart, callback) {
        const handler = range => callback(range ? {
            from: owner.timeShift.toUtc(range.from),
            to: owner.timeShift.toUtc(range.to),
        } : null);
        chart.timeScale().subscribeVisibleTimeRangeChange(handler);

        return () => chart.timeScale().unsubscribeVisibleTimeRangeChange(handler);
    },

    visibleLogicalRangeChange(owner, chart, callback) {
        const handler = range => callback(range ? { from: range.from, to: range.to } : null);
        chart.timeScale().subscribeVisibleLogicalRangeChange(handler);

        return () => chart.timeScale().unsubscribeVisibleLogicalRangeChange(handler);
    },

    sizeChange(owner, chart, callback) {
        const handler = (width, height) => callback({ width, height });
        chart.timeScale().subscribeSizeChange(handler);

        return () => chart.timeScale().unsubscribeSizeChange(handler);
    },
};
