use crate::dapi::team_box_score::TeamBoxScore;

use crate::edit::edit_builder::EditBuilder;
use crate::edit::edit_loader::{load_edit_list, save_edit_list, EditLoadingError};

use crate::proc::error::ReadProcessError;
use crate::proc::hunting::load_season_from_source;
use crate::proc::revise::revise_nba_season;

use crate::stats::game_obj::GameObject;
use crate::stats::identity::Identity;

use crate::dapi::store_disk::{save_nba_games, SaveGameError};
use crate::stats::stat_column::StatColumn;

use crate::types::{GameId, SeasonId};

use thiserror::Error;

use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum InscriptionError {
    #[error("{0}\n❌ edit file failed serialization. ")]
    LoadEditListError(EditLoadingError),
    #[error("{0}\n❌ Save Game Error")]
    FileError(ReadProcessError),
    #[error("❌ failed to revise NBA games.")]
    RevisionError,
    #[error("{0}\n❌ failed to save NBA games.")]
    SerializeGameError(SaveGameError),
    #[error("❌ failed to save edit list.")]
    SaveEditListError,
    #[error("❌ failed to construct an edit, some fields were missing.")]
    BuildEditError,
}

pub fn inscribe(era: SeasonId) -> Result<(), InscriptionError> {
    let mut edits = load_edit_list().map_err(|e| InscriptionError::LoadEditListError(e))?;

    let mut team_games =
        load_season_from_source(era).map_err(|e| InscriptionError::FileError(e))?;

    revise_nba_season(era, &mut team_games, &edits).map_err(|_| InscriptionError::RevisionError)?;

    let pairs = pair_off(team_games);

    match pairs {
        Err(mut edit_builders) => {
            println!(
                "ℹ️  there are {} corrections to make to Team box scores for the {} season.",
                edit_builders.len(),
                era
            );

            for edit_builder in edit_builders.iter_mut() {
                let game_id = edit_builder.game_id();

                // look in already-loaded edits for the sibling with same game_id, different team
                if let Some(resolved) = edits.find_sibling(game_id, edit_builder.team_abbr()) {
                    if resolved.corrects(&StatColumn::MATCHUP) {
                        let matchup = resolved.inverse_matchup_as_value().unwrap();

                        edit_builder.add_missing_field(StatColumn::MATCHUP, matchup);

                        if let Some(edit) = edit_builder.build() {
                            edits.insert(edit);
                        }
                    }
                } else {
                    edit_builder.prompt(); //

                    if let Some(edit) = edit_builder.build() {
                        edits.insert(edit);
                    } else {
                        return Err(InscriptionError::BuildEditError);
                    }
                }
                // otherwise: write it out, wait for manual input next run
            }

            save_edit_list(&edits).map_err(|_| InscriptionError::SaveEditListError)?;

            inscribe(era)
        }
        Ok(games) => save_game_object(games).map_err(|e| InscriptionError::SerializeGameError(e)),
    }
}

fn save_game_object(season: Vec<GameObject>) -> Result<(), SaveGameError> {
    let num_games = season.len();

    if num_games == 0 {
        return Ok(());
    }

    save_nba_games(&season)?;

    Ok(())
}

pub(crate) type TeamGame = (Identity, TeamBoxScore);

pub(crate) fn pair_off(games: Vec<TeamGame>) -> Result<Vec<GameObject>, Vec<EditBuilder>> {
    let mut pairs = HashMap::<GameId, (Option<TeamGame>, Option<TeamGame>)>::new();
    let mut corrections: Vec<EditBuilder> = Vec::new();

    let l = games.len();

    for (id, game) in games.into_iter() {
        match pairs.get_mut(&id.game_id) {
            Some((game1, game2)) => match game1 {
                Some(_game1) => {
                    *game2 = Some((id, game));
                }
                None => {
                    *game1 = Some((id, game));
                }
            },
            None => {
                pairs.insert(id.game_id, (Some((id, game)), None));
            }
        }
    }

    //find and report unpaired unboxed games
    for (id, pair) in pairs.iter() {
        match pair {
            (Some(game), None) | (None, Some(game)) => {
                println!(
                    "⚠️ unpaired game: {} season: {}",
                    id,
                    pair.0.as_ref().unwrap().0.season_id
                );

                let Identity {
                    season_id,
                    player_id,
                    team_id,
                    team_abbr,
                    game_id,
                    game_date,
                } = game.0.clone();

                let mut correction_builder =
                    EditBuilder::new(game_id, season_id, player_id, team_id, team_abbr, game_date);

                correction_builder.set_delete(true);

                corrections.push(correction_builder);
            }
            _ => {
                //do nothing
            }
        }
    }

    let mut games: Vec<GameObject> = Vec::with_capacity(l / 2);

    for (_game_id, (a, b)) in pairs.into_iter() {
        match (a, b) {
            (Some((id1, game1)), Some((id2, game2))) => {
                match GameObject::try_create(id1, game1, id2, game2) {
                    Ok(game_object) => {
                        games.push(game_object);
                    }
                    Err(mut corrections_builders) => {
                        corrections.append(&mut corrections_builders);
                    }
                }
            }
            _ => {}
        }
    }

    if corrections.len() > 0 {
        Err(corrections)
    } else {
        Ok(games)
    }
}
