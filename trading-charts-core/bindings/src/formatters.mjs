// Formatters of prices and times.
//
// Lightweight Charts takes JS functions for its formatting options. The Rust side cannot put functions in the options,
// so it sends descriptors instead: objects with a `$priceFormatter`, `$timeFormatter` or `$tickMarkFormatter` key
// naming a predefined formatter, along with its parameters. `resolveOptions` replaces these descriptors with the
// matching functions before the options are given to Lightweight Charts.
//
// The `custom` formatters refer by name to functions registered with `registerCustomFunction`, which is how Rust
// closures are bridged.

const customFunctions = {
    price: new Map(),
    time: new Map(),
    tickMark: new Map(),
};

/** Register a named function usable by the `custom` formatters of the given kind (`price`, `time`, `tickMark`). */
export function registerCustomFunction(kind, name, fn) {
    const registry = customFunctions[kind];
    if (!registry) {
        throw new Error(`Unknown kind of custom function: '${kind}'`);
    }

    registry.set(name, fn);
}

/** Remove a named function. Returns `true` when the function was registered. */
export function unregisterCustomFunction(kind, name) {
    return customFunctions[kind]?.delete(name) ?? false;
}

function getCustomFunction(kind, name) {
    const fn = customFunctions[kind]?.get(name);
    if (!fn) {
        throw new Error(`No custom ${kind} function registered with the name '${name}'`);
    }

    return fn;
}

// ---------------------------------------------------------------------------------------------------------------------
// Prices
// ---------------------------------------------------------------------------------------------------------------------

function numberFormat(locale, options) {
    const formatter = new Intl.NumberFormat(locale || undefined, options);

    return value => formatter.format(value);
}

function fractionDigits(desc) {
    const options = {};

    if (Number.isFinite(desc.minimumFractionDigits)) {
        options.minimumFractionDigits = desc.minimumFractionDigits;
    }
    if (Number.isFinite(desc.maximumFractionDigits)) {
        options.maximumFractionDigits = desc.maximumFractionDigits;
    }

    return options;
}

const priceFormatterFactories = {
    fixed: desc => price => price.toFixed(desc.precision ?? 2),
    locale: desc => numberFormat(desc.locale, fractionDigits(desc)),
    currency: desc => numberFormat(desc.locale, Object.assign({
        style: 'currency',
        currency: desc.currency,
        currencyDisplay: desc.currencyDisplay || 'symbol',
    }, fractionDigits(desc))),
    percent: desc => price => `${price.toFixed(desc.precision ?? 2)}%`,
    compact: desc => numberFormat(desc.locale, Object.assign({ notation: 'compact' }, fractionDigits(desc))),
    scientific: desc => price => price.toExponential(desc.precision ?? 2),
    affix: desc => price => `${desc.prefix || ''}${price.toFixed(desc.precision ?? 2)}${desc.suffix || ''}`,
    custom: desc => getCustomFunction('price', desc.name),
};

function makePriceFormatter(desc) {
    const factory = priceFormatterFactories[desc.$priceFormatter];
    if (!factory) {
        throw new Error(`Unknown price formatter: '${desc.$priceFormatter}'`);
    }

    return factory(desc);
}

// ---------------------------------------------------------------------------------------------------------------------
// Times
// ---------------------------------------------------------------------------------------------------------------------

/** Convert a Lightweight Charts time (UTC timestamp in seconds, business day object or string) to a `Date`. */
export function timeToDate(time) {
    if (typeof time === 'number') {
        return new Date(time * 1000);
    }

    if (typeof time === 'string') {
        return new Date(`${time}T00:00:00Z`);
    }

    if (time && typeof time === 'object') {
        return new Date(Date.UTC(time.year, time.month - 1, time.day));
    }

    return new Date(NaN);
}

function pad(value, length = 2) {
    return `${value}`.padStart(length, '0');
}

const nameCache = new Map();

function names(locale, field, width) {
    const key = `${locale || ''}|${field}|${width}`;
    if (!nameCache.has(key)) {
        const formatter = new Intl.DateTimeFormat(locale || undefined, { [field]: width, timeZone: 'UTC' });
        const values = field === 'month'
            ? Array.from({ length: 12 }, (_, i) => formatter.format(new Date(Date.UTC(2021, i, 1))))
            // 2021-01-03 is a Sunday
            : Array.from({ length: 7 }, (_, i) => formatter.format(new Date(Date.UTC(2021, 0, 3 + i))));

        nameCache.set(key, values);
    }

    return nameCache.get(key);
}

const patternTokens = /'[^']*'|yyyy|yy|MMMM|MMM|MM|M|dd|d|EEEE|EEE|HH|H|hh|h|mm|ss|SSS|a/g;

/**
 * Format a date with a pattern, in UTC.
 *
 * Supported tokens: `yyyy`, `yy`, `MMMM`, `MMM`, `MM`, `M`, `dd`, `d`, `EEEE`, `EEE`, `HH`, `H`, `hh`, `h`, `mm`, `ss`,
 * `SSS` and `a`. Text between single quotes is copied as is.
 */
