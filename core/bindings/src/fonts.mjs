// Web font loading.
//
// Lightweight Charts draws its texts on canvases: a font which is not loaded yet when the chart is drawn falls back
// to another font, and the chart is not redrawn when the font becomes available. `loadWebFont` loads a font and then
// refreshes the charts which are alive.

const liveCharts = new Set();

/** Track a chart, to refresh it when a web font is loaded. */
export function trackChart(chart) {
    liveCharts.add(chart);
}

export function untrackChart(chart) {
    liveCharts.delete(chart);
}

function fontSource(source) {
    return /^\s*(url|local)\(/.test(source) ? source : `url(${JSON.stringify(source)})`;
}

/**
 * Load a web font and add it to the document, then refresh the live charts.
 *
 * The source is either a font file URL or a CSS `src` value (`url(...)` or `local(...)`). The returned promise is
 * resolved with `true` once the font is loaded, or with `false` when the font cannot be loaded, the error being
 * reported on the console.
 */
export function loadWebFont(family, source, descriptors = null) {
    const face = new FontFace(family, fontSource(source), descriptors || {});

    return face.load()
        .then(loaded => {
            document.fonts.add(loaded);
            for (const chart of liveCharts) {
                chart.refreshFonts();
            }

            return true;
        })
        .catch(err => {
            console.error(`Failed to load the web font '${family}' from '${source}':`, err);

            return false;
        });
}
