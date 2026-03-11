use crate::ml::model::Model;

type ModelFactory = fn() -> Box<dyn Model>;

pub struct Registration {
    pub model_name: &'static str,
    pub factory: ModelFactory,
}

inventory::collect!(Registration);
