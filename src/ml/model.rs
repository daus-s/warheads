use crate::stats::game_obj::GameObject;
use crate::stats::gamecard::GameCard;

pub trait Model {
    fn model_name(&self) -> String;

    fn train(&mut self, data: &[(GameCard, GameObject)]);
    fn evaluate(&self) -> f64; // this could return a vec of measurements or a like structure
    fn predict(&mut self, obj: &GameCard) -> f64;
}
// the idea is
// model name is
// predict - given some trait like predictable or something?
// evaluate - calculate log loss or residuals or something? calculate the Objective. Create an ObjectiveTrait that does somethign idk
//
