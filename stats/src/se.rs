use serde_json::Value;

pub trait SerdeEnum {
    type Item: ToString;

    fn enumerate() -> Vec<Self::Item>;

    fn evaluate() -> Vec<Value>;
}
