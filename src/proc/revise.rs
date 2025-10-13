use crate::corrections::correction::Correction;
use crate::corrections::correction_loader::load_season_corrections;

use crate::dapi::team_box_score::TeamBoxScore;

use crate::format::season::season_fmt;

use crate::stats::id::{Identifiable, Identity};
use crate::stats::nba_kind::NBAStatKind;

use std::collections::HashMap;

pub fn revise_nba_season(year: i32, games: &mut Vec<(Identity, TeamBoxScore)>) -> Result<(), ()> {
    let mut player_corrections = load_season_corrections(year, NBAStatKind::Player)
        .map_err(|msg| {
            eprintln!(
                "{msg}\n❌ failed to load player corrections for the {} season:",
                season_fmt(year)
            );
            ()
        })?
        .into_iter()
        .map(|correction| (correction.identity(), correction))
        .collect::<HashMap<Identity, Correction>>();

    let mut team_corrections = load_season_corrections(year, NBAStatKind::Team)
        .map_err(|msg| {
            eprintln!(
                "{msg}\n❌ failed to load team corrections for the {} season:",
                season_fmt(year)
            );
            ()
        })?
        .into_iter()
        .map(|correction| (correction.identity(), correction))
        .collect::<HashMap<Identity, Correction>>();

    for (identity, game) in games.iter_mut() {
        if let Some(correction) = team_corrections.get_mut(&identity) {
            game.correct_box_score(correction);
        }

        //apply player corrections
        for player in game.roster_mut() {
            let mut player_identity = identity.clone();

            player_identity.player_id = Some(player.player_id());

            if let Some(correction) = player_corrections.get_mut(&player_identity) {
                player.correct_box_score(correction);
            }
        }
    }

    Ok(())
}
