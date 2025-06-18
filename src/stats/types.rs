use crate::stats::se::SerdeEnum;
use serde_json::{json, Value};

/// BoolInt is an int that is either 1 or 0
/// to represent a boolean value

pub struct BoolInt(pub u8);

impl BoolInt {
    // Constructor that validates the input
    pub fn new(value: u8) -> Self {
        if value != 0 && value != 1 {
            panic!("BoolInt can only be 0 or 1");
        }
        BoolInt(value)
    }
    pub fn get(&self) -> u8 {
        self.0
    }
}

impl SerdeEnum for BoolInt {
    type Item = u64;

    fn items() -> Vec<Self::Item> {
        vec![0, 1]
    }

    fn values() -> Vec<Value> {
        vec![json!(0), json!(1)]
    }

    fn evaluate(&self) -> Value {
        debug_assert!(
            self.0 == 0 || self.0 == 1,
            "BoolInt can only be 0 or 1, got {}",
            self.0
        );

        json!(self.0)
    }
}

impl SerdeEnum for bool {
    type Item = bool;

    fn items() -> Vec<Self::Item> {
        vec![true, false]
    }

    fn values() -> Vec<Value> {
        vec![json!(true), json!(false)]
    }

    fn evaluate(&self) -> Value {
        json!(self)
    }
}
