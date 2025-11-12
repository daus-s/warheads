use crate::stats::game_obj::GameObject;

pub trait Model {
    fn model_name(&self) -> String;
    fn predict(&mut self, obj: &GameObject) -> f64;
}
