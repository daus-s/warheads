use crate::dapi::season_manager::get_era_by_date;

use crate::format::path_manager::{nba_source_path, nba_storage_file};

use crate::stats::nba_kind::NBAStatKind;
use crate::stats::teamcard::TeamCard;
use crate::stats::visiting::Visiting;

use crate::types::{GameDate, GameId, SeasonId};

use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

use derive_builder::Builder;

use regex::Regex;

use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameCard {
    game_id: GameId,
    season_id: SeasonId,
    date: GameDate,
    home: TeamCard,
    away: TeamCard,
}

impl GameCard {
    pub fn new(
        game_id: GameId,
        season_id: SeasonId,
        date: GameDate,
        home: TeamCard,
        away: TeamCard,
    ) -> Self {
        GameCard {
            game_id,
            season_id,
            date,
            home,
            away,
        }
    }

    //is this seach inefficient reading the file each time for each game?
    pub fn check_source_data(&self) -> bool {
        let era = get_era_by_date(self.date);

        let team_source_path = nba_source_path(era, NBAStatKind::Team);

        let contents = match fs::read_to_string(team_source_path) {
            Ok(contents) => contents,
            Err(_) => return false,
        };

        let home_regex = self.home_regex();
        let away_regex = self.away_regex();

        home_regex.is_match(&contents) && away_regex.is_match(&contents)
    }

    pub fn get_storage_path(&self) -> PathBuf {
        // let era = get_era_by_date(self.date);
        //this would lowkey smart except its not guaranteed to work on edge cases you maybe have to look else weher

        let team_source_path = nba_storage_file(self.season_id, self.game_id());

        team_source_path
    }

    pub fn home_regex(&self) -> Regex {
        self.regex(Visiting::Home)
    }

    pub fn away_regex(&self) -> Regex {
        self.regex(Visiting::Away)
    }

    fn regex(&self, visiting: Visiting) -> Regex {
        let (team_id, team_abbr) = match visiting {
            Visiting::Home => (self.home.team_id(), self.home.team_abbr()),
            Visiting::Away => (self.away.team_id(), self.away.team_abbr()),
        };

        let pattern = format!(
            r#"{},"{}","[^"]*","{}","{:?}"#,
            team_id, team_abbr, self.game_id, self.date
        );

        let re = Regex::new(&pattern).unwrap();

        re
    }

    pub fn date(&self) -> GameDate {
        self.date
    }
}

//todo: get this all squared as in shaped like even rows even columns for tui display
impl Display for GameCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Game ID: {}, Date: {}\nhome: {}\naway: {}",
            self.game_id, self.date, self.home, self.away
        )
    }
}
// getters and setters
impl GameCard {
    pub fn game_id(&self) -> GameId {
        self.game_id
    }

    pub fn home(&self) -> &TeamCard {
        &self.home
    }

    pub fn away(&self) -> &TeamCard {
        &self.away
    }

    pub fn mut_home(&mut self) -> &mut TeamCard {
        &mut self.home
    }

    pub fn mut_away(&mut self) -> &mut TeamCard {
        &mut self.away
    }
}

#[cfg(test)]
mod test_gamecard {
    use chrono::NaiveDate;

    use crate::{
        stats::record::Record,
        types::{TeamAbbreviation, TeamId, TeamName},
    };

    use super::*;

