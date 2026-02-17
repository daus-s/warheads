use std::fs;

use crate::edit::edit_loader::{load_edit_list, split_edit_list};

use crate::dapi::team_box_score::TeamBoxScore;

use crate::format::path_manager::nba_storage_file;

use crate::stats::identity::{Identifiable, Identity};

use crate::types::SeasonId;

pub fn revise_nba_season(
    era: SeasonId,
    games: &mut Vec<(Identity, TeamBoxScore)>,
) -> Result<(), ()> {
    let (mut player_corrections, mut team_corrections) =
        split_edit_list(load_edit_list().map_err(|_| ())?);

    //only delete if the team correction says the game shouldnt be recorded
    for correction in team_corrections.iter() {
        if correction.delete {
            let id = correction.identity();

            let file = nba_storage_file(id.season_id, id.game_id);

            if file.exists() {
                if let Err(e) = fs::remove_file(&file) {
                    eprintln!(
                        "{}\n❗ failed to remove record on disk in the file {}",
                        e,
                        file.display()
                    );
                }
            }
        }
    }

    for (identity, game) in games.iter_mut() {
        if let Some(correction) = team_corrections
            .iter_mut()
            .find(|c| c.identity() == *identity)
        {
            game.reorient(correction);
            game.correct_box_score(correction);
        }

        for player in game.roster_mut() {
            let mut player_identity = identity.clone();
            player_identity.player_id = Some(player.player_id());
            if let Some(correction) = player_corrections
                .iter_mut()
                .find(|c| c.identity() == player_identity)
            {
                player.reorient(correction);
                player.correct_box_score(correction);
            }
        }
    }

    Ok(())
}
