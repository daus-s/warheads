use std::fs::File;
use std::io::Read;

use serde_json::Value;

use crate::dapi::player_box_score::PlayerBoxScore;
use crate::dapi::team_box_score::TeamBoxScore;

use crate::edit::edit_list::EditList;

use crate::edit::edit_loader::{load_edit_list, save_edit_list};
use crate::format::parse::parse_season;
use crate::format::path_manager::nba_source_path;

use crate::proc::error::ReadProcessError;
use crate::proc::rip;
use crate::proc::rip::ProcessingResult;

use crate::stats::identity::Identity;
use crate::stats::nba_boxscore::NBABoxScore::{self, Player, Team};
use crate::stats::nba_kind::NBAStatKind;

use crate::types::SeasonId;

pub fn load_season_from_source(
    era: SeasonId,
) -> Result<Vec<(Identity, TeamBoxScore)>, ReadProcessError> {
    let mut edit_list: EditList = load_edit_list().unwrap_or_default();

    let mut team_games_vec = Vec::new();

    let player_games_of_period = load_player_games_from_source(era, &mut edit_list)?;

    let team_games_of_period =
        load_team_games_from_source(era, player_games_of_period, &mut edit_list)?;

    save_edit_list(&edit_list).map_err(|_| ReadProcessError::SerializeEditError)?;

    team_games_vec.extend(team_games_of_period);

    Ok(team_games_vec)
}

pub fn load_player_games_from_source(
    season_id: SeasonId,
    edit_list: &mut EditList,
) -> Result<Vec<(Identity, PlayerBoxScore)>, ReadProcessError> {
    let player_source_path = nba_source_path(season_id, NBAStatKind::Player);

    let mut file = File::open(player_source_path).map_err(|e| ReadProcessError::IOError(e))?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|e| ReadProcessError::IOError(e))?;

    let json = serde_json::from_str(&contents).map_err(|e| ReadProcessError::JSONParseError(e))?;

    let (rows, headers) =
        parse_season(json).map_err(|e| ReadProcessError::ObjectStructureError(e))?;

    Ok(generate_nba_games_from_source(headers, rows, edit_list)?
        .into_iter()
        .filter_map(|(id, stat)| match stat {
            Player(box_score) => Some((id, box_score)),
            _ => None,
        })
        .collect::<Vec<(Identity, PlayerBoxScore)>>())
}

pub fn load_team_games_from_source(
    season_id: SeasonId,
    player_games: Vec<(Identity, PlayerBoxScore)>,
    edit_list: &mut EditList,
) -> Result<Vec<(Identity, TeamBoxScore)>, ReadProcessError> {
    let team_source_path = nba_source_path(season_id, NBAStatKind::Team);

    let mut file = File::open(team_source_path).map_err(|e| ReadProcessError::IOError(e))?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|e| ReadProcessError::IOError(e))?;

    let json = serde_json::from_str(&contents).map_err(|e| ReadProcessError::JSONParseError(e))?;

    let (rows, headers) =
        parse_season(json).map_err(|e| ReadProcessError::ObjectStructureError(e))?;

    let mut games: Vec<(Identity, TeamBoxScore)> =
        generate_nba_games_from_source(headers, rows, edit_list)?
            .into_iter()
            .filter_map(|(id, stat)| match stat {
                Team(t) => Some((id, t)),
                _ => None,
            })
            .collect();

    for (p_id, player_box_score) in player_games.into_iter() {
        for (t_id, team_box_score) in games.iter_mut() {
            if p_id.game() == t_id.game() {
                team_box_score.add_player_stats(player_box_score.clone());
            }
        }
    }

    Ok(games)
}

fn generate_nba_games_from_source(
    headers: Vec<String>,
    rows: Vec<Value>,
    edits: &mut EditList,
) -> Result<Vec<(Identity, NBABoxScore)>, ReadProcessError> {
    let mut games = Vec::new();

    let mut results = rip::season(rows.clone(), headers.clone(), edits)?;

    let mut incompletions = 0;

    for result in results.iter_mut() {
        match result {
            ProcessingResult::Record(identity, boxscore) => {
                games.push((identity.to_owned(), boxscore.to_owned()))
            },
            ProcessingResult::Edit(edit_builder) => {
                if edit_builder.date().is_today() {
                    println!("⏳ game is live. omitting stats.")
                } else {
                    edit_builder.prompt(); //starts the tui prompter

                    let edit = edit_builder.build().ok_or(ReadProcessError::BuildEditError)?;

                    edits.insert(edit);

                    incompletions += 1;
                }
            },
            ProcessingResult::Delete(ident) => match ident.team_or_player() {
                NBAStatKind::Team => println!(
                    "🗑️ deleting team record for {} game: {}. all associated player records will be ignored",
                    ident.team_abbr(),
                    ident.game_id
                ),
                NBAStatKind::Player => println!(
                    "🗑️ deleting player record for id: {} game: {}. the respective game object will not be affected though stat totals may not be consistent.",
                    ident.player_id.unwrap(),
                    ident.game_id
                ),
                NBAStatKind::LineUp => unimplemented!(),
            },
        }
    }

    if incompletions > 0 {
        generate_nba_games_from_source(headers, rows, edits)
    } else {
        Ok(games)
    }
}
