#![allow(dead_code)]
#![allow(unused_imports)]

use crate::constants::paths::test;

use crate::dapi::write::write_games;

use crate::proc::gather::{player_games, team_games};
use crate::proc::query::make_nba_request;
use crate::proc::store::pair_off;

use crate::stats::game_obj::GameObject;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_period::SeasonPeriod;

use crate::types::{GameId, SeasonId};

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use once_cell::sync::Lazy;

static TEST: Lazy<String> = Lazy::new(test);
static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

#[cfg(test)]
mod async_tests {
    use super::*;

    #[tokio::test]
    async fn test_download() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let season = SeasonId::from((2024, SeasonPeriod::PostSeason));
        // test request info
        // url: https://www.nba.com/stats/teams/boxscores?Season=2024-25&SeasonType=Playoffs&DateFrom=04%2F26%2F2025&DateTo=04%2F30%2F2025

        let from = String::from("04/26/2025");
        let to = String::from("04/30/2025");

        // TEAM //////////////////////////////////////////////////////////
        let team_path = PathBuf::from(format!("{}/data/data/tg.json", *TEST));

        let team_response = make_nba_request(
            season,
            NBAStatKind::Team,
            Some(from.clone()),
            Some(to.clone()),
        )
        .await
        .expect("💀 failed to make request to nba.com/stats (Team)");

        let team_body = team_response
            .text()
            .await
            .expect("💀 failed to get body of nba team response. ");

        let team_json = serde_json::from_str(&team_body)
            .expect("💀 failed to parse json from nba team response");

        assert!(write_games(&team_path, &team_json).is_ok());

        let expected_team_file =
            fs::read_to_string(PathBuf::from(format!("{}/data/expected_tg.json", *TEST))).expect(
                &format!(
                    "💀 failed to read test team data string: {}/data/expected_tg.json",
                    *TEST,
                ),
            );

        let actual_team_file =
            fs::read_to_string(&team_path).expect("💀 failed to read fetched team directory");

        pretty_assertions::assert_eq!(
            expected_team_file.trim_end(),
            actual_team_file,
            "💀 team data download failed"
        );

        // PLAYER //////////////////////////////////////////////////////////
        let player_response = make_nba_request(
            season,
            NBAStatKind::Player,
            Some(from.clone()),
            Some(to.clone()),
        )
        .await
        .expect("💀 failed to make request to nba.com/stats (Player)");

        let player_body = player_response
            .text()
            .await
            .expect("💀 failed to get body of nba team response. ");

        let player_json = serde_json::from_str(&player_body)
            .expect("💀 failed to parse json from nba team response");

        let player_path = PathBuf::from(format!("{}/data/data/pg.json", *TEST));

        assert!(write_games(&player_path, &player_json).is_ok());

        let expected_player_file =
            fs::read_to_string(PathBuf::from(format!("{}/data/expected_pg.json", *TEST))).expect(
                &format!(
                    "💀 failed to read test player data string: {}/data/expected_pg.json",
                    *TEST,
                ),
            );

        let actual_player_file =
            fs::read_to_string(&player_path).expect("💀 failed to read fetched player directory");

        pretty_assertions::assert_eq!(expected_player_file.trim_end(), actual_player_file);
    }
}

#[cfg(test)]
mod test_injest {
    use super::*;

    #[tokio::test]
    async fn test_workflow() {
        let _guard = TEST_MUTEX.lock().unwrap();
        clean_up();

        download().await;

        serialize();
        deserialize();

        clean_up();

        download().await;
        serialize();
    }

