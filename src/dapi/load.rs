use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;

use crate::dapi::gather::{player_games, team_games};
use crate::dapi::team_box_score::TeamBoxScore;
use crate::format::path_manager::nba_storage_path;
use crate::stats::game_obj::GameObject;
use crate::stats::id::Identity;
use crate::stats::season_period::minimum_spanning_era;

pub fn load_nba_season_from_file(year: i32) -> Vec<(Identity, TeamBoxScore)> {
    let player_games = player_games(year);

    team_games(year, player_games)
}

pub fn load_nba_season_games_objects(year: i32) -> Vec<GameObject> {
    let periods = minimum_spanning_era(year);

    let mut games = Vec::new();

    for period in periods {
        let path = nba_storage_path(period);

        fs::create_dir_all(&path).expect("💀 failed to create directory. ");

        for dir_entry in
            fs::read_dir(path).expect("💀 failed to read directory even after creating it.")
        {
            let file =
                dir_entry.expect("💀 failed to get directory entry from created directory. ");

            if let Ok(game) = load_game_object(file.path()) {
                games.push(game);
            } else {
                eprintln!("❌ failed to load game object: {}", file.path().display());
            }
        }
    }

    games
}

fn load_game_object(path: PathBuf) -> Result<GameObject, String> {
    let file =
        File::open(path).map_err(|e| format!("❌ failed to open game object file: {}", e))?;

    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
        .map_err(|e| format!("❌ failed to deserialize game object: {}", e))
}

#[cfg(test)]
mod test_load {
    use crate::{
        format::path_manager::nba_storage_file,
        types::{GameId, SeasonId},
    };

    use super::*;

    #[test]
    fn test_load_game() {
        let game = GameId::from("0042400113");

        let season = SeasonId::from(42024);

        let path = nba_storage_file(season, game);

        let actual_game = load_game_object(path).expect("couldn't open or read test_game_object. \nHINT: if actual game is empty try running the observe/revise/chronicle workflow.");

        let expected_game = serde_json::from_str::<GameObject>(&game_as_string())
            .expect("couldnt parse GameObject from test string.");

        assert_eq!(actual_game, expected_game)
    }

