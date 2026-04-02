use crate::ml::model;

use crate::stats::chronology::Chronology;

pub struct SigmaChadModel {}

impl model::Model for SigmaChadModel {
    fn model_name(&self) -> String {
        "sigma v1".to_owned()
    }

    fn initialize(&mut self) -> Result<(), ()> {
        todo!()
    }

    fn train(&mut self, chrono: Chronology) -> Result<(), model::TrainingError> {
        todo!()
    }

    fn evaluate(&self) -> std::collections::HashMap<String, f64> {
        todo!()
    }

    fn predict(&mut self, obj: &crate::stats::gamecard::GameCard) -> f64 {
        todo!()
    }
}
