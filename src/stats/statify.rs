use std::fmt::{Display, Formatter};
use crate::stats::stat_column::StatColumn;
use crate::stats::stat_value::StatValue;

pub trait SafetyValve<T> {
    fn unwrap_f(&self, default: &str) -> String;
}
impl<T> SafetyValve<T> for Option<T>
where
    T: ToString,
{
    fn unwrap_f(&self, default: &str) -> String {
        match self {
            Some(t) => t.to_string(),
            None => default.to_string(),
        }
    }
}

pub struct StatPair(pub StatColumn, pub StatValue);

impl Display for StatPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        match self.0 {
            StatColumn::FG_PCT  |
            StatColumn::FG3_PCT |
            StatColumn::FT_PCT  =>
                write!(f, "{:.6}", self.1.value().as_f64().unwrap()),

            StatColumn::FANTASY_PTS =>
                write!(f, "{:.1}", self.1.value().as_f64().unwrap()),

            _ =>
                write!(f, "{}", self.1),
        }
    }
}
