use crate::dapi::season_manager::nba_lifespan;
use crate::proc::store::save_nba_season;

/// this module contains functions for writing the history of the nba stats
/// you can build around this function but not from it... this is the one function to start the nba into memory then iterate over elo.
pub async fn chronicle_nba() {
    for szn in nba_lifespan() {
        save_nba_season(szn).await;
    }
}