    async fn download() {
        let season = SeasonId::from((2024, SeasonPeriod::PostSeason));
        // test request info
        // url: https://www.nba.com/stats/teams/boxscores?Season=2024-25&SeasonType=Playoffs&DateFrom=04%2F26%2F2025&DateTo=04%2F30%2F2025

        let from = String::from("04/26/2025");
        let to = String::from("04/30/2025");

        // TEAM //////////////////////////////////////////////////////////
        let team_path = PathBuf::from(format!("{}/data/data/tg.json", *TEST));

        let team_response = make_nba_request(
            season,
            NBAStatKind::Team,
            Some(from.clone()),
            Some(to.clone()),
        )
        .await
        .expect("💀 failed to make request to nba.com/stats (Team)");

        let team_body = team_response
            .text()
            .await
            .expect("💀 failed to get body of nba team response. ");

        let team_json = serde_json::from_str(&team_body)
            .expect("💀 failed to parse json from nba team response");

        assert!(write_games(&team_path, &team_json).is_ok());

        let expected_team_file =
            fs::read_to_string(PathBuf::from(format!("{}/data/expected_tg.json", *TEST))).expect(
                &format!(
                    "💀 failed to read test team data string: {}/data/expected_tg.json",
                    *TEST,
                ),
            );

        let actual_team_file =
            fs::read_to_string(&team_path).expect("💀 failed to read fetched team directory");

        pretty_assertions::assert_eq!(
            expected_team_file.trim_end(),
            actual_team_file,
            "💀 team data download failed"
        );

        // PLAYER //////////////////////////////////////////////////////////
        let player_response = make_nba_request(
            season,
            NBAStatKind::Player,
            Some(from.clone()),
            Some(to.clone()),
        )
        .await
        .expect("💀 failed to make request to nba.com/stats (Player)");

        let player_body = player_response
            .text()
            .await
            .expect("💀 failed to get body of nba team response. ");

        let player_json = serde_json::from_str(&player_body)
            .expect("💀 failed to parse json from nba team response");

        let player_path = PathBuf::from(format!("{}/data/data/pg.json", *TEST));

        assert!(write_games(&player_path, &player_json).is_ok());

        let expected_player_file =
            fs::read_to_string(PathBuf::from(format!("{}/data/expected_pg.json", *TEST))).expect(
                &format!(
                    "💀 failed to read test player data string: {}/data/expected_pg.json",
                    *TEST,
                ),
            );

        let actual_player_file =
            fs::read_to_string(&player_path).expect("💀 failed to read fetched player directory");

        pretty_assertions::assert_eq!(expected_player_file.trim_end(), actual_player_file);
    }

    fn serialize() {
        let period = SeasonId::from((2024, SeasonPeriod::PostSeason));

        let mut team_games_vec = Vec::new();

        let player_path = player_source_path();

        let player_games_of_period = player_games(period, &player_path).unwrap_or_else(|e| {
            panic!(
                "{e}\n\
                    💀 failed to load and parse player games as JSON.\n\
                    run `cargo test checksum::assert_checksums`"
            );
        });

        let team_path = team_source_path();

        let team_games_of_period = team_games(period, &team_path, player_games_of_period)
            .unwrap_or_else(|e| {
                panic!(
                    "{e}\n\
                        💀 failed to load and parse team games as JSON.\n\
                        run `cargo test checksum::assert_checksums`"
                );
            });

        team_games_vec.extend(team_games_of_period);

        let games =
            pair_off(team_games_vec).expect("💀 created test games with no corrections to make.");

        for game in games.iter() {
            let path = storage_file(game.game_id());

            let contents = serde_json::to_string_pretty(game).expect("💀 failed to serialize game");

            fs::write(&path, contents).expect("💀 failed to write to the file");
        }

        for game in games.iter() {
            let expected_path = expected_storage_file(game.game_id());
            let actual_path = storage_file(game.game_id());

            let expected_contents =
                fs::read_to_string(&expected_path).expect("💀 failed to read expected game");
            let actual_contents =
                fs::read_to_string(&actual_path).expect("💀 failed to read actual game");

            assert_eq!(expected_contents, actual_contents);
        }
    }

    fn deserialize() {
        let mut game_objects = read_directory(&storage_dir()).expect("💀 failed to read directory");

        game_objects.sort_by(|game1, game2| game1.game_id().cmp(&game2.game_id()));

        let mut expected_game_objects =
            read_directory(&expected_storage_dir()).expect("💀 failed to read expected directory");

        expected_game_objects.sort_by(|game1, game2| game1.game_id().cmp(&game2.game_id()));

        assert_eq!(game_objects, expected_game_objects);
    }

