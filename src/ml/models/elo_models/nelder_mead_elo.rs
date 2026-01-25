use std::collections::HashMap;
use std::f64::INFINITY;
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
    params: EloParams,
    cost: f64,
}

impl NelderMeadEloTracker {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
            is_trained: false,
            params: EloParams::try_from(&Vector::from(vec![32., 400.]))
                .expect("💀 failed to construct elo parameters from default vector"),
            cost: INFINITY,
        }
    }
}

fn delta(x1: &Simplex, x2: &Simplex) -> (Vector, Vector) {
    let mut delta = Vector::origin(2);
    let mut changed = Vector::origin(2);

    for v1 in x1.vertices() {
        let mut found = false;
        for v2 in x2.vertices() {
            if v1 == v2 {
                found = true;
                break;
            }
        }

        if !found {
            changed = v1.clone();
            delta -= v1;
        }
    }

    for v2 in x2.vertices() {
        let mut found = false;
        for v1 in x1.vertices() {
            if v1 == v2 {
                found = true;
                break;
            }
        }

        if !found {
            delta += v2;
        }
    }

    (changed, delta)
}

impl Model for NelderMeadEloTracker {
    fn train(&mut self, games: &[(GameCard, GameObject)]) {
        let mut simplex = Simplex::from(&vec![
            Vector::from(vec![32., 400.]),
            Vector::from(vec![31., 400.]),
            Vector::from(vec![32., 410.]),
        ]);

        let mut cost = |v: &Vector| -> f64 {
            let mut hash: u128 = 0;
            hash |= v.x().to_bits() as u128;
            hash |= (v.y().to_bits() as u128) << 64;

            if let Some(&cached) = self.mapping.get(&hash) {
                return cached;
            }

            let performance = if let Ok(params) = EloParams::try_from(v) {
                let mut tracker = EloTracker::with(params);
                tracker.train(games);

                let result = 1. / tracker.evaluate();
                self.mapping.insert(hash, result);
                result
            } else {
                self.mapping.insert(hash, INFINITY);

                INFINITY
            };

            performance
        };

        let baseline = cost(&Vector::from(vec![32., 400.]));

        //todo: implement
        // if the changes in the point are less than 0.001x of the original, so 0.1%, stop optimizing as (k, f) is likely a good candidate
        //
        //
        //
        let mut prev = simplex.clone();

        while self.cost > baseline * 0.95 {
            nelder_mead(&mut cost, &mut simplex);

            for argv in simplex.vertices() {
                let new_cost = cost(argv);

                if new_cost < self.cost {
                    self.cost = new_cost;
                    self.params =
                        EloParams::try_from(argv).expect("💀 optimal parameters must be valid. ")
                }

                print!(
                    "\rScore: {:.4} ({}, {}) | Optimum: {:.4} ({}, {})| Baseline: {:.4} (32, 400)",
                    new_cost,
                    argv.x(),
                    argv.y(),
                    self.cost,
                    self.params.step(),
                    self.params.scale_factor(),
                    baseline,
                );
                io::stdout().flush().unwrap();
            }

            let (v, d) = delta(&prev, &simplex);
            if d.norm() < 0.001 * v.norm() {
                break;
            }
            prev = simplex.clone();
        }
    }

    fn model_name(&self) -> String {
        "nelder-mead elo".to_string()
    }

    fn predict(&mut self, _card: &crate::stats::gamecard::GameCard) -> f64 {
        if !self.is_trained {
            panic!("💀 model not trained");
        }
        todo!()
    }

    fn evaluate(&self) -> f64 {
        if !self.is_trained {
            panic!("💀 model not trained");
        }

        self.cost
    }
}

#[cfg(test)]
mod test_nelder_mead_elo {
    use crate::stats::chronology::Chronology;

    use super::*;

    #[test]
    fn nelder_mead_optimization() {
        let mut tracker = NelderMeadEloTracker::new();

        let training_data = Chronology::new()
            .as_training_data()
            .expect("failed to load data when testing nelder-mead-elo algorithm");

        tracker.train(&training_data);

        assert!(tracker.cost > 0.9708); //baseline from 32, 400
    }
}
