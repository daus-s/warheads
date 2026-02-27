use std::cmp::Ordering;

use crate::edit::edit::Edit;

impl Ord for Edit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Edit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Sort by game_id first (chronological game order)
        match self.game_id.cmp(&other.game_id) {
            Ordering::Equal => {
                // Within same game, sort by team_id (groups edits for same team)
                match self.team_id.cmp(&other.team_id) {
                    Ordering::Equal => {
                        // Within same team, sort by player_id
                        // None (team edits) comes before Some (player edits)
                        match (&self.player_id, &other.player_id) {
                            (None, Some(_)) => Some(Ordering::Less), // Team edit before player edit
                            (Some(_), None) => Some(Ordering::Greater), // Player edit after team edit
                            (Some(a), Some(b)) => {
                                if a == b {
                                    None
                                } else {
                                    Some(a.cmp(b))
                                }
                            }
                            (None, None) => Some(Ordering::Equal),
                        }
                    }
                    other_ordering => Some(other_ordering),
                }
            }
            other_ordering => Some(other_ordering),
        }
    }
}

#[cfg(test)]
mod test_edit_order {
    use std::{collections::HashMap, str::FromStr};

    use serde_json::Value;

    use super::*;
    use crate::{
        stats::stat_column::StatColumn,
        types::{GameDate, GameId, PlayerId, SeasonId, TeamAbbreviation, TeamId},
    };

