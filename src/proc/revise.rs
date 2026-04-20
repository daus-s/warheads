use crate::dapi::team_box_score::TeamBoxScore;

use crate::edit::edit_list::EditList;
use crate::edit::edit_loader::partition_edit_list;

use crate::stats::identity::{Identifiable, Identity};

use crate::types::SeasonId;

pub fn revise_nba_season(
    era: SeasonId,
    games: &mut Vec<(Identity, TeamBoxScore)>,
    edits: &EditList,
) -> Result<(), ()> {
    let (mut player_edits, mut team_edits) = partition_edit_list(edits);

    player_edits.retain(|c| c.season == era);
    team_edits.retain(|c| c.season == era);

    let mut to_delete: Vec<Identity> = Vec::new();

    for (identity, game) in games.iter_mut() {
        if let Some(edit) = team_edits.iter_mut().find(|c| c.identity() == *identity) {
            game.correct_identifiers(edit);
            game.correct_box_score(edit);
            if edit.delete {
                to_delete.push(identity.clone());
                continue;
            }
        }

        for player in game.roster_mut() {
            let mut player_identity = identity.clone();
            player_identity.player_id = Some(player.player_id());
            if let Some(edit) = player_edits
                .iter_mut()
                .find(|c| c.identity() == player_identity)
            {
                player.correct_identifiers(edit);
                player.correct_box_score(edit);
                if edit.delete {
                    to_delete.push(identity.clone());
                }
            }
        }
    }

    games.retain(|g| !to_delete.contains(&g.0));

    Ok(())
}
