import { createImageWatermark, createTextWatermark } from 'lightweight-charts';
import { mergeOptions } from './formatters.mjs';
import { newId } from './utils.mjs';

/**
 * Text and image watermarks of a chart, attached to panes.
 *
 * Watermarks are kept while the chart is not bound so that they can be recreated on the next binding. Pane indices
 * are kept in sync when panes are moved, swapped or removed.
 */
export class Watermarks {
    constructor(owner) {
        this._owner = owner;
        this._watermarks = {};
    }

    _get(watermarkId) {
        const watermark = this._watermarks[watermarkId];
        if (!watermark) {
            throw new Error(`Watermark with id '${watermarkId}' not found`);
        }

        return watermark;
    }

    _bind(watermarkId) {
        const watermark = this._get(watermarkId);
        const pane = this._owner.ensurePane(watermark.pane);

        if (watermark.api) {
            watermark.api.detach();
        }

        watermark.api = watermark.kind === 'text'
            ? createTextWatermark(pane, watermark.options)
            : createImageWatermark(pane, watermark.imageUrl, watermark.options);
    }

    _add(watermark) {
        const id = newId();
        this._watermarks[id] = Object.assign({ id, api: null }, watermark);

        if (this._owner.chart) {
            this._bind(id);
        }

        return id;
    }

    /** Create the watermarks on the panes of the newly bound chart. */
    bindAll() {
        for (const watermarkId of Object.keys(this._watermarks)) {
            this._bind(watermarkId);
        }
    }

    /** Detach and forget all the watermarks. */
    destroy() {
        for (const watermark of Object.values(this._watermarks)) {
            if (watermark.api) {
                watermark.api.detach();
                watermark.api = null;
            }
        }

        this._watermarks = {};
    }

    addText(paneIndex, options) {
        return this._add({
            kind: 'text',
            pane: paneIndex,
            options: options || {},
        });
    }

    addImage(paneIndex, imageUrl, options) {
        return this._add({
            kind: 'image',
            pane: paneIndex,
            imageUrl,
            options: options || {},
        });
    }

    /**
     * Apply options to a watermark, merged into its kept options.
     *
     * The kept options are applied whole, so that the watermark gets the same options as on the next binding.
     */
    update(watermarkId, options) {
        const watermark = this._get(watermarkId);

        watermark.options = mergeOptions(watermark.options, options);
        if (watermark.api) {
            watermark.api.applyOptions(watermark.options);
        }
    }

    remove(watermarkId) {
        const watermark = this._watermarks[watermarkId];
        if (!watermark) {
            return false;
        }

        if (watermark.api) {
            watermark.api.detach();
        }

        delete this._watermarks[watermarkId];
        return true;
    }

    onPanesSwapped(first, second) {
        for (const watermark of Object.values(this._watermarks)) {
            if (watermark.pane === first) {
                watermark.pane = second;
            } else if (watermark.pane === second) {
                watermark.pane = first;
            }
        }
    }

    onPaneMoved(from, to) {
        if (from === to) {
            return;
        }

        for (const watermark of Object.values(this._watermarks)) {
            if (watermark.pane === from) {
                watermark.pane = to;
            } else if (from < to && watermark.pane > from && watermark.pane <= to) {
                watermark.pane -= 1;
            } else if (from > to && watermark.pane >= to && watermark.pane < from) {
                watermark.pane += 1;
            }
        }
    }

    onPaneRemoved(paneIndex) {
        for (const [watermarkId, watermark] of Object.entries(this._watermarks)) {
            if (watermark.pane === paneIndex) {
                if (watermark.api) {
                    watermark.api.detach();
                }
                delete this._watermarks[watermarkId];
            } else if (watermark.pane > paneIndex) {
                watermark.pane -= 1;
            }
        }
    }
}
