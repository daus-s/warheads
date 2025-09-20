use crate::types::{GameDate, GameId, Matchup, PlayerName, TeamName};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct GameDisplay {
    matchup: Matchup,
    date: GameDate,

    /// ## player_name
    ///
    /// player_name contains an `Option<PlayerName>`. this will often be unwrapped
    /// and replaced if it doesn't exist with the team name.
    ///
    /// The optionality represents whether the statistics contained in this set are a
    /// team or player box score with `None` representing TeamBoxScores
    /// and `Some(player_name)` representing PlayerBoxScores.
    player_name: Option<PlayerName>,

    /// team abbreviation is the unique identifier for a team

    /// team name (full)
    team_name: TeamName,

    game_id: GameId,
}

impl GameDisplay {
    pub fn new(
        matchup: Matchup,
        date: GameDate,
        player_name: Option<PlayerName>,
        team_name: TeamName,
        game_id: GameId,
    ) -> Self {
        GameDisplay {
            matchup,
            date,
            player_name,
            team_name,
            game_id,
        }
    }

    pub fn display_name(&self) -> String {
        match &self.player_name {
            Some(n) => n.to_string(),
            None => self.team_name.to_string(),
        }
    }

    pub fn matchup(&self) -> Matchup {
        self.matchup.clone()
    }
}

impl Display for GameDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let matchup = format!("{}", self.matchup);
        // matching on name's existence is the same as checking Player vs. Team box score

        let url = format!("https://nba.com/game/{}", self.game_id);

        match &self.player_name {
            Some(s) => write!(
                f,
                "{} - {}\n{}:{}\n{url}",
                matchup,
                self.date,
                self.team_name.to_string(),
                s.to_string()
            ),
            None => write!(
                f,
                "{} - {}\n{}\n{url}",
                matchup,
                self.date,
                self.team_name.to_string()
            ),
        }
    }
}
