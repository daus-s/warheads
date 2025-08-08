use warheads::dapi::hunting::{chronicle_nba, observe_nba, query_nba, revise_nba};
use warheads::stats::nba_kind::NBAStatKind::{Player, Team};
use warheads::stats::season_period::SeasonPeriod::{PlayIn, PostSeason, PreSeason, RegularSeason};
use warheads::types::SeasonId;

#[tokio::main]
async fn main() {
    println!("hello, {}!", "lisan al-gaib"); //TODO: make this say hi to the user with auth/name

    // observe_nba().await;

    // revise_nba();

    chronicle_nba().await;
}
