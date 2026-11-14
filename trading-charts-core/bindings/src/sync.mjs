// Synchronization of charts.
//
// Charts of the same synchronization group share their crosshair position and their visible range: moving the
// crosshair or scrolling one chart updates the other charts of the group. A module flag prevents the updates made
// by the synchronization from being propagated again.

const groups = new Map();
let syncing = false;

function withSyncing(fn) {
    if (syncing) {
        return;
    }

    syncing = true;
    try {
        fn();
    } finally {
        syncing = false;
    }
}

function members(chart) {
    const group = chart.syncGroup ? groups.get(chart.syncGroup.name) : null;

    return group ? [...group].filter(member => member !== chart && member.chart) : [];
}

/** Add a chart to a synchronization group. */
export function joinSyncGroup(chart, name) {
    if (!groups.has(name)) {
        groups.set(name, new Set());
    }

    groups.get(name).add(chart);
}

/** Remove a chart from its synchronization group. */
export function leaveSyncGroup(chart, name) {
    const group = groups.get(name);
    if (group) {
        group.delete(chart);
        if (group.size === 0) {
            groups.delete(name);
        }
    }
}

/**
 * Attach the synchronization handlers to the Lightweight Charts chart of a member, and return the function
 * detaching them.
 */
export function attachSync(chart) {
    const api = chart.chart;
    const options = chart.syncGroup.options;
    const detachers = [];

    if (options.crosshair) {
        const handler = param => withSyncing(() => {
            const time = param.time === undefined || !param.point ? null : chart.timeShift.toUtc(param.time);

            for (const member of members(chart)) {
                member.syncCrosshair(time);
            }
        });

        api.subscribeCrosshairMove(handler);
        detachers.push(() => api.unsubscribeCrosshairMove(handler));
    }

    if (options.logicalRange) {
        const handler = range => withSyncing(() => {
            if (range) {
                for (const member of members(chart)) {
                    member.chart.timeScale().setVisibleLogicalRange(range);
                }
            }
        });

        api.timeScale().subscribeVisibleLogicalRangeChange(handler);
        detachers.push(() => api.timeScale().unsubscribeVisibleLogicalRangeChange(handler));
    }

    if (options.timeRange) {
        const handler = () => withSyncing(() => {
            const range = chart.getVisibleRange();
            if (range) {
                for (const member of members(chart)) {
                    member.setVisibleRange(range);
                }
            }
        });

        api.timeScale().subscribeVisibleTimeRangeChange(handler);
        detachers.push(() => api.timeScale().unsubscribeVisibleTimeRangeChange(handler));
    }

    return () => {
        for (const detach of detachers) {
            detach();
        }
    };
}
