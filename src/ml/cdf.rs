use num::pow::Pow;

pub(crate) fn prob(diff: i32) -> f64 {
    let exponent = -1. * diff as f64 / 400.0;
    /* FIDE uses 400 for the scale factor and chess players have a similar number of games to nba players
     * each year, so this factor seems appropriate
     *
     * source: https://www.chess.com/article/view/games-per-year
     */

    1. / (1. + 10f64.pow(exponent))
}

#[test]
fn test_diff_0() {
    let diff = 0; //0 std devs

    let expected = prob(diff);

    assert_eq!(expected, 0.5) //expect 50-50
}

#[test]
fn test_diff_1() {
    let diff = 400; //1 std devs

    let expected = prob(diff);

    assert_eq!(expected, 0.9090909090909091)
}

#[test]
fn test_specific_diff() {
    let diff = 200; // https://en.wikipedia.org/wiki/Elo_rating_system#Mathematical_details::text=Performance%20is%20not,of%20approximately%200.75.

    let expected = prob(diff);

    assert_eq!(expected, 0.7597469266479578)
}

#[test]
fn test_symmetry() {
    for diff in 0..200 {
        let win = prob(diff);

        let loss = prob(-1 * diff);

        let tol = 1. - (win + loss) <= 0.00000000000005;

        assert_eq!(tol, true) //check if the error is within the tolerance range
    }
}
