use corrections::correction_loader::load_corrections;
use corrections::corrector::Corrector;
use dapi::hunting::{load_nba_season, save_nba_season};
use stats;
use stats::kind::NBAStatKind::Player;

#[tokio::main]
async fn main() {
    println!("hello, {}!", "lisan al-gaib"); //TODO: make this say hi to the user with auth/name

    let corr= load_corrections(1959, Player);

    match corr {
        Ok(cs) => cs.apply().unwrap(),
        Err(e) => eprintln!("{e}"),
    };

    let season = load_nba_season(1959);

    for box_score in season {
        println!("{:?}", box_score);
    }
}