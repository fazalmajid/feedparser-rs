use chrono::{DateTime, Datelike, Timelike, Utc, Weekday};
use pyo3::prelude::*;

/// Convert DateTime<Utc> to Python time.struct_time for feedparser compatibility
pub fn datetime_to_struct_time(py: Python<'_>, dt: &DateTime<Utc>) -> PyResult<Py<PyAny>> {
    let time_module = py.import("time")?;
    let struct_time = time_module.getattr("struct_time")?;

    // Monday=0 in Python's time module
    let weekday = match dt.weekday() {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    };

    let tuple = (
        dt.year(),
        dt.month() as i32,
        dt.day() as i32,
        dt.hour() as i32,
        dt.minute() as i32,
        dt.second() as i32,
        weekday,
        dt.ordinal() as i32,
        0i32, // tm_isdst (0 for UTC)
    );

    let result = struct_time.call1((tuple,))?;
    Ok(result.unbind())
}

pub fn optional_datetime_to_struct_time(
    py: Python<'_>,
    dt: &Option<DateTime<Utc>>,
) -> PyResult<Option<Py<PyAny>>> {
    match dt {
        Some(dt) => Ok(Some(datetime_to_struct_time(py, dt)?)),
        None => Ok(None),
    }
}
