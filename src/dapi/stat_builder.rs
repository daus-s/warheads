use serde_json::Value;

use crate::dapi::from_value::FromValue;

use crate::stats::stat_column::StatColumn;
use crate::stats::stat_column::StatColumn::*;
use crate::stats::types::BoolInt;

use crate::types::{
    Assists, Blocks, DefensiveRebounds, FantasyPoints, FieldGoalAttempts, FieldGoalMakes,
    FieldGoalPercentage, FreeThrowAttempts, FreeThrowMakes, FreeThrowPercentage, GameDate, GameId,
    GameResult, Matchup, Minutes, OffensiveRebounds, PersonalFouls, PlayerId, PlayerName,
    PlusMinus, Points, Rebounds, SeasonId, Steals, TeamAbbreviation, TeamId, TeamName,
    ThreePointAttempts, ThreePointMakes, ThreePointPercentage, Turnovers,
};

impl FromValue for Value {
    fn season_id(&self) -> Result<SeasonId, StatColumn> {
        match SeasonId::try_from(self) {
            Ok(season_id) => Ok(season_id),
            Err(_) => Err(SEASON_ID),
        }
    }
    fn player_id(&self) -> Result<PlayerId, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(PlayerId(u)),
            None => Err(PLAYER_ID),
        }
    }

    fn player_name(&self) -> Result<PlayerName, StatColumn> {
        match self.as_str() {
            Some(s) => Ok(PlayerName(s.to_string())),
            None => Err(PLAYER_NAME),
        }
    }

    fn team_id(&self) -> Result<TeamId, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(TeamId(u)),
            None => Err(TEAM_ID),
        }
    }

    fn team_abbreviation(&self) -> Result<TeamAbbreviation, StatColumn> {
        match self.as_str() {
            Some(s) => Ok(TeamAbbreviation(s.to_string())),
            None => Err(TEAM_ABBREVIATION),
        }
    }

    fn team_name(&self) -> Result<TeamName, StatColumn> {
        match self.as_str() {
            Some(s) => Ok(TeamName(s.to_string())),
            None => Err(TEAM_NAME),
        }
    }

    fn game_id(&self) -> Result<GameId, StatColumn> {
        match self.as_str() {
            Some(s) => {
                if let Ok(id) = s.parse() {
                    Ok(GameId(id))
                } else {
                    Err(GAME_ID)
                }
            }
            None => Err(GAME_ID),
        }
    }

    fn game_date(&self) -> Result<GameDate, StatColumn> {
        match self.as_str() {
            Some(s) => match s.parse() {
                Ok(date) => Ok(date),
                Err(_) => Err(GAME_DATE),
            },
            None => Err(GAME_DATE),
        }
    }

    fn matchup(&self) -> Result<Matchup, StatColumn> {
        match self.as_str() {
            Some(s) => match s.parse() {
                Ok(matchup) => Ok(matchup),
                Err(_) => Err(MATCHUP),
            },
            None => Err(MATCHUP),
        }
    }

    fn game_result(&self) -> Result<GameResult, StatColumn> {
        match self.as_str() {
            Some(s) => match s.parse() {
                Ok(result) => Ok(result),
                Err(_) => Err(WL),
            },
            None => Err(WL),
        }
    }

    fn minutes(&self) -> Result<Minutes, StatColumn> {
        match self.as_u64() {
            Some(minutes) => Ok(Minutes(minutes as u8)),
            None => Err(WL),
        }
    }

    fn field_goal_makes(&self) -> Result<FieldGoalMakes, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(FieldGoalMakes(u as u8)),
            None => Err(FGM),
        }
    }

    fn field_goal_attempts(&self) -> Result<FieldGoalAttempts, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(FieldGoalAttempts(Some(u as u8))),
            None => Err(FGA),
        }
    }

    fn field_goal_percent(&self) -> Result<FieldGoalPercentage, StatColumn> {
        match self.as_f64() {
            Some(percent) => Ok(FieldGoalPercentage(Some(percent as f32))),
            None => Err(FG_PCT),
        }
    }

    fn three_point_makes(&self) -> Result<ThreePointMakes, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(ThreePointMakes(Some(u as u8))),
            None => Err(FG3M),
        }
    }

    fn three_point_attempts(&self) -> Result<ThreePointAttempts, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(ThreePointAttempts(Some(u as u8))),
            None => Err(FG3A),
        }
    }

    fn three_point_percent(&self) -> Result<ThreePointPercentage, StatColumn> {
        match self.as_f64() {
            Some(percent) => Ok(ThreePointPercentage(Some(percent as f32))),
            None => Err(FG3_PCT),
        }
    }

    fn free_throw_makes(&self) -> Result<FreeThrowMakes, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(FreeThrowMakes(u as u8)),
            None => Err(FTM),
        }
    }

    fn free_throw_attempts(&self) -> Result<FreeThrowAttempts, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(FreeThrowAttempts(Some(u as u8))),
            None => Err(FTA),
        }
    }

    fn free_throw_percent(&self) -> Result<FreeThrowPercentage, StatColumn> {
        match self.as_f64() {
            Some(percent) => Ok(FreeThrowPercentage(Some(percent as f32))),
            None => Err(FT_PCT),
        }
    }

    fn offensive_rebounds(&self) -> Result<OffensiveRebounds, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(OffensiveRebounds(Some(u as u8))),
            None => Err(OREB),
        }
    }

    fn defensive_rebounds(&self) -> Result<DefensiveRebounds, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(DefensiveRebounds(Some(u as u8))),
            None => Err(DREB),
        }
    }

    fn rebounds(&self) -> Result<Rebounds, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(Rebounds(Some(u as u8))),
            None => Err(REB),
        }
    }

    fn assists(&self) -> Result<Assists, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(Assists(Some(u as u8))),
            None => Err(AST),
        }
    }

    fn steals(&self) -> Result<Steals, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(Steals(Some(u as u8))),
            None => Err(STL),
        }
    }

    fn blocks(&self) -> Result<Blocks, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(Blocks(Some(u as u8))),
            None => Err(BLK),
        }
    }

    fn turnovers(&self) -> Result<Turnovers, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(Turnovers(Some(u as u8))),
            None => Err(TOV),
        }
    }

    fn personal_fouls(&self) -> Result<PersonalFouls, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(PersonalFouls(u as u8)),
            None => Err(PF),
        }
    }

    fn points(&self) -> Result<Points, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(Points(u as u8)),
            None => Err(PTS),
        }
    }

    fn plus_minus(&self) -> Result<PlusMinus, StatColumn> {
        match self.as_i64() {
            Some(i) => Ok(PlusMinus(Some(i as i16))),
            None => Err(PLUS_MINUS),
        }
    }

    fn fantasy_points(&self) -> Result<FantasyPoints, StatColumn> {
        match self.as_f64() {
            Some(f) => Ok(FantasyPoints(Some(f as f32))),
            None => Err(FANTASY_PTS),
        }
    }

    /// # video_available
    /// Returns whether the game has a video available.
    ///
    /// ## errors
    /// BoolInt::new() can panic if the integer provided is not in {0, 1}.
    fn video_available(&self) -> Result<BoolInt, StatColumn> {
        match self.as_u64() {
            Some(u) => Ok(BoolInt::from(u as u8)),
            None => Err(VIDEO_AVAILABLE),
        }
    }
}
