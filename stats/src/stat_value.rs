use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::Value::Null;

///
/// StatValue
///
/// in code this returns an option of the value when calling val to copy it so
/// as it can be treated as `None` in match case statements.
///
///
#[derive(Serialize, Deserialize, Eq, PartialEq, Derivative, Clone)]
#[derivative(Hash)]
pub struct StatValue (Value);

impl StatValue {
    pub fn new() -> StatValue {
        StatValue( Null )
    }
    pub fn with_value(val: Value) -> StatValue {
        StatValue(
            val
        )
    }

    pub fn val(&self) -> Option<Value> {
        match &self.0 {
            Null => None,
            Value::Bool(_)   |
            Value::Number(_) |
            Value::String(_) |
            Value::Array(_)  |
            Value::Object(_) => {
                Some(self.0.clone())
            },
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
}
