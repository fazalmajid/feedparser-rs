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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_datetime_to_struct_time() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            // 2025-12-15 14:30:00 UTC (Monday)
            let dt = Utc.with_ymd_and_hms(2025, 12, 15, 14, 30, 0).unwrap();
            let st = datetime_to_struct_time(py, &dt).unwrap();

            // Verify it's a time.struct_time
            let time_module = py.import("time").unwrap();
            let struct_time_type = time_module.getattr("struct_time").unwrap();
            assert!(st.bind(py).is_instance(&struct_time_type).unwrap());

            // Extract fields
            let st_obj = st.bind(py);
            assert_eq!(st_obj.getattr("tm_year").unwrap().extract::<i32>().unwrap(), 2025);
            assert_eq!(st_obj.getattr("tm_mon").unwrap().extract::<i32>().unwrap(), 12);
            assert_eq!(st_obj.getattr("tm_mday").unwrap().extract::<i32>().unwrap(), 15);
            assert_eq!(st_obj.getattr("tm_hour").unwrap().extract::<i32>().unwrap(), 14);
            assert_eq!(st_obj.getattr("tm_min").unwrap().extract::<i32>().unwrap(), 30);
            assert_eq!(st_obj.getattr("tm_sec").unwrap().extract::<i32>().unwrap(), 0);
            assert_eq!(st_obj.getattr("tm_wday").unwrap().extract::<i32>().unwrap(), 0); // Monday
            assert_eq!(st_obj.getattr("tm_isdst").unwrap().extract::<i32>().unwrap(), 0); // UTC
        });
    }

    #[test]
    fn test_optional_datetime_none() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let dt: Option<DateTime<Utc>> = None;
            let result = optional_datetime_to_struct_time(py, &dt).unwrap();
            assert!(result.is_none());
        });
    }

    #[test]
    fn test_optional_datetime_some() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let dt = Some(Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap());
            let result = optional_datetime_to_struct_time(py, &dt).unwrap();
            assert!(result.is_some());
        });
    }
}