    fn clean_up() {
        let storage_dir = storage_dir();
        let source_dir = source_dir();

        fs::remove_dir_all(&storage_dir).expect("💀 failed to clean up test storage file system.");
        fs::remove_dir_all(&source_dir).expect("💀 failed to clean up test source file system.");

        fs::create_dir_all(&storage_dir).expect("💀 failed to reinitialize storage directory.");
        fs::create_dir_all(&source_dir).expect("💀 failed to reinitialize source directory.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let period = SeasonId::from((2024, SeasonPeriod::PostSeason));

        let mut team_games_vec = Vec::new();

        let player_path = player_source_path();

        let player_games_of_period = player_games(period, &player_path).unwrap_or_else(|e| {
            panic!(
                "{e}\n\
                    💀 failed to load and parse player games as JSON.\n\
                    run `cargo test checksum::assert_checksums`"
            );
        });

        let team_path = team_source_path();

        let team_games_of_period = team_games(period, &team_path, player_games_of_period)
            .unwrap_or_else(|e| {
                panic!(
                    "{e}\n\
                        💀 failed to load and parse team games as JSON.\n\
                        run `cargo test checksum::assert_checksums`"
                );
            });

        team_games_vec.extend(team_games_of_period);

        let games = pair_off(team_games_vec).expect(
            "💀 failed to create test games from test data. unexpectedly created corrections",
        );

        for game in games.iter() {
            let path = storage_file(game.game_id());

            let contents = serde_json::to_string_pretty(game).expect("💀 failed to serialize game");

            fs::write(&path, contents).expect("💀 failed to write to the file");
        }

        for game in games.iter() {
            let expected_path = expected_storage_file(game.game_id());
            let actual_path = storage_file(game.game_id());

            let expected_contents =
                fs::read_to_string(&expected_path).expect("💀 failed to read expected game");
            let actual_contents =
                fs::read_to_string(&actual_path).expect("💀 failed to read actual game");

            assert_eq!(expected_contents, actual_contents);
        }
    }

    #[test]
    fn test_deserialize() {
        let _guard = TEST_MUTEX.lock().unwrap();
        let mut game_objects = read_directory(&storage_dir()).expect("💀 failed to read directory");

        game_objects.sort_by(|game1, game2| game1.game_id().cmp(&game2.game_id()));

        let mut expected_game_objects =
            read_directory(&expected_storage_dir()).expect("💀 failed to read expected directory");

        expected_game_objects.sort_by(|game1, game2| game1.game_id().cmp(&game2.game_id()));

        assert_eq!(game_objects, expected_game_objects);
    }
}

fn player_source_path() -> PathBuf {
    PathBuf::from(format!("{}/data/data/pg.json", *TEST))
}

fn team_source_path() -> PathBuf {
    PathBuf::from(format!("{}/data/data/tg.json", *TEST))
}

fn source_dir() -> PathBuf {
    PathBuf::from(format!("{}/data/data", *TEST))
}

fn storage_file(game: GameId) -> PathBuf {
    PathBuf::from(format!("{}/data/store/{}", *TEST, game))
}

fn expected_storage_file(game: GameId) -> PathBuf {
    PathBuf::from(format!("{}/data/expected_store/expected_{}", *TEST, game))
}

fn expected_storage_dir() -> PathBuf {
    PathBuf::from(format!("{}/data/expected_store", *TEST))
}

fn storage_dir() -> PathBuf {
    PathBuf::from(format!("{}/data/store", *TEST))
}

fn read_directory(path: &PathBuf) -> Result<Vec<GameObject>, String> {
    let files = fs::read_dir(path).map_err(|e| format!("❌ failed to read directory: {e}"))?;

    let mut games = Vec::new();

    for file in files {
        match file {
            Ok(entry) => {
                let s = fs::read_to_string(entry.path())
                    .map_err(|e| format!("❌ failed to read file {:?}: {e}", entry.file_name()))?;

                let game = serde_json::from_str::<GameObject>(&s).map_err(|e| {
                    format!("❌ couldn't parse json for {:?}: {e}", entry.file_name())
                })?;

                games.push(game);
            }
            Err(e) => return Err(format!("❌ failed to get an entry from file: {e}")),
        }
    }

    Ok(games)
}
