use crate::stats::gamecard::GameCard;

pub trait Model {
    fn get_model_name(&self) -> String;
    fn predict(&self, card: &GameCard) -> f64;
}
