use crate::corrections::correction::Correction;
use crate::corrections::correction_builder::CorrectionBuilder;
use crate::corrections::corrector::Corrector;
use crate::dapi::extract::record_stat;
use crate::dapi::gather::read_nba_file;
use crate::dapi::map_reader::MapReader;
use crate::dapi::parse::*;
use crate::dapi::player_box_score::PlayerBoxScore;
use crate::dapi::team_box_score::TeamBoxScore;
use crate::format::path_manager::nba_data_path;
use crate::format::season::season_fmt;
use crate::stats::box_score::BoxScoreBuilder;
use crate::stats::game_display::GameDisplay;
use crate::stats::id::Identity;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::nba_kind::NBAStatKind::{LineUp, Player, Team};
use crate::stats::nba_stat::NBAStat;
use crate::stats::stat_column::StatColumn;
use crate::stats::stat_column::StatColumn::MATCHUP;
use crate::types::matchup::is_matchup_for_team;
use crate::types::SeasonId;
use serde_json::Value::Null;
use serde_json::{from_str, Value};
use std::collections::HashMap;

pub fn fetch_and_process_nba_games(
    season_id: SeasonId,
    stat: NBAStatKind,
) -> Vec<(Identity, NBAStat)> {
    match process_nba_games(season_id, stat) {
        Ok(games) => games,

        // handle corrections, maybe use something other than `result` in the future
        Err(corrections_meta) => {
            println!(
                "ℹ️ there are {} {} corrections to make for the {}",
                corrections_meta.len(),
                stat,
                season_id
            );

            //check if the corrections exist?

            let corrections: Vec<Correction> = corrections_meta
                .into_iter()
                .map(|mut corr| corr.create())
                .collect();

            let mut dap = HashMap::new();

            let domain = (season_id, stat);

            dap.insert(domain, nba_data_path(season_id, stat));

            corrections
                .apply(&mut dap)
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
) -> Result<Vec<(Identity, NBAStat)>, Vec<CorrectionBuilder>> {
    let file_path = nba_data_path(season_id, stat);

    let json = &read_nba_file(file_path);

    let (rows, headers) = parse_season(from_str(json).expect(&format!(
        "💀 failed to parse a season json object for the {} {} ({})",
        season_fmt(season_id.year()),
        season_id.period(),
        stat
    )));

    season(rows, headers, stat)
}

fn season(
    rows: Vec<Value>,
    headers: Vec<String>,
    stat: NBAStatKind,
) -> Result<Vec<(Identity, NBAStat)>, Vec<CorrectionBuilder>> {
    let mut season: Vec<(Identity, NBAStat)> = Vec::new();
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
                    Ok((id, box_score)) => {
                        season.push((id, NBAStat::Player(box_score)));
                    }
                    Err(correction) => {
                        corrections.push(correction);
                    }
                },
                Team => match fields_to_team_box_score(&fields) {
                    Ok((id, box_score)) => {
                        season.push((id, NBAStat::Team(box_score)));
                    }
                    Err(correction) => {
                        corrections.push(correction);
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
) -> Result<(Identity, TeamBoxScore), CorrectionBuilder> {
    //if it fails to parse the identifier then it will crash

    let mut box_score_builder = BoxScoreBuilder::default();

    let game_id = s.game_id().expect(
        "💀 couldn't get GameId from map which is necessary for (Identity, CorrectionBuilder).",
    );

    let season_id = s.season_id().expect(
        "💀 couldn't get SeasonId from map which is necessary for (Identity, CorrectionBuilder).",
    );

    let team_id = s.team_id().expect(
        "💀 couldn't get TeamId from map which is necessary for (Identity, CorrectionBuilder).",
    );

    let team_abbr = s.team_abbreviation().expect(
        "💀 couldn't get TeamAbbreviation from map which is necessary for (Identity, CorrectionBuilder).",
    );

    let game_date = s
        .game_date()
        .expect("💀 couldn't get GameDate from map, which is necessary for Identity. ");

    let identity = Identity {
        season_id,
        player_id: None,
        team_id,
        team_abbr: team_abbr.clone(),
        game_id,
        game_date,
    };

    let period = season_id.period();

    let mut correction_builder = CorrectionBuilder::new(
        game_id.clone(),
        season_id,
        None,
        team_id,
        team_abbr.clone(),
        Team,
        game_date,
    );

    let matchup = s
        .matchup()
        .expect("💀 couldn't get Matchup from map, which is necessary for GameMetaData. ");

    //check matchup is well-ordered (self team is first listed)
    //this only needs to be checked on team box_scores

    let matchup_string = s.get(&MATCHUP).expect("💀 couldn't get Matchup from map, which is necessary for checking Matchup-TeamAbbreviation validity. ").as_str().expect("💀 Matchup in HashMap is not of type JSON::String");

    if !is_matchup_for_team(matchup_string, &team_abbr) {
        correction_builder.add_missing_field(MATCHUP, Null);
    }

    let visiting = matchup
        .home_or_away(&team_abbr)
        .expect("💀 expected TeamBoxScore's matchup to contain itself. ");

    let team_name = s
        .team_name()
        .expect("💀 couldn't get TeamName from map, which is necessary for GameMetaData. ");

    let meta = GameDisplay::new(
        matchup.clone(),
        game_date.clone(),
        None,
        team_name.clone(),
    );

    correction_builder.update_display(meta);

    record_stat(
        s.game_result(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(s.minutes(), &mut box_score_builder, &mut correction_builder);
    record_stat(
        s.field_goal_makes(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.field_goal_attempts(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.three_point_makes(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.three_point_attempts(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.free_throw_makes(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.free_throw_attempts(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.offensive_rebounds(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.defensive_rebounds(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.rebounds(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(s.assists(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.steals(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.blocks(), &mut box_score_builder, &mut correction_builder);
    record_stat(
        s.turnovers(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.personal_fouls(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(s.points(), &mut box_score_builder, &mut correction_builder);
    record_stat(
        s.plus_minus(),
        &mut box_score_builder,
        &mut correction_builder,
    );

    if correction_builder.correcting() {
        eprintln!("\n❌ failed to create a TeamBoxScore for {team_name}. id: {team_id} game id: {game_id}");

        Err(correction_builder)
    } else {
        let box_score = box_score_builder.build()
            .map_err(|e| format!("{e}"))
            .unwrap_or_else(|e| panic!("💀 failed to create TeamBoxScore: {e}\n💀 GameId: {game_id}\n💀 SeasonId: {season_id}\n💀 TeamId: {team_id}\n💀 TeamAbbreviation: {team_abbr}"));

        println!("✅ successfully created TeamBoxScore for {team_name}. id: {team_id} game id: {game_id}");

        let team = TeamBoxScore::construct(team_abbr, team_name, team_id, visiting, box_score);

        Ok((identity, team))
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
) -> Result<(Identity, PlayerBoxScore), CorrectionBuilder> {
    //if it fails to parse the identifier then it will crash

    let mut box_score_builder = BoxScoreBuilder::default();

    let game_id = s
        .game_id()
        .expect("💀 couldn't get GameId from map which is necessary for CorrectionBuilder.");
    let player_id = s.player_id().expect("💀 couldn't get PlayerId from map which is necessary for CorrectionBuilder. (variant: Player)");
    let season_id = s
        .season_id()
        .expect("💀 couldn't get SeasonId from map which is necessary for CorrectionBuilder.");
    let team_id = s
        .team_id()
        .expect("💀 couldn't get TeamId from map which is necessary for CorrectionBuilder.");
    let team_abbr = s.team_abbreviation().expect(
        "💀 couldn't get TeamAbbreviation from map which is necessary for CorrectionBuilder.",
    );
    let game_date = s
        .game_date()
        .expect("💀 couldn't get GameDate from map, which is necessary for Identity. ");

    let identity = Identity {
        season_id,
        player_id: Some(player_id),
        team_id,
        team_abbr: team_abbr.clone(),
        game_date,
        game_id,
    };

    let period = season_id.period();

    let mut correction_builder = CorrectionBuilder::new(
        game_id.clone(),
        season_id,
        Some(player_id),
        team_id,
        team_abbr.clone(),
        Player,
        game_date,
    );

    let matchup = s
        .matchup()
        .expect("💀 couldn't get Matchup from map, which is necessary for GameMetaData. ");

    let player_name = s.player_name().expect("💀 couldn't get PlayerName from map, which is necessary for GameMetaData. (variant: Player)");
    let team_name = s
        .team_name()
        .expect("💀 couldn't get TeamName from map, which is necessary for GameMetaData. ");

    let meta = GameDisplay::new(
        matchup.clone(),
        game_date.clone(),
        Some(player_name.clone()),
        team_name.clone(),
    );

    correction_builder.update_display(meta);

    record_stat(
        s.game_result(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(s.minutes(), &mut box_score_builder, &mut correction_builder);
    record_stat(
        s.field_goal_makes(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.field_goal_attempts(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.three_point_makes(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.three_point_attempts(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.free_throw_makes(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.free_throw_attempts(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.offensive_rebounds(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.defensive_rebounds(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.rebounds(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(s.assists(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.steals(), &mut box_score_builder, &mut correction_builder);
    record_stat(s.blocks(), &mut box_score_builder, &mut correction_builder);
    record_stat(
        s.turnovers(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(
        s.personal_fouls(),
        &mut box_score_builder,
        &mut correction_builder,
    );
    record_stat(s.points(), &mut box_score_builder, &mut correction_builder);
    record_stat(
        s.plus_minus(),
        &mut box_score_builder,
        &mut correction_builder,
    );

    if correction_builder.correcting() {
        eprintln!("\n❌ failed to create a PlayerBoxScore for {player_name}. id: {player_id} game id: {game_id}");

        Err(correction_builder)
    } else {
        let box_score = box_score_builder.build()
            .map_err(|e| format!("{e}"))
            .unwrap_or_else(|e| panic!("💀 failed to create PlayerBoxScore: {e}\n💀 GameId: {game_id}\n💀 PlayerId: {player_id}\n💀 SeasonId: {season_id}\n💀 TeamId: {team_id}\n💀 TeamAbbreviation: {team_abbr}"));

        let msg = format!("✅ successfully created PlayerBoxScore for {player_name}. id: {player_id} game id: {game_id}");

        let player = PlayerBoxScore::construct(player_id, player_name, box_score);

        println!("{msg}");

        Ok((identity, player))
    }
}
