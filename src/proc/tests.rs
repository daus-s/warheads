#[cfg(test)]
mod test_transform {
    use std::fs;
    use std::path::PathBuf;

    use once_cell::sync::Lazy;

    use crate::constants::paths::test;

    use crate::proc::gather::{player_games, team_games};
    use crate::proc::store::pair_off;
    use crate::stats::season_period::SeasonPeriod;
    use crate::types::{GameId, SeasonId};

    static TEST: Lazy<String> = Lazy::new(test);

    #[test]
    fn test_serialization() {
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
            let path = storage_path(game.game_id());

            let contents = serde_json::to_string_pretty(game).expect("💀 failed to serialize game");

            fs::write(&path, contents).expect("💀 failed to write to the file");
        }

        for game in games.iter() {
            let expected_path = expected_storage_path(game.game_id());
            let actual_path = storage_path(game.game_id());

            let expected_contents =
                fs::read_to_string(&expected_path).expect("💀 failed to read expected game");
            let actual_contents =
                fs::read_to_string(&actual_path).expect("💀 failed to read actual game");

            assert_eq!(expected_contents, actual_contents);
        }
    }

    fn player_source_path() -> PathBuf {
        PathBuf::from(format!("{}/data/data/pg.json", *TEST))
    }

    fn team_source_path() -> PathBuf {
        PathBuf::from(format!("{}/data/data/tg.json", *TEST))
    }

    fn storage_path(game: GameId) -> PathBuf {
        PathBuf::from(format!("{}/data/store/{}", *TEST, game))
    }

    fn expected_storage_path(game: GameId) -> PathBuf {
        PathBuf::from(format!("{}/data/expected_{}", *TEST, game))
    }
}

#[cfg(test)]
mod test_revision {}
