use std::collections::HashMap;

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
}

impl NelderMeadEloTracker {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
            is_trained: false,
            params: Vector::from(vec![32., 400.]),
            performance: 0.0,
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

        let cost = |v: &Vector| -> f64 {
            let mut tracker = EloTracker::params(EloParams::new(v));

            tracker.train(games);

            println!(
                "{}/{}=>{}",
                tracker.freq(),
                tracker.log_loss(),
                tracker.evaluate()
            );

            tracker.evaluate()
        };

        let baseline = 0.46304378813918995;

        while self.performance < baseline * 0.7 {
            nelder_mead(cost, &mut simplex);

            for v in simplex.iter() {
                let new_performance = cost(v);
                let mut hash: u128 = 0;

                hash |= v.x().to_bits() as u128;
                hash |= v.y().to_bits() as u128 >> 64;

                self.mapping.insert(hash, new_performance);

                if new_performance > self.performance {
                    self.performance = new_performance;
                }
            }
        }

        println!(
            "Score: {}\nBaseline: {}\nstep: {}\tscale factor: {}",
            self.performance,
            baseline,
            self.params.x(),
            self.params.y()
        );
    }

    fn model_name(&self) -> String {
        "nelder-mead elo v1".to_string()
    }

    fn predict(&mut self, _obj: &crate::stats::gamecard::GameCard) -> f64 {
        if !self.is_trained {
            panic!("💀 model not trained");
        }
        todo!()
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

    #[allow(dead_code)]
    fn get_optimal_params() {
        let mut tracker = NelderMeadEloTracker::new();

        let training_data = Chronology::new()
            .as_training_data()
            .expect("failed to load data when testing nelder-mead-elo algorithm");

        tracker.train(&training_data);

        assert!(tracker.performance > 0.46304378813918995 * 0.7);
    }
}