    #[test]
    fn test_regex() {
        let gamecard = test_gamecard();

        let home_regex = gamecard.home_regex();
        let away_regex = gamecard.away_regex();

        let text = r#"{"parameters":{"Counter":1000,"DateFrom":null,"DateTo":null,"Direction":"DESC","LeagueID":"00","PlayerOrTeam":"T","Season":"2025-26","SeasonType":"Regular Season","Sorter":"DATE"},"resource":"leaguegamelog","resultSets":[{"headers":["SEASON_ID","TEAM_ID","TEAM_ABBREVIATION","TEAM_NAME","GAME_ID","GAME_DATE","MATCHUP","WL","MIN","FGM","FGA","FG_PCT","FG3M","FG3A","FG3_PCT","FTM","FTA","FT_PCT","OREB","DREB","REB","AST","STL","BLK","TOV","PF","PTS","PLUS_MINUS","VIDEO_AVAILABLE"],"name":"LeagueGameLog","rowSet":[["22025",1610612742,"DAL","Dallas Mavericks","0022500004","2025-10-22","DAL vs. SAS","L",240,31,83,0.373,9,21,0.429,21,26,0.808,9,28,37,20,5,3,16,24,92,-33,1],["22025",1610612757,"POR","Portland Trail Blazers","0022500089","2025-10-22","POR vs. MIN","L",240,36,90,0.4,13,42,0.31,29,35,0.829,7,32,39,25,10,3,16,17,114,-4,1],["22025",1610612737,"ATL","Atlanta Hawks","0022500082","2025-10-22","ATL vs. TOR","L",240,38,90,0.422,10,35,0.286,32,37,0.865,8,26,34,25,7,6,16,24,118,-20,1],["22025",1610612738,"BOS","Boston Celtics","0022500083","2025-10-22","BOS vs. PHI","L",240,41,91,0.451,11,43,0.256,23,26,0.885,10,32,42,16,7,4,11,28,116,-1,1],["22025",1610612748,"MIA","Miami Heat","0022500081","2025-10-22","MIA @ ORL","L",240,44,91,0.484,12,35,0.343,21,27,0.778,10,37,47,26,9,4,19,28,121,-4,1],["22025",1610612751,"BKN","Brooklyn Nets","0022500080","2025-10-22","BKN @ CHA","L",240,39,88,0.443,14,40,0.35,25,34,0.735,12,26,38,25,8,3,17,28,117,-19,1],["22025",1610612752,"NYK","New York Knicks","0022500003","2025-10-22","NYK vs. CLE","W",240,37,86,0.43,14,40,0.35,31,36,0.861,9,39,48,22,8,4,14,23,119,8,1],["22025",1610612766,"CHA","Charlotte Hornets","0022500080","2025-10-22","CHA vs. BKN","W",240,48,90,0.533,17,36,0.472,23,33,0.697,14,36,50,33,5,5,17,25,136,19,1],["22025",1610612755,"PHI","Philadelphia 76ers","0022500083","2025-10-22","PHI @ BOS","W",240,39,85,0.459,16,40,0.4,23,30,0.767,10,30,40,23,3,4,14,23,117,1,1],["22025",1610612764,"WAS","Washington Wizards","0022500086","2025-10-22","WAS @ MIL","L",240,46,98,0.469,17,42,0.405,11,19,0.579,13,43,56,23,5,4,13,19,120,-13,1],["22025",1610612761,"TOR","Toronto Raptors","0022500082","2025-10-22","TOR @ ATL","W",240,54,95,0.568,6,25,0.24,24,29,0.828,12,42,54,36,10,4,19,31,138,20,1],["22025",1610612741,"CHI","Chicago Bulls","0022500084","2025-10-22","CHI vs. DET","W",240,39,87,0.448,11,30,0.367,26,32,0.813,12,38,50,29,6,8,18,23,115,4,1],["22025",1610612749,"MIL","Milwaukee Bucks","0022500086","2025-10-22","MIL vs. WAS","W",240,50,98,0.51,18,44,0.409,15,19,0.789,6,40,46,31,8,6,8,22,133,13,1],["22025",1610612765,"DET","Detroit Pistons","0022500084","2025-10-22","DET @ CHI","L",240,43,97,0.443,7,24,0.292,18,26,0.692,13,33,46,29,8,10,10,29,111,-4,1],["22025",1610612740,"NOP","New Orleans Pelicans","0022500085","2025-10-22","NOP @ MEM","L",240,45,98,0.459,10,27,0.37,22,31,0.71,17,30,47,20,10,4,15,30,122,-6,1],["22025",1610612763,"MEM","Memphis Grizzlies","0022500085","2025-10-22","MEM vs. NOP","W",240,41,80,0.513,11,37,0.297,35,43,0.814,6,33,39,20,5,12,17,29,128,6,1],["22025",1610612739,"CLE","Cleveland Cavaliers","0022500003","2025-10-22","CLE @ NYK","L",240,40,86,0.465,19,46,0.413,12,18,0.667,4,28,32,21,8,4,15,25,111,-8,1],["22025",1610612753,"ORL","Orlando Magic","0022500081","2025-10-22","ORL vs. MIA","W",240,42,90,0.467,12,30,0.4,29,35,0.829,9,37,46,23,8,7,15,20,125,4,1],["22025",1610612759,"SAS","San Antonio Spurs","0022500004","2025-10-22","SAS @ DAL","W",240,50,87,0.575,8,20,0.4,17,28,0.607,11,39,50,24,10,5,15,24,125,33,1],["22025",1610612756,"PHX","Phoenix Suns","0022500088","2025-10-22","PHX vs. SAC","W",240,45,95,0.474,10,34,0.294,20,28,0.714,18,33,51,23,7,7,16,25,120,4,1],["22025",1610612750,"MIN","Minnesota Timberwolves","0022500089","2025-10-22","MIN @ POR","W",240,42,86,0.488,15,30,0.5,19,24,0.792,10,35,45,21,10,10,19,29,118,4,1],["22025",1610612758,"SAC","Sacramento Kings","0022500088","2025-10-22","SAC @ PHX","L",240,47,94,0.5,9,24,0.375,13,23,0.565,9,28,37,24,10,10,13,22,116,-4,1],["22025",1610612762,"UTA","Utah Jazz","0022500087","2025-10-22","UTA vs. LAC","W",240,48,87,0.552,17,44,0.386,16,20,0.8,13,33,46,38,11,6,13,21,129,21,1],["22025",1610612746,"LAC","LA Clippers","0022500087","2025-10-22","LAC @ UTA","L",240,39,88,0.443,14,45,0.311,16,21,0.762,15,23,38,28,8,6,15,21,108,-21,1],["22025",1610612745,"HOU","Houston Rockets","0022500001","2025-10-21","HOU @ OKC","L",290,43,97,0.443,11,39,0.282,27,31,0.871,16,36,52,23,6,5,25,26,124,-1,1],["22025",1610612744,"GSW","Golden State Warriors","0022500002","2025-10-21","GSW @ LAL","W",240,38,78,0.487,17,40,0.425,26,29,0.897,9,31,40,29,10,4,19,27,119,10,1],["22025",1610612747,"LAL","Los Angeles Lakers","0022500002","2025-10-21","LAL vs. GSW","L",240,42,77,0.545,8,32,0.25,17,28,0.607,7,32,39,23,7,2,20,21,109,-10,1],["22025",1610612760,"OKC","Oklahoma City Thunder","0022500001","2025-10-21","OKC vs. HOU","W",290,46,104,0.442,13,52,0.25,20,25,0.8,11,27,38,29,12,4,12,27,125,1,1]]}]}"#;

        assert!(home_regex.is_match(&text));
        assert!(away_regex.is_match(&text));
    }

