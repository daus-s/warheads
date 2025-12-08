/// this elo algorithm is optimized on (k, f) pairs,
/// the algorithm also uses a initial rating of 0 for symmetry.
pub struct OptimizedElo {}

impl OptimizedElo {
    pub fn train() -> Self {
        for k in 1..=100 {
            for f in (10..=1000).step_by(10) {
                // Train the model with (k, f) pair
            }
        }
        Self {}
    }
}
