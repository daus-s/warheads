use crate::corrections::correction::Correction;
use crate::corrections::correction_builder::CorrectionBuilder;
use crate::corrections::corrector::Corrector;
use crate::dapi::extract::{record_stat, record_usable_stat};
use crate::dapi::gather::read_nba_file;
use crate::dapi::map_reader::MapReader;
use crate::dapi::parse::*;
use crate::format::path_manager::nba_data_path;
use crate::stats::game_metadata::GameMetaData;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::nba_kind::NBAStatKind::{LineUp, Player, Team};
use crate::stats::nba_stat::NBAStat;
use crate::stats::player_box_score::{PlayerBoxScore, PlayerBoxScoreBuilder};
use crate::stats::stat_column::StatColumn;
use crate::stats::stat_column::StatColumn::*;
use crate::stats::team_box_score::{TeamBoxScore, TeamBoxScoreBuilder};
use crate::types::*;
use serde_json::{from_str, Value};
use std::collections::HashMap;

pub fn fetch_and_process_nba_games(
    season_id: SeasonId,
    stat: NBAStatKind,
) -> Vec<NBAStat> {
    match process_nba_games(season_id, stat) {
        Ok(games) => games,

        // handle corrections, maybe use something other than `result` in the future
        Err(corrections_meta) => {
            let corrections: Vec<Correction> = corrections_meta
                .into_iter()
                .map(|mut corr| corr.create())
                .collect();

            corrections.iter().for_each(|c| {
                let _ = c.save(); // we assume t that saving to the file is allowed and disregard
                                  // the result. todo: add more robust error handling here.
            });

            corrections
                .apply()
                .map(|_| fetch_and_process_nba_games(season_id, stat))
                .unwrap_or_else(|e| panic!("💀 failed to apply corrections: {}", e))
        }
    }
}

///
/// rips through the json using the header provided as per NBA apis convention/schema.
/// return a Result of Ok(Vec<NBAStat>) or Err(Vec<Correction>). it is important to remember
/// an NBAStat is a BoxScore.
///
/// process games will crash if the JSON is poorly shaped.
///

fn process_nba_games(
    season_id: SeasonId,
    stat: NBAStatKind,
) -> Result<Vec<NBAStat>, Vec<CorrectionBuilder>> {
    let file_path = nba_data_path(season_id, stat);

    let json = &read_nba_file(file_path);

    let (rows, headers) = parse_season(from_str(json).unwrap());

    season(rows, headers, stat)
}

fn season(
    rows: Vec<Value>,
    headers: Vec<String>,
    stat: NBAStatKind,
) -> Result<Vec<NBAStat>, Vec<CorrectionBuilder>> {
    let mut season: Vec<NBAStat> = Vec::new();
    let mut corrections: Vec<CorrectionBuilder> = Vec::new();

    for row in rows {
        if let Some(row_data) = row.as_array() {
            let fields: HashMap<StatColumn, Value> = headers
                .iter()
                .zip(row_data.iter())
                .map(|(name, value)| (StatColumn::from(name.to_owned()), value.clone()))
                .collect();

            match stat {
                Player => match fields_to_player_box_score(&fields) {
                    Ok(box_score) => {
                        season.push(NBAStat::Player(box_score));
                    }
                    Err(e) => {
                        corrections.push(e);
                    }
                },
                Team => match fields_to_team_box_score(&fields) {
                    Ok(box_score) => {
                        season.push(NBAStat::Team(box_score));
                    }
                    Err(e) => {
                        corrections.push(e);
                    }
                },
                LineUp => unimplemented!("lineup stats are not yet supported."),
            };
        }
    }

    if corrections.len() == 0 {
        Ok(season)
    } else {
        Err(corrections)
    }
}