    fn test_gamecard() -> GameCard {
        let day = NaiveDate::from_ymd_opt(2025, 10, 21).unwrap(); //fuck timezones

        let tch = TeamCard::new(
            TeamId(1610612747),
            TeamName("Los Angeles Lakers".to_owned()),
            TeamAbbreviation("LAL".to_owned()),
            Record { wins: 0, losses: 1 },
        );

        let tca = TeamCard::new(
            TeamId(1610612744),
            TeamName("Golden State Warriors".to_owned()),
            TeamAbbreviation("GSW".to_owned()),
            Record { wins: 1, losses: 0 },
        );

        let gc = GameCard::new(
            GameId(22500002),
            SeasonId::from(22025),
            day.into(),
            tch,
            tca,
        );

        gc
    }

    fn expected_gamecards() -> Vec<GameCard> {
        let day1 = NaiveDate::from_ymd_opt(2025, 10, 21).unwrap();
        let day2 = NaiveDate::from_ymd_opt(2025, 10, 21).unwrap(); //fuck timezones

        let tc1h = TeamCard::new(
            TeamId(1610612760),
            TeamName("Oklahoma City Thunder".to_owned()),
            TeamAbbreviation("OKC".to_owned()),
            Record { wins: 1, losses: 0 },
        );

        let tc1a = TeamCard::new(
            TeamId(1610612745),
            TeamName("Houston Rockets".to_owned()),
            TeamAbbreviation("HOU".to_owned()),
            Record { wins: 0, losses: 1 },
        );

        let g1 = GameCard::new(
            GameId(22500001),
            SeasonId::from(22025),
            day1.into(),
            tc1h,
            tc1a,
        );

        let tc2h = TeamCard::new(
            TeamId(1610612747),
            TeamName("Los Angeles Lakers".to_owned()),
            TeamAbbreviation("LAL".to_owned()),
            Record { wins: 0, losses: 1 },
        );

        let tc2a = TeamCard::new(
            TeamId(1610612744),
            TeamName("Golden State Warriors".to_owned()),
            TeamAbbreviation("GSW".to_owned()),
            Record { wins: 1, losses: 0 },
        );

        let g2 = GameCard::new(
            GameId(22500002),
            SeasonId::from(22025),
            day2.into(),
            tc2h,
            tc2a,
        );

        vec![g1, g2]
    }

    #[test]
    fn test_serialization() {
        let gc = test_gamecard();

        let expected = r#"{"game_id":"0022500002","season_id":"22025","date":"2025-10-21","home":{"team_id":1610612747,"team_name":"Los Angeles Lakers","team_abbr":"LAL","record":"0-1"},"away":{"team_id":1610612744,"team_name":"Golden State Warriors","team_abbr":"GSW","record":"1-0"}}"#;

        let actual = serde_json::to_string(&gc)
            .expect("failed to serialize struct that is specified serializable");

        pretty_assertions::assert_eq!(expected, actual);
    }

    #[test]
    fn test_vec_serialization() {
        let vec = expected_gamecards();

        let expected = r#"[{"game_id":"0022500001","season_id":"22025","date":"2025-10-21","home":{"team_id":1610612760,"team_name":"Oklahoma City Thunder","team_abbr":"OKC","record":"1-0"},"away":{"team_id":1610612745,"team_name":"Houston Rockets","team_abbr":"HOU","record":"0-1"}},{"game_id":"0022500002","season_id":"22025","date":"2025-10-21","home":{"team_id":1610612747,"team_name":"Los Angeles Lakers","team_abbr":"LAL","record":"0-1"},"away":{"team_id":1610612744,"team_name":"Golden State Warriors","team_abbr":"GSW","record":"1-0"}}]"#;

        let actual = serde_json::to_string(&vec).unwrap();

        pretty_assertions::assert_eq!(expected, actual);
    }
}
