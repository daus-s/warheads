use std::collections::HashMap;

use crate::ml::model::Model;
use crate::ml::simplex::Simplex;
use crate::ml::vector::Vector;

use crate::stats::game_obj::GameObject;
use crate::stats::gamecard::GameCard;

/// this elo algorithm is optimized on (k, f) pairs,
/// the algorithm also uses a initial rating of 0 for symmetry.
pub struct OptimizedElo {
    mapping: HashMap<(u8, u16), f64>,
    is_trained: bool,
}

impl Model for OptimizedElo {
    fn train(&mut self, games: &[(GameCard, GameObject)]) {
        let simplex = Simplex::from(&vec![
            Vector::from(vec![32., 400.]),
            Vector::from(vec![24., 400.]),
            Vector::from(vec![32., 500.]),
        ]);

        todo!("Implement training logic")
    }

    fn model_name(&self) -> String {
        "nelder-mead elo v1".to_string()
    }

    fn predict(&mut self, obj: &crate::stats::gamecard::GameCard) -> f64 {
        if !self.is_trained {
            panic!("💀 model not trained");
        }
        todo!()
    }

    fn evaluate(&self) -> f64 {
        todo!()
    }
}
