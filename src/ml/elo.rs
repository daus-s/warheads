use crate::dapi::team_box_score::TeamBoxScore;
use crate::ml::cdf;
use crate::types::GameResult::{Draw, Loss, Win};

pub fn update_elo(home: &mut TeamBoxScore, away: &mut TeamBoxScore) {
    assert_ne!(home.result(), away.result(), "💀 both teams have the same result. home: {} away: {}", home.result(), away.result());

    let elo1 = home.elo();

    let elo2 = away.elo();

    let diff = elo1 - elo2;

    let k = 32.;

    let change1 = match home.result() {
        Win => k*(1.- cdf::prob(elo1)),
        Loss => -k*(1.- cdf::prob(elo1)),
        Draw => panic!("💀 this elo algorithm does not support ternary game results. "),
    };

    let change2 = match away.result() {
        Win => k*(1.- cdf::prob(elo2)),
        Loss => -k*(1.- cdf::prob(elo2)),
        Draw => panic!("💀 this elo algorithm does not support ternary game results. "),
    };


}