    fn game_as_string() -> String {
        r#"{"season_id":"42024","game_date":"2025-04-25","game_id":"0042400113","home":{"team_id":1610612753,"team_abbreviation":"ORL","team_name":"Orlando Magic","visiting":"Home","roster":[{"player_id":1630175,"player_name":"Cole Anthony","box_score":{"wl":"W","min":14,"fgm":2,"fga":4,"fg3m":0,"fg3a":1,"ftm":0,"fta":0,"oreb":2,"dreb":2,"reb":4,"ast":2,"stl":0,"blk":0,"tov":5,"pf":4,"pts":4,"plus_minus":15}},{"player_id":203484,"player_name":"Kentavious Caldwell-Pope","box_score":{"wl":"W","min":32,"fgm":0,"fga":3,"fg3m":0,"fg3a":2,"ftm":0,"fta":0,"oreb":1,"dreb":1,"reb":2,"ast":2,"stl":1,"blk":1,"tov":1,"pf":2,"pts":0,"plus_minus":-10}},{"player_id":202709,"player_name":"Cory Joseph","box_score":{"wl":"W","min":28,"fgm":2,"fga":4,"fg3m":2,"fg3a":4,"ftm":0,"fta":0,"oreb":0,"dreb":3,"reb":3,"ast":1,"stl":0,"blk":0,"tov":1,"pf":1,"pts":6,"plus_minus":-9}},{"player_id":1641710,"player_name":"Anthony Black","box_score":{"wl":"W","min":13,"fgm":3,"fga":8,"fg3m":0,"fg3a":4,"ftm":3,"fta":4,"oreb":1,"dreb":1,"reb":2,"ast":0,"stl":2,"blk":0,"tov":0,"pf":3,"pts":9,"plus_minus":16}},{"player_id":1628371,"player_name":"Jonathan Isaac","box_score":{"wl":"W","min":17,"fgm":1,"fga":1,"fg3m":0,"fg3a":0,"ftm":0,"fta":0,"oreb":1,"dreb":2,"reb":3,"ast":1,"stl":1,"blk":1,"tov":0,"pf":1,"pts":2,"plus_minus":10}},{"player_id":203914,"player_name":"Gary Harris","box_score":{"wl":"W","min":22,"fgm":0,"fga":2,"fg3m":0,"fg3a":2,"ftm":0,"fta":0,"oreb":0,"dreb":2,"reb":2,"ast":0,"stl":1,"blk":1,"tov":0,"pf":2,"pts":0,"plus_minus":9}},{"player_id":1630532,"player_name":"Franz Wagner","box_score":{"wl":"W","min":38,"fgm":11,"fga":27,"fg3m":3,"fg3a":13,"ftm":7,"fta":12,"oreb":2,"dreb":5,"reb":7,"ast":8,"stl":2,"blk":0,"tov":1,"pf":4,"pts":32,"plus_minus":-4}},{"player_id":1631094,"player_name":"Paolo Banchero","box_score":{"wl":"W","min":42,"fgm":10,"fga":25,"fg3m":2,"fg3a":4,"ftm":7,"fta":8,"oreb":2,"dreb":4,"reb":6,"ast":1,"stl":2,"blk":0,"tov":4,"pf":4,"pts":29,"plus_minus":-3}},{"player_id":1631216,"player_name":"Caleb Houstan","box_score":{"wl":"W","min":3,"fgm":1,"fga":1,"fg3m":1,"fg3a":1,"ftm":0,"fta":0,"oreb":0,"dreb":1,"reb":1,"ast":0,"stl":0,"blk":0,"tov":0,"pf":0,"pts":3,"plus_minus":-4}},{"player_id":1628976,"player_name":"Wendell Carter Jr.","box_score":{"wl":"W","min":31,"fgm":4,"fga":8,"fg3m":0,"fg3a":1,"ftm":2,"fta":2,"oreb":6,"dreb":6,"reb":12,"ast":1,"stl":0,"blk":1,"tov":1,"pf":2,"pts":10,"plus_minus":-10}}],"box_score":{"wl":"W","min":240,"fgm":34,"fga":83,"fg3m":8,"fg3a":32,"ftm":19,"fta":26,"oreb":15,"dreb":27,"reb":42,"ast":16,"stl":9,"blk":4,"tov":14,"pf":23,"pts":95,"plus_minus":2}},"away":{"team_id":1610612738,"team_abbreviation":"BOS","team_name":"Boston Celtics","visiting":"Away","roster":[{"player_id":1628369,"player_name":"Jayson Tatum","box_score":{"wl":"L","min":39,"fgm":10,"fga":22,"fg3m":4,"fg3a":10,"ftm":12,"fta":12,"oreb":2,"dreb":7,"reb":9,"ast":4,"stl":1,"blk":1,"tov":7,"pf":1,"pts":36,"plus_minus":2}},{"player_id":204001,"player_name":"Kristaps Porziņģis","box_score":{"wl":"L","min":34,"fgm":3,"fga":10,"fg3m":0,"fg3a":3,"ftm":1,"fta":3,"oreb":0,"dreb":6,"reb":6,"ast":1,"stl":1,"blk":0,"tov":2,"pf":4,"pts":7,"plus_minus":-16}},{"player_id":201143,"player_name":"Al Horford","box_score":{"wl":"L","min":34,"fgm":2,"fga":5,"fg3m":2,"fg3a":3,"ftm":0,"fta":0,"oreb":2,"dreb":5,"reb":7,"ast":3,"stl":0,"blk":2,"tov":0,"pf":4,"pts":6,"plus_minus":6}},{"player_id":1628436,"player_name":"Luke Kornet","box_score":{"wl":"L","min":19,"fgm":3,"fga":6,"fg3m":0,"fg3a":0,"ftm":0,"fta":0,"oreb":3,"dreb":3,"reb":6,"ast":0,"stl":2,"blk":0,"tov":0,"pf":1,"pts":6,"plus_minus":8}},{"player_id":1630573,"player_name":"Sam Hauser","box_score":{"wl":"L","min":13,"fgm":0,"fga":1,"fg3m":0,"fg3a":1,"ftm":0,"fta":0,"oreb":0,"dreb":1,"reb":1,"ast":1,"stl":0,"blk":0,"tov":1,"pf":2,"pts":0,"plus_minus":0}},{"player_id":1627759,"player_name":"Jaylen Brown","box_score":{"wl":"L","min":34,"fgm":7,"fga":14,"fg3m":1,"fg3a":2,"ftm":4,"fta":5,"oreb":0,"dreb":6,"reb":6,"ast":1,"stl":1,"blk":1,"tov":6,"pf":5,"pts":19,"plus_minus":-8}},{"player_id":1628401,"player_name":"Derrick White","box_score":{"wl":"L","min":40,"fgm":7,"fga":14,"fg3m":2,"fg3a":8,"ftm":0,"fta":0,"oreb":0,"dreb":3,"reb":3,"ast":5,"stl":2,"blk":1,"tov":0,"pf":0,"pts":16,"plus_minus":4}},{"player_id":1630202,"player_name":"Payton Pritchard","box_score":{"wl":"L","min":26,"fgm":1,"fga":2,"fg3m":0,"fg3a":0,"ftm":1,"fta":2,"oreb":1,"dreb":0,"reb":1,"ast":0,"stl":2,"blk":0,"tov":3,"pf":3,"pts":3,"plus_minus":-6}}],"box_score":{"wl":"L","min":240,"fgm":33,"fga":74,"fg3m":9,"fg3a":27,"ftm":18,"fta":22,"oreb":8,"dreb":31,"reb":39,"ast":15,"stl":9,"blk":5,"tov":21,"pf":20,"pts":93,"plus_minus":-2}}}
            "#.to_string()
    }
}
