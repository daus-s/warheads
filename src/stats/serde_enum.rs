use serde_json::Value;

pub trait SerdeEnum {
    type Item: ToString;

    fn items() -> Vec<Self::Item>;

    fn values() -> Vec<Value>;

    fn evaluate(&self) -> Value;
}
