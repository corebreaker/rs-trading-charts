// Series markers.
//
// Markers sent by Rust have a `type`: either empty (the marker is fully described by its own properties), `remove`
// (removal request), or the identifier of a style of the marker registry. The registry holds the `buy` and `sell`
// styles by default, and can be extended or overridden with `registerMarkerStyle`.

const positions = ['aboveBar', 'belowBar', 'inBar', 'atPriceTop', 'atPriceBottom', 'atPriceMiddle'];
const shapes = ['circle', 'square', 'arrowUp', 'arrowDown'];

const defaultMarker = {
    position: 'aboveBar',
    shape: 'circle',
    color: '#2196f3',
};

const builtInStyles = {
    buy: {
        text: 'B',
        position: 'belowBar',
        shape: 'arrowUp',
        color: 'green',
        size: 1,
    },
    sell: {
        text: 'S',
        position: 'aboveBar',
        shape: 'arrowDown',
        color: 'red',
        size: 1,
    },
};

const styles = new Map(Object.entries(builtInStyles));

function cleanStyle(style) {
    const result = {};

    if (positions.includes(style.position)) {
        result.position = style.position;
    }
    if (shapes.includes(style.shape)) {
        result.shape = style.shape;
    }
    for (const name of ['color', 'text']) {
        if (typeof style[name] === 'string' && style[name].length > 0) {
            result[name] = style[name];
        }
    }
    for (const name of ['size', 'price']) {
        if (Number.isFinite(style[name])) {
            result[name] = style[name];
        }
    }
    if (typeof style.id === 'string' && style.id.length > 0) {
        result.id = style.id;
    }

    return result;
}

/** Register (or replace) a marker style under an identifier, used by the markers of that type. */
export function registerMarkerStyle(id, style) {
    if (!id || id === 'remove') {
        throw new Error(`Invalid marker style identifier: '${id}'`);
    }

    styles.set(id, cleanStyle(style || {}));
}

/** Remove a marker style; the built-in `buy` and `sell` styles are restored instead of being removed. */
export function unregisterMarkerStyle(id) {
    if (id in builtInStyles) {
        styles.set(id, builtInStyles[id]);
        return true;
    }

    return styles.delete(id);
}

/** Return `true` when the marker description is a removal request. */
export function isRemoval(markerDesc) {
    return markerDesc.type === 'remove';
}

/**
 * Convert a marker description sent by Rust into a Lightweight Charts marker.
 *
 * The properties of the marker take precedence over its style. Returns `null` for removal requests and markers with
 * an unknown style.
 */
export function resolveMarker(markerDesc) {
    if (!markerDesc || isRemoval(markerDesc)) {
        return null;
    }

    let style = {};
    if (markerDesc.type) {
        style = styles.get(markerDesc.type);
        if (!style) {
            console.warn(`Unknown marker style '${markerDesc.type}', the marker is ignored`);
            return null;
        }
    }

    const marker = Object.assign({ time: markerDesc.time }, defaultMarker, style, cleanStyle(markerDesc));
    if (marker.position.startsWith('atPrice') && !Number.isFinite(marker.price)) {
        console.warn(`The marker at ${markerDesc.time} has the '${marker.position}' position without price`);
        marker.position = defaultMarker.position;
    }

    return marker;
}
