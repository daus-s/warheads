use chrono::{DateTime, Datelike, Local, NaiveDate};
use serde_json::Value;
use serde_json::Value::Null;
use crate::format::extract::{get_result_set, headers, get_rows};
use crate::stats::types::GameResult;
use crate::stats::types::GameResult::{Draw, Loss, Win};

pub fn parse_string(s: Option<&Value>) -> String {
    s.unwrap_or(&Null).to_string().replace("\"", "")
}

pub fn parse_u64(value: Option<&Value>) -> Option<u64> {
    match value {
        Some(v) => match v.as_u64() {
            Some(x) => Some(x),
            None => None,
        },
        None => None,
    }
}

pub fn parse_u32(value: Option<&Value>) -> Option<u32> {
    match value {
        Some(v) => match v.as_u64() {
            Some(x) => {
                if x <= u32::MAX as u64 {
                    Some(x as u32)
                } else {
                    None
                }
            }
            None => None,
        },
        None => None,
    }
}

pub fn parse_i32(value: Option<&Value>) -> Option<i32> {
    match value {
        Some(v) => match v.as_i64() {
            Some(x) => {
                if x >= i32::MIN as i64 && x <= i32::MAX as i64 {
                    Some(x as i32)
                } else {
                    None
                }
            }
            None => None,
        },
        None => None,
    }
}

pub fn parse_f32(value: Option<&Value>) -> Option<f32> {
    match value {
        Some(v) => match v.as_f64() {
            Some(x) => Some(x as f32),
            None => None,
        },
        None => None,
    }
}

pub fn str_to_num(value: Option<&Value>) -> u64 {
    value
        .unwrap()
        .as_str()
        .unwrap()
        .parse::<u64>()
        .expect(format!("could not parse {:?} into unsigned 64-bit integer\n", value).as_str())
}

pub fn parse_wl(value: Option<&Value>) -> Option<GameResult> {
    match value {
        // the fuck?
        Some(wl) => match wl.as_str() {
            Some("W") => Some(Win),
            Some("L") => Some(Loss),
            Some("D") => Some(Draw),
            Some(x) => panic!(
                "Unknown game result: {}. Acceptable results are: [\"W\", \"L\", \"D\"]",
                x
            ),
            None => None,
        },
        None => panic!(
            "could not unwrap a game result from the provided serde::Value {:#?}",
            value
        ),
    }
}

pub fn parse_date(value: Option<&Value>) -> Option<NaiveDate> {
    let json_date = value.unwrap();

    let date_str = json_date.to_string().replace("\"", "");

    NaiveDate::parse_from_str(&*date_str, "%Y-%m-%d").ok()
}

pub fn destructure_dt(dt: DateTime<Local>) -> DT {
    DT {
        year: dt.year(),
        month: dt.month(),
        day: dt.day(),
    }
}

pub struct DT {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}


pub fn parse_season(value: Value) -> (Vec<Value>, Vec<String>) {
    let set = get_result_set(&value).unwrap_or_else(|err| panic!("{}", err));

    let headers: Vec<String> = headers(&set).unwrap();

    let rows: Vec<Value> = get_rows(&set).unwrap();

    (rows, headers)
}