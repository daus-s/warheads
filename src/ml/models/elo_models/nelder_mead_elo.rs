use std::collections::HashMap;
use std::io::{self, Write};

use crate::ml::elo_params::EloParams;
use crate::ml::elo_tracker::EloTracker;
use crate::ml::model::Model;
use crate::ml::nelder_mead::nelder_mead;
use crate::ml::simplex::Simplex;
use crate::ml::vector::Vector;

use crate::stats::game_obj::GameObject;
use crate::stats::gamecard::GameCard;

/// this elo algorithm is optimized on (k, f) pairs,
/// the algorithm also uses a initial rating of 0 for symmetry.
pub struct NelderMeadEloTracker {
    mapping: HashMap<u128, f64>,
    is_trained: bool,
    params: Vector,
    performance: f64,
    model: Option<EloTracker>,
}

impl NelderMeadEloTracker {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
            is_trained: false,
            params: Vector::from(vec![32., 400.]),
            performance: 0.0,
            model: None,
        }
    }
}

impl Model for NelderMeadEloTracker {
    fn train(&mut self, games: &[(GameCard, GameObject)]) {
        let mut simplex = Simplex::from(&vec![
            Vector::from(vec![32., 400.]),
            Vector::from(vec![24., 400.]),
            Vector::from(vec![32., 500.]),
        ]);

        let mut cost = |v: &Vector| -> f64 {
            let mut hash: u128 = 0;
            hash |= v.x().to_bits() as u128;
            hash |= (v.y().to_bits() as u128) << 64;

            if let Some(&cached) = self.mapping.get(&hash) {
                return cached;
            }

            let mut tracker = EloTracker::params(EloParams::new(v));
            tracker.train(games);

            let result = tracker.evaluate();
            self.mapping.insert(hash, result);
            result
        };

        let baseline = cost(&Vector::from(vec![32., 400.]));

        while self.performance < baseline * 1.05 {
            //minimum improvement of 5%
            nelder_mead(&mut cost, &mut simplex);

            for params in simplex.iter() {
                let new_performance = cost(params);

                if new_performance > self.performance {
                    self.performance = new_performance;
                    self.params = params.clone();
                    self.model = Some(EloTracker::params(EloParams::new(params)));
                }
            }
            print!(
                "\rScore: {:.4} | Baseline: {:.4} | step: {:.4} | scale: {:.4}",
                self.performance,
                baseline,
                self.params.x(),
                self.params.y()
            );
            io::stdout().flush().unwrap();
        }
        println!();
    }

    fn model_name(&self) -> String {
        "nelder-mead elo".to_string()
    }

    fn predict(&mut self, card: &crate::stats::gamecard::GameCard) -> f64 {
        if !self.is_trained {
            panic!("💀 model not trained");
        }
        self.model.as_mut().unwrap().predict(card)
    }

    fn evaluate(&self) -> f64 {
        if !self.is_trained {
            panic!("💀 model not trained");
        }

        self.performance
    }
}

#[cfg(test)]
mod test_nelder_mead_elo {
    use crate::stats::chronology::Chronology;

    use super::*;

    #[test]
    fn get_optimal_params() {
        let mut tracker = NelderMeadEloTracker::new();

        let training_data = Chronology::new()
            .as_training_data()
            .expect("failed to load data when testing nelder-mead-elo algorithm");

        tracker.train(&training_data);

        assert!(tracker.performance > 0.9708); //baseline from 32, 400
    }
}
