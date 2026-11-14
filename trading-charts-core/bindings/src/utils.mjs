import uuidv4 from '@bundled-es-modules/uuid/v4.js';

/** Generate a unique identifier for the objects managed by the binding (series, watermarks, subscriptions...). */
export function newId() {
    return uuidv4();
}

/** Sort data items in place by ascending time. */
export function sortByTime(data) {
    data.sort((a, b) => a.time - b.time);
}

/** Return `true` when the value is neither `null` nor `undefined`. */
export function isDefined(value) {
    return value !== null && value !== undefined;
}
