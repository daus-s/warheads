use crate::dapi::hunting;
use crate::dapi::rip::fetch_and_process_nba_games;
use crate::format::path_manager::nba_data_path;
use crate::format::season::season_fmt;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::nba_stat::NBAStat::{Player, Team};
use crate::stats::player_box_score::PlayerBoxScore;
use crate::stats::season_period::minimum_spanning_era;
use crate::stats::team_box_score::TeamBoxScore;
use crate::types::SeasonId;
use reqwest;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::{fs, io};

pub fn read_nba_file(file_path: PathBuf) -> String {
    let mut file =
        File::open(&file_path).expect(format!("Failed to open {}", file_path.display()).as_str());

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect(format!("Failed to read {}", file_path.display()).as_str());

    contents
}

pub fn write_games(file_path: PathBuf, raw_json: &str) -> io::Result<()> {
    if let Some(parent) = file_path.parent() {
        //this creates the directory from the ground up.
        fs::create_dir_all(parent)?;
    }

    fs::write(file_path, raw_json)
}

pub fn player_games(year: i32) -> Vec<PlayerBoxScore> {
    let minimum_spanning_era = minimum_spanning_era(year);

    minimum_spanning_era
        .iter()
        .flat_map(|&season| {
            fetch_and_process_nba_games(season, NBAStatKind::Player)
                .into_iter()
                .filter_map(|stat| match stat {
                    Player(p) => Some(p),
                    _ => None,
                })
        })
        .collect()
}

pub fn team_games(year: i32, roster: Vec<PlayerBoxScore>) -> Vec<TeamBoxScore> {
    let minimum_spanning_era = minimum_spanning_era(year);

    let mut games: Vec<TeamBoxScore> = minimum_spanning_era
        .iter()
        .flat_map(|&season| {
            fetch_and_process_nba_games(season, NBAStatKind::Team)
                .into_iter()
                .filter_map(|stat| match stat {
                    Team(t) => Some(t),
                    _ => None,
                })
        })
        .collect();

    for player in roster {
        for team in &mut games {
            if player.played_in(&team) {
                team.add_player_stats(player.clone());
            }
        }
    }

    games.clone()
}

pub async fn fetch_and_save_nba_stats(season: &SeasonId, stat: NBAStatKind) -> Result<(), String> {
    let file_path = || nba_data_path(season, stat);

    let (year, _period) = season.destructure();

    match hunting::query_nba(season, stat).await {
        Ok(response_data) => match write_games(file_path(), &response_data) {
            Ok(_) => {
                println!(
                    "✅ successfully saved nba stats for {} season at file: {:?}",
                    season_fmt(season.year()),
                    file_path()
                );
                Ok(())
            }
            Err(e) => Err(format!(
                "❌ error saving nba stats for {} season at file {:?}: {}",
                season_fmt(season.year()),
                file_path(),
                e
            )),
        },
        Err(e) => Err(format!(
            "❌ failed to fetch {} stats for {} season: {:?}",
            year, stat, e
        )),
    }
}
