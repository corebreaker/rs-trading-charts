// Synchronization of charts.
//
// Charts of the same synchronization group share their crosshair position and their visible range:
// moving the crosshair or scrolling one chart updates the other charts of the group.
// A flag of the group prevents the updates made by the synchronization from being propagated again in the group,
// without blocking the synchronization of the other groups.

const groups = new Map();

/**
 * Call `fn` with the other bound members of the synchronization group of a chart,
 * unless the group is already propagating an update.
 */
function propagate(chart, fn) {
    const group = chart.syncGroup ? groups.get(chart.syncGroup.name) : null;
    if (!group || group.syncing) {
        return;
    }

    group.syncing = true;
    try {
        fn([...group.members].filter(member => member !== chart && member.chart));
    } finally {
        group.syncing = false;
    }
}

/** Add a chart to a synchronization group. */
export function joinSyncGroup(chart, name) {
    if (!groups.has(name)) {
        groups.set(name, { members: new Set(), syncing: false });
    }

    groups.get(name).members.add(chart);
}

/** Remove a chart from its synchronization group. */
export function leaveSyncGroup(chart, name) {
    const group = groups.get(name);
    if (group) {
        group.members.delete(chart);
        if (group.members.size === 0) {
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
        const handler = param => propagate(chart, members => {
            const time = param.time === undefined || !param.point ? null : chart.timeShift.toUtc(param.time);

            for (const member of members) {
                member.syncCrosshair(time);
            }
        });

        api.subscribeCrosshairMove(handler);
        detachers.push(() => api.unsubscribeCrosshairMove(handler));
    }

    if (options.logicalRange) {
        const handler = range => propagate(chart, members => {
            if (range) {
                for (const member of members) {
                    member.chart.timeScale().setVisibleLogicalRange(range);
                }
            }
        });

        api.timeScale().subscribeVisibleLogicalRangeChange(handler);
        detachers.push(() => api.timeScale().unsubscribeVisibleLogicalRangeChange(handler));
    }

    if (options.timeRange) {
        const handler = () => propagate(chart, members => {
            const range = chart.getVisibleRange();
            if (range) {
                for (const member of members) {
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
