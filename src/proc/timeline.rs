use crate::dapi::season_manager::nba_lifespan_period;

use crate::format::path_manager::nba_timeline_file;

use crate::stats::gamecard::GameCard;
use crate::stats::visiting::Visiting;

use crate::storage::read_disk::read_nba_season;
use crate::storage::write::write_serializable_with_directory;

use crate::types::GameDate;

use std::collections::HashMap;

/// sequence_nba creates a timeline of all games in NBA history based on already saved games
pub fn sequence_nba() -> Result<(), Box<dyn std::error::Error>> {
    // create chronological timeline of all game events
    for era in nba_lifespan_period() {
        let mut dates = HashMap::<GameDate, Vec<GameCard>>::new();

        let mut games = read_nba_season(era)?;

        games.sort_by_key(|game| game.game_id);

        for game in games.iter() {
            let date = game.game_date;
            dates.entry(date).or_insert_with(Vec::new).push(game.card());
        }

        for game in games {
            let date = game.game_date;

            let winner = game.winning_side();

            let cards = dates.get_mut(&date).unwrap(); //it must be in here if we added it. i think u can prove this wont unwrap

            let gc = cards
                .iter_mut()
                .find(|card| card.game_id() == game.game_id())
                .unwrap(); //same protective level for this unwrap as before

            match winner {
                Visiting::Home => {
                    gc.mut_home().add_win();
                    gc.mut_away().add_loss();
                }
                Visiting::Away => {
                    gc.mut_away().add_win();
                    gc.mut_home().add_loss();
                }
            }
        }

        for (date, cards) in dates.iter() {
            let path = nba_timeline_file(era, *date);

            write_serializable_with_directory(&path, &cards)?;
        }
    }

    Ok(())

    // todo: add checksums for each era.
    // chronologica
}
