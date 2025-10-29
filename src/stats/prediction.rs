use crate::stats::gamecard::GameCard;

#[derive(Debug, Clone, PartialEq)]
pub struct Prediction {
    card: GameCard,

    /// ### Probability
    /// probability a `f64`, represents the probability that the home team wins as per scoring
    /// home win = 1
    /// away win = 0
    probability: f64,
}
