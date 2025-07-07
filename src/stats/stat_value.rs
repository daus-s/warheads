use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::Value::Null;
use std::fmt::{Display, Formatter};

///
/// StatValue
///
/// in code this returns an option of the value when calling val to copy it so
/// as it can be treated as `None` in match case statements.
///
///
#[derive(Serialize, Deserialize, Eq, PartialEq, Derivative, Clone)]
#[derivative(Hash)]
pub struct StatValue(Value);

impl StatValue {
    pub fn new() -> Self {
        StatValue(Null)
    }
    pub fn from_value(val: Value) -> Self {
        StatValue(val)
    }

    pub fn val(&self) -> Option<Value> {
        match &self.0 {
            Null => None,
            Value::Bool(_)
            | Value::Number(_)
            | Value::String(_)
            | Value::Array(_)
            | Value::Object(_) => Some(self.0.clone()),
        }
    }

    pub fn value(&self) -> Value {
        self.0.clone()
    }

    pub fn set(&mut self, arg: Value) {
        self.0 = arg;
    }

    pub fn clear(&mut self) {
        self.0 = Null
    }

    pub fn to_season_id(&self) -> Option<i32> {
        match &self.0 {
            Value::Number(i) => match i.as_i64() {
                Some(i) => Some(i as i32),
                None => None,
            },
            _ => None,
        }
    }
}

impl Display for StatValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
