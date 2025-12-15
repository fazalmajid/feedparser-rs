use chrono::{DateTime, Datelike, Timelike, Utc, Weekday};
use pyo3::prelude::*;

/// Convert chrono::DateTime<Utc> to Python's time.struct_time
///
/// Returns a time.struct_time object compatible with feedparser's *_parsed fields.
/// The struct_time format is:
/// (tm_year, tm_mon, tm_mday, tm_hour, tm_min, tm_sec, tm_wday, tm_yday, tm_isdst)
///
/// Note: tm_wday is 0-6 where Monday=0, matching Python's time module.
/// Note: tm_isdst is always 0 for UTC times (DST not applicable).
pub fn datetime_to_struct_time(py: Python<'_>, dt: &DateTime<Utc>) -> PyResult<Py<PyAny>> {
    let time_module = py.import("time")?;
    let struct_time = time_module.getattr("struct_time")?;

    // Calculate day of week (Monday=0 in Python's time module)
    let weekday = match dt.weekday() {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    };

    // Create tuple with struct_time fields
    let tuple = (
        dt.year(),           // tm_year
        dt.month() as i32,   // tm_mon (1-12)
        dt.day() as i32,     // tm_mday (1-31)
        dt.hour() as i32,    // tm_hour (0-23)
        dt.minute() as i32,  // tm_min (0-59)
        dt.second() as i32,  // tm_sec (0-61)
        weekday,             // tm_wday (0-6, Monday is 0)
        dt.ordinal() as i32, // tm_yday (1-366)
        0i32,                // tm_isdst (0 = not DST, always 0 for UTC)
    );

    // Call time.struct_time(tuple)
    let result = struct_time.call1((tuple,))?;
    Ok(result.unbind())
}

/// Convert Option<DateTime<Utc>> to Option<time.struct_time>
///
/// Convenience wrapper for optional datetime fields.
pub fn optional_datetime_to_struct_time(
    py: Python<'_>,
    dt: &Option<DateTime<Utc>>,
) -> PyResult<Option<Py<PyAny>>> {
    match dt {
        Some(dt) => Ok(Some(datetime_to_struct_time(py, dt)?)),
        None => Ok(None),
    }
}

// NOTE: Datetime conversion to Python time.struct_time is tested via pytest
// in tests/test_datetime.py. Rust unit tests cannot be used here because
// cdylib crates don't link Python symbols - they're loaded at runtime by Python.
// The deprecated APIs (prepare_freethreaded_python, Python::with_gil) would
// fail to link in tests.
