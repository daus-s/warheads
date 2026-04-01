use crate::ml::model::Model;

pub struct LogisiticPerformance {}

impl Model for LogisiticPerformance {
    fn model_name(&self) -> String {
        "log-model v1".to_owned()
    }

    fn initialize(&mut self) -> Result<(), ()> {
        todo!()
    }

    fn train(
        &mut self,
        data: &[(
            crate::stats::gamecard::GameCard,
            crate::stats::game_obj::GameObject,
        )],
    ) {
        todo!()
    }

    fn evaluate(&self) -> std::collections::HashMap<String, f64> {
        todo!()
    }

    fn predict(&mut self, obj: &crate::stats::gamecard::GameCard) -> f64 {
        todo!()
    }
}
