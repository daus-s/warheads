use crate::dapi::team_box_score::TeamBoxScore;
use num::pow;
use crate::types::GameResult;
use crate::types::GameResult::{Win, Loss, Draw};

pub fn update_elo(home: &mut TeamBoxScore, away: &mut TeamBoxScore) {
    assert_ne!(home.result(), away.result()); // this assert statement makes sure that there is not a tie

    let elo1 = home.elo();

    let elo2 = away.elo();

    let diff = elo1 - elo2;

    let k = 32.;

    let change1 = match home.result() {
        Win => k*(1.-prob(elo1 as usize)),
        Loss => -k*(1.-prob(elo1 as usize)),
        GameResult::Draw => panic!("💀 this elo algorithm does not support ternary game results. "),
    };

    let change2 = match away.result() {
        Win => k*(1.-prob(elo2 as usize)),
        Loss => -k*(1.-prob(elo2 as usize)),
        GameResult::Draw => panic!("💀 this elo algorithm does not support ternary game results. "),
    };
}

pub fn process_elo() {
    todo!("assign elo values to players on a game by game basis")
}

fn prob(x: usize) -> f32 {
    1./(1. + pow(10, x)  as f32)
}