export function formatDatePattern(date, pattern, locale = undefined) {
    if (Number.isNaN(date.getTime())) {
        return '';
    }

    const hours = date.getUTCHours();

    return pattern.replace(patternTokens, token => {
        switch (token) {
            case 'yyyy':
                return `${date.getUTCFullYear()}`;
            case 'yy':
                return pad(date.getUTCFullYear() % 100);
            case 'MMMM':
                return names(locale, 'month', 'long')[date.getUTCMonth()];
            case 'MMM':
                return names(locale, 'month', 'short')[date.getUTCMonth()];
            case 'MM':
                return pad(date.getUTCMonth() + 1);
            case 'M':
                return `${date.getUTCMonth() + 1}`;
            case 'dd':
                return pad(date.getUTCDate());
            case 'd':
                return `${date.getUTCDate()}`;
            case 'EEEE':
                return names(locale, 'weekday', 'long')[date.getUTCDay()];
            case 'EEE':
                return names(locale, 'weekday', 'short')[date.getUTCDay()];
            case 'HH':
                return pad(hours);
            case 'H':
                return `${hours}`;
            case 'hh':
                return pad(hours % 12 || 12);
            case 'h':
                return `${hours % 12 || 12}`;
            case 'mm':
                return pad(date.getUTCMinutes());
            case 'ss':
                return pad(date.getUTCSeconds());
            case 'SSS':
                return pad(date.getUTCMilliseconds(), 3);
            case 'a':
                return hours < 12 ? 'AM' : 'PM';
            default:
                return token.slice(1, -1);
        }
    });
}

function intlDateTimeOptions(desc) {
    const options = { timeZone: 'UTC' };

    for (const name of ['dateStyle', 'timeStyle', 'hour12']) {
        if (desc[name] !== undefined && desc[name] !== null) {
            options[name] = desc[name];
        }
    }

    if (!options.dateStyle && !options.timeStyle) {
        options.dateStyle = 'medium';
        options.timeStyle = 'short';
    }

    return options;
}

// The times given to the predefined time formatters are the displayed times: when the chart has a time zone, the
// data times are shifted so that their UTC representation is the wall-clock time of the time zone. The predefined
// formatters therefore always format in UTC. The custom functions receive the real UTC timestamps instead, given by
// `context.toUtc`.

const timeFormatterFactories = {
    pattern: desc => time => formatDatePattern(timeToDate(time), desc.pattern, desc.locale),
    intl: desc => {
        const formatter = new Intl.DateTimeFormat(desc.locale || undefined, intlDateTimeOptions(desc));

        return time => {
            const date = timeToDate(time);

            return Number.isNaN(date.getTime()) ? '' : formatter.format(date);
        };
    },
    custom: (desc, context) => {
        const fn = getCustomFunction('time', desc.name);

        return time => fn(context.toUtc(time));
    },
};

function makeTimeFormatter(desc, context) {
    const factory = timeFormatterFactories[desc.$timeFormatter];
    if (!factory) {
        throw new Error(`Unknown time formatter: '${desc.$timeFormatter}'`);
    }

    return factory(desc, context);
}

const tickMarkPatternNames = ['year', 'month', 'dayOfMonth', 'time', 'timeWithSeconds'];

const tickMarkFormatterFactories = {
    patterns: desc => (time, tickMarkType) => {
        const pattern = desc[tickMarkPatternNames[tickMarkType]];

        return pattern ? formatDatePattern(timeToDate(time), pattern, desc.locale) : null;
    },
    custom: (desc, context) => {
        const fn = getCustomFunction('tickMark', desc.name);

        return (time, tickMarkType) => fn(context.toUtc(time), tickMarkType) ?? null;
    },
};

function makeTickMarkFormatter(desc, context) {
    const factory = tickMarkFormatterFactories[desc.$tickMarkFormatter];
    if (!factory) {
        throw new Error(`Unknown tick mark formatter: '${desc.$tickMarkFormatter}'`);
    }

    return factory(desc, context);
}

// ---------------------------------------------------------------------------------------------------------------------
// Options
// ---------------------------------------------------------------------------------------------------------------------

const identityContext = {
    toUtc: time => time,
};

function resolveValue(key, value, context) {
    if (Array.isArray(value)) {
        return value.map(item => resolveValue(null, item, context));
    }

    if (!value || typeof value !== 'object') {
        return value;
    }

    if ('$priceFormatter' in value) {
        const formatter = makePriceFormatter(value);

        // The tick mark formatters of the price scale receive all the prices at once.
        return key && key.startsWith('tickmarks') ? prices => prices.map(formatter) : formatter;
    }

    if ('$timeFormatter' in value) {
        return makeTimeFormatter(value, context);
    }

    if ('$tickMarkFormatter' in value) {
        return makeTickMarkFormatter(value, context);
    }

    const result = {};
    for (const [name, item] of Object.entries(value)) {
        result[name] = resolveValue(name, item, context);
    }

    return result;
}

/**
 * Return a copy of the options where the formatter descriptors are replaced by functions.
 *
 * The context gives `toUtc(time)`, converting a displayed time to the real UTC time, for the custom functions.
 */
export function resolveOptions(options, context = identityContext) {
    return resolveValue(null, options, context);
}
