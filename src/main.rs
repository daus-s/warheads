use corrections::correction_loader::load_corrections;
use dapi::hunting::save_nba_season;
use stats;
use stats::kind::NBAStatKind::Player;

#[tokio::main]
async fn main() {
    println!("hello, {}!", "lisan al-gaib"); //TODO: make this say hi to the user with auth/name


    let corr= load_corrections(1959, Player);

    match corr {
        Ok(cs) => cs.iter().for_each(|c| println!("{:?}", c)),
        Err(e) => eprintln!("{e}"),
    }
}