    #[test]
    fn test_team_edit_order() {
        let edits = vec![
            // Game 1 - Team edits
            Edit {
                game_id: GameId(1),
                game_date: GameDate::ymd(2022, 1, 15).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: None,
                team_id: TeamId(1),
                team_abbr: TeamAbbreviation::from_str("LAL")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(110)),
                    (StatColumn::REB, Value::from(45)),
                ]),
            },
            Edit {
                game_id: GameId(1),
                game_date: GameDate::ymd(2022, 1, 15).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: Some(PlayerId(101)),
                team_id: TeamId(1),
                team_abbr: TeamAbbreviation::from_str("LAL")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(28)),
                    (StatColumn::AST, Value::from(7)),
                ]),
            },
            Edit {
                game_id: GameId(1),
                game_date: GameDate::ymd(2022, 1, 15).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: Some(PlayerId(102)),
                team_id: TeamId(1),
                team_abbr: TeamAbbreviation::from_str("LAL")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(22)),
                    (StatColumn::REB, Value::from(11)),
                ]),
            },
            Edit {
                game_id: GameId(1),
                game_date: GameDate::ymd(2022, 1, 15).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: None,
                team_id: TeamId(2),
                team_abbr: TeamAbbreviation::from_str("GSW")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(108)),
                    (StatColumn::AST, Value::from(28)),
                ]),
            },
            Edit {
                game_id: GameId(1),
                game_date: GameDate::ymd(2022, 1, 15).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: Some(PlayerId(201)),
                team_id: TeamId(2),
                team_abbr: TeamAbbreviation::from_str("GSW")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(32)),
                    (StatColumn::FG3M, Value::from(6)),
                ]),
            },
            Edit {
                game_id: GameId(1),
                game_date: GameDate::ymd(2022, 1, 15).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: Some(PlayerId(202)),
                team_id: TeamId(2),
                team_abbr: TeamAbbreviation::from_str("GSW")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::AST, Value::from(9)),
                    (StatColumn::STL, Value::from(3)),
                ]),
            },
            Edit {
                game_id: GameId(2),
                game_date: GameDate::ymd(2022, 3, 10).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: None,
                team_id: TeamId(3),
                team_abbr: TeamAbbreviation::from_str("BOS")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(115)),
                    (StatColumn::REB, Value::from(52)),
                ]),
            },
            Edit {
                game_id: GameId(2),
                game_date: GameDate::ymd(2022, 3, 10).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: Some(PlayerId(301)),
                team_id: TeamId(3),
                team_abbr: TeamAbbreviation::from_str("BOS")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(30)),
                    (StatColumn::REB, Value::from(8)),
                ]),
            },
            Edit {
                game_id: GameId(2),
                game_date: GameDate::ymd(2022, 3, 10).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: Some(PlayerId(302)),
                team_id: TeamId(3),
                team_abbr: TeamAbbreviation::from_str("BOS")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(25)),
                    (StatColumn::AST, Value::from(6)),
                ]),
            },
            Edit {
                game_id: GameId(2),
                game_date: GameDate::ymd(2022, 3, 10).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: None,
                team_id: TeamId(4),
                team_abbr: TeamAbbreviation::from_str("MIA")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(112)),
                    (StatColumn::BLK, Value::from(8)),
                ]),
            },
            Edit {
                game_id: GameId(2),
                game_date: GameDate::ymd(2022, 3, 10).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: Some(PlayerId(401)),
                team_id: TeamId(4),
                team_abbr: TeamAbbreviation::from_str("MIA")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(27)),
                    (StatColumn::AST, Value::from(8)),
                ]),
            },
            Edit {
                game_id: GameId(2),
                game_date: GameDate::ymd(2022, 3, 10).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: Some(PlayerId(402)),
                team_id: TeamId(4),
                team_abbr: TeamAbbreviation::from_str("MIA")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::BLK, Value::from(4)),
                    (StatColumn::REB, Value::from(13)),
                ]),
            },
            Edit {
                game_id: GameId(3),
                game_date: GameDate::ymd(2022, 3, 17).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: None,
                team_id: TeamId(3),
                team_abbr: TeamAbbreviation::from_str("BOS")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(120)),
                    (StatColumn::REB, Value::from(48)),
                ]),
            },
            Edit {
                game_id: GameId(3),
                game_date: GameDate::ymd(2022, 3, 17).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: Some(PlayerId(301)),
                team_id: TeamId(3),
                team_abbr: TeamAbbreviation::from_str("BOS")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(35)),
                    (StatColumn::REB, Value::from(9)),
                ]),
            },
            Edit {
                game_id: GameId(3),
                game_date: GameDate::ymd(2022, 3, 17).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: None,
                team_id: TeamId(4),
                team_abbr: TeamAbbreviation::from_str("MIA")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(125)),
                    (StatColumn::BLK, Value::from(9)),
                ]),
            },
            Edit {
                game_id: GameId(3),
                game_date: GameDate::ymd(2022, 3, 17).expect("Failed to create GameDate"),
                season: SeasonId::from(22021),
                player_id: Some(PlayerId(401)),
                team_id: TeamId(4),
                team_abbr: TeamAbbreviation::from_str("MIA")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(30)),
                    (StatColumn::AST, Value::from(9)),
                ]),
            },
            Edit {
                game_id: GameId(4),
                game_date: GameDate::ymd(2022, 6, 1).expect("Failed to create GameDate"),
                season: SeasonId::from(22022),
                player_id: None,
                team_id: TeamId(1),
                team_abbr: TeamAbbreviation::from_str("BRK")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(150)), // Changed PTS value for team statistics
                    (StatColumn::REB, Value::from(40)),  // Changed REB value for team statistics
                ]),
            },
            Edit {
                game_id: GameId(4),
                game_date: GameDate::ymd(2022, 6, 1).expect("Failed to create GameDate"),
                season: SeasonId::from(22022),
                player_id: Some(PlayerId(601)), // Player Id for the player statistics edit
                team_id: TeamId(1),
                team_abbr: TeamAbbreviation::from_str("BRK")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(40)), // Changed PTS value for player statistics
                    (StatColumn::AST, Value::from(25)), // Changed AST value for player statistics
                ]),
            },
            Edit {
                game_id: GameId(4),
                game_date: GameDate::ymd(2022, 3, 30).expect("Failed to create GameDate"),
                season: SeasonId::from(22022),
                player_id: None,
                team_id: TeamId(2),
                team_abbr: TeamAbbreviation::from_str("TOR")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(160)), // Changed PTS value for team statistics
                    (StatColumn::REB, Value::from(45)),  // Changed REB value for team statistics
                ]),
            },
            Edit {
                game_id: GameId(4),
                game_date: GameDate::ymd(2022, 6, 1).expect("Failed to create GameDate"),
                season: SeasonId::from(22022),
                player_id: Some(PlayerId(501)), // Player Id for the player statistics edit
                team_id: TeamId(2),
                team_abbr: TeamAbbreviation::from_str("TOR")
                    .expect("Failed to create TeamAbbreviation"),
                delete: false,
                corrections: HashMap::from([
                    (StatColumn::PTS, Value::from(30)), // Changed PTS value for player statistics
                    (StatColumn::AST, Value::from(15)), // Changed AST value for player statistics
                ]),
            },
        ];

        assert!(edits.is_sorted())
    }
}
