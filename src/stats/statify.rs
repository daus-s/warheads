use crate::stats::stat_column::StatColumn;
use crate::stats::stat_value::StatValue;
use serde_json::Value;
use std::fmt::{Display, Formatter};

pub trait SafetyValve<T> {
    fn unwrap_fmt(&self, default: &str) -> String;
}
impl<T> SafetyValve<T> for Option<T>
where
    T: ToString,
{
    fn unwrap_fmt(&self, default: &str) -> String {
        match self {
            Some(t) => t.to_string(),
            None => default.to_string(),
        }
    }
}

pub struct StatPair(pub StatColumn, pub Value);

impl Display for StatPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            StatColumn::FG_PCT | StatColumn::FG3_PCT | StatColumn::FT_PCT => {
                match self.1.as_f64() {
                    Some(n) => write!(f, "{:.6}", n),
                    None => write!(f, "null"),
                }
            }
            StatColumn::FANTASY_PTS => match self.1.as_f64() {
                Some(n) => write!(f, "{:.1}", n),
                None => write!(f, "null"),
            },
            _ => {
                write!(f, "{}", self.1)
            }
        }
    }
}