fn fields_to_team_box_score(
    s: &HashMap<StatColumn, Value>,
) -> Result<TeamBoxScore, CorrectionBuilder> {

    //if it fails to parse the identifier then it will crash
    //if it fails to parse the identifier then it will crash
    let game_id = s.game_id().expect("💀 couldn't get GameId from map which is necessary for CorrectionBuilder.");
    let season_id = s.season_id().expect("💀 couldn't get SeasonId from map which is necessary for CorrectionBuilder.");
    let team_id = s.team_id().expect("💀 couldn't get TeamId from map which is necessary for CorrectionBuilder.");
    let team_abbr = s.team_abbreviation().expect("💀 couldn't get TeamAbbreviation from map which is necessary for CorrectionBuilder.");

    let period = season_id.period();


    let mut correction_builder = CorrectionBuilder::new(
        game_id.clone(),
        season_id,
        None,
        team_id,
        team_abbr.clone(),
        Team,
        period,
    );

    let mut box_score_builder = TeamBoxScoreBuilder::default();

    let matchup = record_usable_stat(s.matchup(), &mut box_score_builder, &mut correction_builder).expect("💀 couldn't get Matchup from map, which is necessary for GameMetaData. ");
    let game_date = record_usable_stat(s.game_date(), &mut box_score_builder, &mut correction_builder).expect("💀 couldn't get GameDate from map, which is necessary for GameMetaData. ");
    let team_name = record_usable_stat(s.team_name(), &mut box_score_builder, &mut correction_builder).expect("💀 couldn't get TeamName from map, which is necessary for GameMetaData. ");


    //set metadata for correction before correcting stats (for display purposes mainly)
    let meta = GameMetaData::new(
        matchup.clone(),
        game_date.clone(),
        None,
        team_abbr.clone(),
        team_name.clone()
    );

    correction_builder.update_meta(meta);

    // collect stat fixes
    // todo: macro idea all these stats act exactly the same and i didnt want to have to write the
    // code slightly different for each one how would that work is this derive stuff
    record_stat(s.game_result(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.minutes(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.field_goal_makes(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.field_goal_attempts(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.three_point_makes(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.three_point_attempts(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.free_throw_makes(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.free_throw_attempts(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.offensive_rebounds(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.defensive_rebounds(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.rebounds(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.assists(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.steals(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.blocks(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.turnovers(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.personal_fouls(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.points(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.plus_minus(), &mut box_score_builder, &mut correction_builder);

    let box_score = box_score_builder.build().unwrap();

    if correction_builder.correcting() {
        eprintln!("❌ failed to create a TeamBoxScore for {team_name}. id: {team_id} game id: {game_id}");

        Err(correction_builder)
    } else {
        Ok(box_score)
    }
}


///
/// fields_to_player_box_score returns a result of either a player box score or a correction.
/// if the function returns a correction, the correction acts as a form that needs to be
/// completed before that entry can be finalized. as such, seemingly inconsequentially,
/// the player stats must always be ripped from file before team results.
///
fn fields_to_player_box_score(
    s: &HashMap<StatColumn, Value>,
) -> Result<PlayerBoxScore, CorrectionBuilder> {

    //if it fails to parse the identifier then it will crash
    let game_id = s.game_id().expect("💀 couldn't get GameId from map which is necessary for CorrectionBuilder.");
    let player_id = s.player_id().expect("💀 couldn't get PlayerId from map which is necessary for CorrectionBuilder. (variant: Player)");
    let season_id = s.season_id().expect("💀 couldn't get SeasonId from map which is necessary for CorrectionBuilder.");
    let team_id = s.team_id().expect("💀 couldn't get TeamId from map which is necessary for CorrectionBuilder.");
    let team_abbr = s.team_abbreviation().expect("💀 couldn't get TeamAbbreviation from map which is necessary for CorrectionBuilder.");

    let period = season_id.period();

    let mut correction_builder = CorrectionBuilder::new(
        game_id.clone(),
        season_id,
        Some(player_id),
        team_id,
        team_abbr.clone(),
        Player,
        period
    );

    let mut box_score_builder = PlayerBoxScoreBuilder::default();

    let matchup = record_usable_stat(s.matchup(), &mut box_score_builder, &mut correction_builder).expect("💀 couldn't get Matchup from map, which is necessary for GameMetaData. ");
    let game_date = record_usable_stat(s.game_date(), &mut box_score_builder, &mut correction_builder).expect("💀 couldn't get GameDate from map, which is necessary for GameMetaData. ");
    let player_name = record_usable_stat(s.player_name(), &mut box_score_builder, &mut correction_builder).expect("💀 couldn't get PlayerName from map, which is necessary for GameMetaData. (variant: Player)");
    let team_name = record_usable_stat(s.team_name(), &mut box_score_builder, &mut correction_builder).expect("💀 couldn't get TeamName from map, which is necessary for GameMetaData. ");

    let meta = GameMetaData::new(
        matchup.clone(),
        game_date.clone(),
        Some(player_name.clone()),
        team_abbr.clone(),
        team_name.clone()
    );

    correction_builder.update_meta(meta);

    record_stat(s.game_result(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.minutes(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.field_goal_makes(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.field_goal_attempts(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.three_point_makes(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.three_point_attempts(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.free_throw_makes(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.free_throw_attempts(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.offensive_rebounds(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.defensive_rebounds(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.rebounds(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.assists(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.steals(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.blocks(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.turnovers(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.personal_fouls(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.points(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.plus_minus(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.fantasy_points(), &mut box_score_builder, &mut correction_builder);


    let box_score = box_score_builder.build().unwrap();

    if correction_builder.correcting() {
        eprintln!("❌ failed to create a PlayerBoxScore for {player_name}. id: {player_id} game id: {game_id}");

        Err(correction_builder)
    } else {
        Ok(box_score)
    }
}
