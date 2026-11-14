// Time zone support.
//
// Lightweight Charts works with UTC timestamps and has no notion of time zone. As recommended by its documentation,
// the times are shifted so that their UTC representation is the wall-clock time of the time zone: a candle at
// 14:00 UTC is displayed at 16:00 for the `Europe/Paris` time zone in summer. The binding keeps the real UTC times
// on its side, shifts them when giving data to Lightweight Charts, and shifts back the times it returns.

const HOUR = 3600;

/** Converter between real UTC times and displayed times for a time zone. */
export class TimeZoneShift {
    constructor(timeZone) {
        // Throws a `RangeError` for an invalid time zone.
        this._formatter = new Intl.DateTimeFormat('en-US', {
            timeZone,
            hourCycle: 'h23',
            year: 'numeric',
            month: 'numeric',
            day: 'numeric',
            hour: 'numeric',
            minute: 'numeric',
            second: 'numeric',
        });
        this._offsets = new Map();
        this.timeZone = timeZone;
    }

    /** Offset in seconds of the time zone at the given UTC time in seconds. Offsets are cached per hour. */
    offset(utcSeconds) {
        const bucket = Math.floor(utcSeconds / HOUR);
        let offset = this._offsets.get(bucket);

        if (offset === undefined) {
            const time = bucket * HOUR;
            const parts = {};
            for (const part of this._formatter.formatToParts(new Date(time * 1000))) {
                parts[part.type] = part.value;
            }

            const wallClock = Date.UTC(parts.year, parts.month - 1, parts.day, parts.hour, parts.minute, parts.second);
            offset = wallClock / 1000 - time;
            this._offsets.set(bucket, offset);
        }

        return offset;
    }

    /** Convert a real UTC time to the displayed time. */
    toChart(time) {
        return typeof time === 'number' ? time + this.offset(time) : time;
    }

    /** Convert a displayed time back to the real UTC time. */
    toUtc(time) {
        if (typeof time !== 'number') {
            return time;
        }

        const guess = time - this.offset(time);

        return time - this.offset(guess);
    }
}

/** Converter used when the chart has no time zone. */
export const utcShift = {
    timeZone: null,
    toChart: time => time,
    toUtc: time => time,
};

/** Return a copy of a data item or a marker with its time converted to the displayed time. */
export function shiftItem(item, shift) {
    if (shift === utcShift || !item || typeof item.time !== 'number') {
        return item;
    }

    return Object.assign({}, item, { time: shift.toChart(item.time) });
}

/** Return a copy of a range of times converted with the given conversion function. */
export function shiftRange(range, convert) {
    if (!range) {
        return range;
    }

    return {
        from: convert(range.from),
        to: convert(range.to),
    };
}
