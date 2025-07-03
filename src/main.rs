use warheads::corrections::correction_loader::load_corrections;
use warheads::dapi::hunting::{chronicle_nba, load_nba_season_from_file, observe_nba};
use warheads::stats::nba_kind::NBAStatKind::Team;
use warheads::types::SeasonId;

#[tokio::main]
async fn main() {
    println!("hello, {}!", "lisan al-gaib"); //TODO: make this say hi to the user with auth/name

    // observe_nba().await;

    // check if corrections exist and try to apply them

    // chronicle_nba().await;

    let year = 1947;

    let season = load_nba_season_from_file(year);

    let corrections = load_corrections(year, Team)
        .map_err(|err_str| eprintln!("{}", err_str))
        .unwrap();

    for c in corrections {
        println!("{:?}", c);
    }
}
