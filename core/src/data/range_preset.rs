use serde::{Deserialize, Serialize};

/// Range of the data to show, relative to the last data item of the chart, for
/// [`ChartHandle::show_range`](crate::ChartHandle::show_range).
///
/// The calendar ranges (days, weeks, months, years, year to date) are computed in UTC.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "count", rename_all = "camelCase")]
pub enum RangePreset {
    /// The last days.
    Days(u32),

    /// The last weeks.
    Weeks(u32),

    /// The last months.
    Months(u32),

    /// The last years.
    Years(u32),

    /// From the start of the year of the last data item.
    YearToDate,

    /// The last bars.
    Bars(u32),

    /// All the data.
    All,
}
