use crate::format::extract::{get_result_set, get_rows, headers};
use chrono::{DateTime, Datelike, Local, NaiveDate};
use serde_json::Value;

pub fn value_to_date(value: &Value) -> Option<NaiveDate> {
    match value {
        Value::String(s) => NaiveDate::parse_from_str(&*s, "%Y-%m-%d").ok(),
        _ => {
            eprintln!("⚠️ JSON Value to parse GameDate from is not a String. ");

            None
        }
    }
}
/// # returns
///
/// - month:
///   -  Returns the month number starting from 1.
///   -  The return value ranges from 1 to 12.
/// ________________________________________________
/// - day:
///   -  Returns the day of month starting from 1.
///   -  The return value ranges from 1 to 31. (The last day of month differs by months.)
/// ________________________________________________
/// - year:
///   -  Returns the year number in the calendar date.
pub fn destructure_dt(dt: DateTime<Local>) -> DestructuredDateTime {
    DestructuredDateTime {
        year: dt.year(),
        month: dt.month(),
        day: dt.day(),
    }
}

pub struct DestructuredDateTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

pub fn parse_season(value: Value) -> (Vec<Value>, Vec<String>) {
    let set = get_result_set(&value)
        .unwrap_or_else(|err| panic!("💀 could not unwrap result set: {err}"));

    let headers: Vec<String> =
        headers(&set).unwrap_or_else(|err| panic!("💀 could not unwrap headers from set: {err}"));

    let rows: Vec<Value> =
        get_rows(&set).unwrap_or_else(|err| panic!("💀 could not unwrap rows from set: {err}"));

    (rows, headers)
}
