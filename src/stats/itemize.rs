use serde_json::Value;

pub trait Itemize {
    fn itemize(&self) -> Vec<Value>;
}
