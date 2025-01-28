use dapi::rip::{process_nba_games};
use dapi::gather::read_nba;

use stats::kind::NBAStatKind::*;
use stats::kind::NBAStatType;

fn main() {
    println!("hello, lisan al-gaib!"); //TODO: make this say hi to the user with auth/name

    let player_games = process_nba_games((&read_nba(2023, Player)), Player).unwrap();


    for game in player_games {
        match game {
            NBAStatType::Player(pg) => println!("{}", pg),
            NBAStatType::Team(_tg) => panic!("fuck you"),
        }
    }


    let team_games = process_nba_games(&read_nba(2023, Team), Team).unwrap();


    for game in team_games {
        match game {
            NBAStatType::Player(_pg) =>panic!("fuck you") ,
            NBAStatType::Team(tg) => println!("{}", tg),
        }
    }
}
