const markerProps = {
    size: false,
    color: false,
    position: ['aboveBar', 'belowBar', 'inBar'],
    shape: ['arrowUp', 'arrowDown', 'circle', 'square', 'triangleUp', 'triangleDown'],
};

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

/**
 * Convert a marker description sent by Rust into a Lightweight Charts marker.
 *
 * Returns `null` for removal requests and unknown marker types.
 */
export function resolveMarker(markerDesc) {
    const { time, text, options } = markerDesc;
    const marker = { time };

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
            return null;
    }

    return Object.assign(marker, makeMarkerProps(options || {}));
}
