use std::collections::HashMap;
use std::f64::INFINITY;
use std::fs::{create_dir_all, File};
use std::io::{self, Write};

use crate::format::path_manager;
use crate::ml::elo::elo_params::EloParams;
use crate::ml::elo::elo_tracker::EloTracker;
use crate::ml::model::Model;
use crate::ml::nelder_mead::nelder_mead;
use crate::ml::simplex::{self, Simplex};
use crate::ml::vector::Vector;

use crate::stats::game_obj::GameObject;
use crate::stats::gamecard::GameCard;

/// this elo algorithm is optimized on (k, f) pairs,
/// the algorithm also uses a initial rating of 0 for symmetry.
pub struct NelderMeadEloTracker {
    mapping: HashMap<HashKey, Results>,
    mesh: Vec<(Vector, Results)>,
    is_trained: bool,
    params: EloParams,
    cost: f64,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct HashKey(u128);

impl HashKey {
    pub fn from_vec(vec: &Vector) -> Self {
        assert!(vec.dim() == 2, "💀 nelder-mead elo hash is the bitwise concatenation of 2 64-bit floats. unexpected number of elements: {}", vec.dim());

        Self((vec.x().to_bits() as u128) << 64 | vec.y().to_bits() as u128)
    }

    #[allow(dead_code)]
    pub fn to_vec(&self) -> Vector {
        let x = f64::from_ne_bytes(((self.0 >> 64) as u64).to_ne_bytes());
        let y = f64::from_ne_bytes(((self.0 & 0xFFFFFFFFFFFFFFFF) as u64).to_ne_bytes());
        Vector::from(vec![x, y])
    }
}

#[derive(Debug, Clone, Copy)]
struct Results {
    freq: f64,
    log_loss: f64,
}

impl Results {
    pub fn new(freq: f64, log_loss: f64) -> Self {
        Self { freq, log_loss }
    }

    pub fn cost(&self) -> f64 {
        self.log_loss / self.freq
    }

    #[allow(dead_code)]
    pub fn reward(&self) -> f64 {
        self.freq / self.log_loss
    }
}

impl NelderMeadEloTracker {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
            mesh: vec![],
            is_trained: false,
            params: EloParams::try_from(&Vector::from(vec![32., 400.]))
                .expect("💀 failed to construct elo parameters from default vector"),
            cost: INFINITY,
        }
    }

    //save results
    pub fn serialize(&self) -> Result<(), io::Error> {
        let dir = path_manager::model_directory(self);

        create_dir_all(&dir)?;

        let iterations = dir.join("iterations.csv");

        let mut iter_file = File::create(&iterations)?;

        writeln!(iter_file, "step,scale_factor,freq,log_loss")?;

        for (hash, result) in &self.mapping {
            let v = hash.to_vec();
            writeln!(
                iter_file,
                "{},{},{:.6},{:.6}",
                v.x(),
                v.y(),
                result.freq,
                result.log_loss
            )?;
        }

        let mesh = dir.join("mesh.csv");
        let mut mesh_file = File::create(&mesh)?;

        writeln!(iter_file, "step,scale_factor,freq,log_loss")?;

        for (hash, result) in &self.mapping {
            let v = hash.to_vec();
            writeln!(
                mesh_file,
                "{},{},{:.6},{:.6}",
                v.x(),
                v.y(),
                result.freq,
                result.log_loss
            )?;
        }

        writeln!(mesh_file, "step,scale_factor,freq,log_loss")?;

        for (v, result) in &self.mesh {
            writeln!(
                mesh_file,
                "{},{},{:.6},{:.6}",
                v.x(),
                v.y(),
                result.freq,
                result.log_loss
            )?;
        }

        Ok(())
    }

    fn compute_mesh(&mut self, games: &[(GameCard, GameObject)]) {
        let mut mesh = Vec::new();

        for k in 10..=100 {
            for f in (100..=1000).step_by(10) {
                if f < 10 * k || f > 100 * k {
                    continue;
                }
                let v = Vector::from(vec![k as f64, f as f64]);

                //if this has errors it is pure skill issues coding
                let mut tracker = EloTracker::with(EloParams::try_from(&v).expect(
                    "💀 invalid mesh parameters. expected a solution to the inequality 0 < k, f < 0, 10k <  f <= 100k",
                ));
                tracker.train(games);

                let result = Results::new(tracker.freq(), tracker.log_loss());
                mesh.push((v, result));
            }
        }

        self.mesh = mesh;
    }
}

impl Model for NelderMeadEloTracker {
    fn train(&mut self, games: &[(GameCard, GameObject)]) {
        self.compute_mesh(games);

        let mut simplex = Simplex::from(&vec![
            Vector::from(vec![32., 400.]),
            Vector::from(vec![31., 400.]),
            Vector::from(vec![32., 410.]),
        ]);

        let mut cost = |v: &Vector| -> f64 {
            let hash = HashKey::from_vec(v);

            if let Some(&cached) = self.mapping.get(&hash) {
                return cached.cost();
            }

            let performance = if let Ok(params) = EloParams::try_from(v) {
                let mut tracker = EloTracker::with(params);
                tracker.train(games);

                let result = Results::new(tracker.freq(), tracker.log_loss());
                self.mapping.insert(hash, result);

                result.cost()
            } else {
                self.mapping.insert(hash, Results::new(0., INFINITY));

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

            let (v, d) = simplex::delta(&prev, &simplex);
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

#[cfg(test)]
mod test_nme_helpers {
    use super::*;

    #[test]
    fn test_hash_key_from_vec() {
        let expected_vec = Vector::from(vec![1.0, 2.0]);

        let hash = HashKey::from_vec(&expected_vec);

        let actual_vec = hash.to_vec();

        assert_eq!(expected_vec, actual_vec);
    }

    #[test]
    fn test_vec_from_hash() {
        let expected_hash = HashKey::from_vec(&Vector::from(vec![1.0, 2.0]));

        let vec = expected_hash.to_vec();

        let actual_hash = HashKey::from_vec(&vec);

        assert_eq!(expected_hash, actual_hash);
    }
}
