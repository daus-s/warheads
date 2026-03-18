use crate::ml::model::Model;

type ModelFactory = fn(&clap::ArgMatches) -> Box<dyn Model>;

pub struct Registration {
    pub model_name: &'static str,
    pub args_schema: fn() -> clap::Command,
    pub factory: ModelFactory,
}

inventory::collect!(Registration);
