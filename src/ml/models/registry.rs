use std::collections::HashMap;

use crate::ml::model::Model;

type ModelFactory = fn() -> Box<dyn Model>;

pub struct Registry {
    models: HashMap<String, ModelFactory>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, factory: ModelFactory) {
        self.models.insert(name.to_string(), factory);
    }
}
