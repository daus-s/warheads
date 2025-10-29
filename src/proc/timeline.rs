use crate::dapi::season_manager::nba_lifespan_period;

use crate::stats::gamecard::GameCard;

use crate::storage::read_disk::read_nba_season;

use crate::types::GameDate;

use std::collections::HashMap;

pub fn sequence_nba() -> Result<(), ()> {
    // create chronological timeline of all game events
    for szn in nba_lifespan_period() {
        let dates = HashMap::<GameDate, Vec<GameCard>>::new();

        let games = read_nba_season(szn).map_err(|_| ())?;
    }

    Ok(())

    // todo: add checksums for each era.
    // chronologica
}
