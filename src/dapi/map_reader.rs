use crate::dapi::parse::value_to_date;
use crate::stats::stat_column::StatColumn;
use crate::stats::stat_column::StatColumn::*;
use crate::stats::types::BoolInt;
use crate::types::*;
use serde_json::{Number, Value};
use std::collections::HashMap;
use std::str::FromStr;

impl MapReader for HashMap<StatColumn, Value> {
    fn season_id(&self) -> Result<SeasonId, StatColumn> {
        match self.get(&SEASON_ID) {
            Some(v) => match SeasonId::try_from(v) {
                Ok(season_id) => Ok(season_id),
                Err(_) => Err(SEASON_ID),
            },
            None => {
                eprintln!("\t⚠️  failed to get a SeasonId from the stat map.");

                Err(SEASON_ID)
            }
        }
    }
    fn player_id(&self) -> Result<PlayerId, StatColumn> {
        let value = match self.get(&PLAYER_ID) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a PlayerId from the stat map.");

                return Err(PLAYER_ID);
            }
        };

        let u = match value.as_u64() {
            Some(u) => u,
            None => {
                eprintln!("\t⚠️  PlayerId is not a JSON Number (u64).");

                return Err(PLAYER_ID);
            }
        };

        Ok(PlayerId(u))
    }

    fn player_name(&self) -> Result<PlayerName, StatColumn> {
        let value = match self.get(&PLAYER_NAME) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a PlayerName from the stat map.");

                return Err(PLAYER_NAME);
            }
        };

        let s = match value.as_str() {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  PlayerName is not a JSON String.");

                return Err(PLAYER_NAME);
            }
        };

        Ok(PlayerName(s.to_string()))
    }

    fn team_id(&self) -> Result<TeamId, StatColumn> {
        let value = match self.get(&TEAM_ID) {
            Some(v) => v,
            None => {
                eprintln!("\t⚠️  failed to get a TeamId from the stat map.");

                return Err(TEAM_ID);
            }
        };

        let u = match value.as_u64() {
            Some(u) => u,
            None => {
                eprintln!("\t⚠️  TeamId is not an unsigned JSON number.");

                return Err(TEAM_ID);
            }
        };

        Ok(TeamId(u))
    }

    fn team_abbreviation(&self) -> Result<TeamAbbreviation, StatColumn> {
        let value = match self.get(&TEAM_ABBREVIATION) {
            Some(v) => v,
            None => {
                eprintln!("\t⚠️  failed to get a TeamAbbreviation from the stat map.");

                return Err(TEAM_ABBREVIATION);
            }
        };

        let s = match value.as_str() {
            Some(s) => s,
            None => {
                eprintln!("\t⚠️  TeamAbbreviation is not a JSON String.");

                return Err(TEAM_ABBREVIATION);
            }
        };

        Ok(TeamAbbreviation(s.trim().to_string()))
    }

    fn team_name(&self) -> Result<TeamName, StatColumn> {
        let value = match self.get(&TEAM_NAME) {
            Some(v) => v,
            None => {
                eprintln!("\t⚠️  failed to get a TeamName from the stat map.");

                return Err(TEAM_NAME);
            }
        };

        match value {
            Value::Null => Err(TEAM_NAME),
            Value::String(s) => Ok(TeamName(s.to_owned())),
            _ => {
                eprintln!("\t⚠️  TeamName is not a JSON String.");

                Err(TEAM_NAME)
            }
        }
    }

    fn game_id(&self) -> Result<GameId, StatColumn> {
        let value = match self.get(&GAME_ID) {
            None => {
                eprintln!("\t⚠️  failed to get a GameId from the stat map.");

                return Err(GAME_ID);
            }
            Some(v) => v,
        };

        let s = match value.as_str() {
            Some(s) => s,
            None => {
                eprintln!("\t⚠️  GameId is not a JSON String.");

                return Err(GAME_ID);
            }
        };

        if let Ok(u) = s.parse::<u64>() {
            Ok(GameId(u))
        } else {
            Err(GAME_ID)
        }
    }

    fn game_date(&self) -> Result<GameDate, StatColumn> {
        let value = match self.get(&GAME_DATE) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a GameDate from the stat map.");

                return Err(GAME_DATE);
            }
        };

        let d = match value_to_date(value) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to parse GameDate from the JSON String.");

                return Err(GAME_DATE);
            }
        };

        Ok(GameDate(d))
    }

    fn matchup(&self) -> Result<Matchup, StatColumn> {
        let value = match self.get(&MATCHUP) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a MatchupString from the stat map.");

                return Err(MATCHUP);
            }
        };

        let s = match value.as_str() {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  MatchupString is not a JSON String.");

                return Err(MATCHUP);
            }
        };

        match s.parse::<Matchup>() {
            Ok(s) => Ok(s),
            Err(_) => {
                eprintln!("\t⚠️  failed to parse MatchupString from the JSON String.");

                Err(MATCHUP)
            }
        }
    }

    fn game_result(&self) -> Result<GameResult, StatColumn> {
        let value = match self.get(&WL) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a GameResult from the stat map.");

                return Err(WL);
            }
        };

        let s = match value.as_str() {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  GameResult is not a JSON String.");

                return Err(WL);
            }
        };

        match GameResult::from_str(s) {
            Ok(g) => Ok(g),
            Err(e) => {
                eprintln!("\t⚠️  failed to parse a GameResult from the JSON String: {e}");

                Err(WL)
            }
        }
    }

    fn minutes(&self) -> Result<Minutes, StatColumn> {
        let value = match self.get(&MIN) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a Minutes from the stat map.");

                return Err(MIN);
            }
        };

        match value.as_u64() {
            Some(u) => Ok(Minutes(u as u8)), // this cast is safe, the longest NBA game was 78 minutes. source:
            // https://www.guinnessworldrecords.com/world-records/428821-longest-nba-basketball-game
            None => {
                eprintln!("\t⚠️  Minutes is not a JSON Number.");

                Err(MIN)
            }
        }
    }

    fn field_goal_makes(&self) -> Result<FieldGoalMakes, StatColumn> {
        let value = match self.get(&FGM) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a FieldGoalMakes from the stat map.");

                return Err(FGM);
            }
        };

        match value {
            Value::Number(u) if u.is_u64() => Ok(FieldGoalMakes(u.as_u64().unwrap() as u8)),
            Value::Number(u) if !u.is_u64() => {
                eprintln!("\t⚠️  FieldGoalMakes is a number but not a unsigned int.");

                Err(FGM)
            }
            _ => {
                eprintln!("\t⚠️  FieldGoalMakes is not a JSON Number.");

                Err(FGM)
            }
        }
    }

    fn field_goal_attempts(&self) -> Result<FieldGoalAttempts, StatColumn> {
        let value = match self.get(&FGA) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a FieldGoalAttempts from the stat map.");

                return Err(FGA);
            }
        };

        match value {
            Value::Number(u) if u.is_u64() => {
                Ok(FieldGoalAttempts(Some(u.as_u64().unwrap() as u8)))
            }
            Value::Number(u) if !u.is_u64() => {
                eprintln!(
                    "\t⚠️  Could not parse an unsigned int from FieldGoalAttempts JSON Number."
                );

                Err(FGA)
            }
            Value::Null => {
                eprintln!("\t❕  FieldGoalAttempts is null and only permitted in special cases. ");

                Ok(FieldGoalAttempts(None))
            }
            _ => {
                eprintln!("\t⚠️  FieldGoalAttempts is null. Field goals are always recorded. ");

                Err(FGA)
            }
        }
    }

    fn field_goal_percent(&self) -> Result<FieldGoalPercentage, StatColumn> {
        let value = match self.get(&FG_PCT) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a FieldGoalPercentage from the stat map.");

                return Err(FG_PCT);
            }
        };

        match value {
            Value::Number(f) if f.is_f64() => {
                let percent = f.as_f64().unwrap() as f32;

                if percent >= 0f32 && percent <= 1f32 {
                    Ok(FieldGoalPercentage(Some(f.as_f64().unwrap() as f32)))
                } else {
                    eprintln!("\t⚠️  FieldGoalPercent ({percent}) is not bounded by the constraint: [0., 1.]");

                    Err(FG_PCT)
                }
            }
            Value::Number(f) if !f.is_f64() => {
                eprintln!("\t⚠️  FieldGoalPercent is a JSON Number but not a float.");

                Err(FG_PCT)
            }
            Value::Null => Ok(FieldGoalPercentage(None)),
            _ => {
                eprintln!("\t⚠️  FieldGoalPercent is not a Number (u8) or Null (null).");

                Err(FG_PCT)
            }
        }
    }

    fn three_point_makes(&self) -> Result<ThreePointMakes, StatColumn> {
        let value = match self.get(&FG3M) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a ThreePointMakes from the stat map.");

                return Err(FG3M);
            }
        };

        match value {
            Value::Number(u) if u.is_u64() => Ok(ThreePointMakes(Some(u.as_u64().unwrap() as u8))),
            Value::Number(u) if !u.is_u64() => {
                eprintln!("\t⚠️  ThreePointMakes is a Number but not unsigned or integral: {u}");

                Err(FG3M)
            }
            Value::Null => Ok(ThreePointMakes(None)),
            _ => {
                eprintln!("\t⚠️  ThreePointMakes is not a Number (u8) or Null (null).");

                Err(FG3M)
            }
        }
    }

    fn three_point_attempts(&self) -> Result<ThreePointAttempts, StatColumn> {
        let value = match self.get(&FG3A) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a ThreePointAttempts from the stat map.");

                return Err(FG3A);
            }
        };

        match value {
            Value::Number(u) if u.is_u64() => {
                Ok(ThreePointAttempts(Some(u.as_u64().unwrap() as u8)))
            }
            Value::Number(u) if !u.is_u64() => {
                eprintln!("\t⚠️  ThreePointAttempts is a Number but not unsigned or integral: {u}");

                Err(FG3A)
            }
            Value::Null => Ok(ThreePointAttempts(None)),
            _ => {
                eprintln!("\t⚠️  ThreePointAttempts is not a Number (u8) or Null (null).");

                Err(FG3A)
            }
        }
    }

    fn three_point_percent(&self) -> Result<ThreePointPercentage, StatColumn> {
        let value = match self.get(&FG3_PCT) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a ThreePointPercentage from the stat map.");

                return Err(FG3_PCT);
            }
        };

        match value {
            Value::Number(f) if f.is_f64() => {
                let percent = f.as_f64().unwrap() as f32;

                if percent >= 0f32 && percent <= 1f32 {
                    Ok(ThreePointPercentage(Some(f.as_f64().unwrap() as f32)))
                } else {
                    eprintln!("\t⚠️  ThreePointPercentage ({percent}) is not bounded by the constraint: [0., 1.]");

                    Err(FG3_PCT)
                }
            }
            Value::Number(f) if !f.is_f64() => {
                eprintln!("\t⚠️  ThreePointPercentage is a JSON Number but not a float.");

                Err(FG3_PCT)
            }
            Value::Null => Ok(ThreePointPercentage(None)),
            _ => {
                eprintln!("\t⚠️  ThreePointPercentage is not a Number (u8) or Null (null).");

                Err(FG3_PCT)
            }
        }
    }

    fn free_throw_makes(&self) -> Result<FreeThrowMakes, StatColumn> {
        let value = match self.get(&FTM) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a FreeThrowMakes from the stat map.");

                return Err(FTM);
            }
        };

        match value {
            Value::Number(u) if u.is_u64() => Ok(FreeThrowMakes(u.as_u64().unwrap() as u8)),
            Value::Number(u) if !u.is_u64() => {
                eprintln!("\t⚠️  FreeThrowMakes is a Number but not unsigned or integral: {u}");

                Err(FTM)
            }
            Value::Null => {
                eprintln!(
                    "\t⚠️  FreeThrowMakes is null when it is a non-optional field in the JSON."
                );

                Err(FTM)
            }
            _ => {
                eprintln!("\t⚠️  FreeThrowMakes is not a Number (u8) or Null (null).");

                Err(FTM)
            }
        }
    }

    fn free_throw_attempts(&self) -> Result<FreeThrowAttempts, StatColumn> {
        let value = match self.get(&FTA) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a FreeThrowAttempts from the stat map.");

                return Err(FTA);
            }
        };

        match value {
            Value::Number(u) if u.is_u64() => Ok(FreeThrowAttempts(u.as_u64().map(|u| u as u8))),
            Value::Null => Ok(FreeThrowAttempts(None)),
            Value::Number(u) if !u.is_u64() => {
                eprintln!("\t⚠️  FreeThrowAttempts is a Number but not unsigned or integral: {u}");

                Err(FTA)
            }
            _ => {
                eprintln!("\t⚠️  FreeThrowAttempts is the wrong type.");

                Err(FTA)
            }
        }
    }

    fn free_throw_percent(&self) -> Result<FreeThrowPercentage, StatColumn> {
        let value = match self.get(&FT_PCT) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a FreeThrowPercentage from the stat map.");

                return Err(FT_PCT);
            }
        };

        match value {
            Value::Number(f) if f.is_f64() => {
                let percent = f.as_f64().unwrap() as f32;

                if percent >= 0f32 && percent <= 1f32 {
                    Ok(FreeThrowPercentage(Some(f.as_f64().unwrap() as f32)))
                } else {
                    eprintln!("\t⚠️  FreeThrowPercentage ({percent}) is not bounded by the constraint: [0., 1.]");

                    Err(FT_PCT)
                }
            }
            Value::Number(f) if !f.is_f64() => {
                eprintln!("\t⚠️  FreeThrowPercentage is a JSON Number but not a float.");

                Err(FT_PCT)
            }
            Value::Null => Ok(FreeThrowPercentage(None)),
            _ => {
                eprintln!("\t⚠️  FreeThrowPercentage is not a Number (u8) or Null (null).");

                Err(FT_PCT)
            }
        }
    }

    fn offensive_rebounds(&self) -> Result<OffensiveRebounds, StatColumn> {
        let value = match self.get(&OREB) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a OffensiveRebounds from the stat map.");

                return Err(OREB);
            }
        };

        match value {
            Value::Null => Ok(OffensiveRebounds(None)),
            Value::Number(u) if u.is_u64() => {
                Ok(OffensiveRebounds(Some(u.as_u64().unwrap() as u8)))
            }
            Value::Number(u) if !u.is_u64() => {
                eprintln!("\t⚠️  OffensiveRebounds is a Number but not unsigned or integral.");

                Err(OREB)
            }
            _ => {
                eprintln!("\t⚠️  OffensiveRebounds is not a Number (u8) or Null (null).");

                Err(OREB)
            }
        }
    }

    fn defensive_rebounds(&self) -> Result<DefensiveRebounds, StatColumn> {
        let value = match self.get(&DREB) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a DefensiveRebounds from the stat map.");

                return Err(DREB);
            }
        };

        match value {
            Value::Null => Ok(DefensiveRebounds(None)),
            Value::Number(u) if u.is_u64() => {
                Ok(DefensiveRebounds(Some(u.as_u64().unwrap() as u8)))
            }
            Value::Number(u) if !u.is_u64() => {
                eprintln!("\t⚠️  DefensiveRebounds is a Number but not unsigned or integral.");

                Err(OREB)
            }
            _ => {
                eprintln!("\t⚠️  DefensiveRebounds is not a Number (u8) or Null (null).");

                Err(OREB)
            }
        }
    }

    fn rebounds(&self) -> Result<Rebounds, StatColumn> {
        let value = match self.get(&REB) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a Rebounds from the stat map.");

                return Err(REB);
            }
        };

        match value {
            Value::Null => Ok(Rebounds(None)),
            Value::Number(u) if u.is_u64() => Ok(Rebounds(Some(u.as_u64().unwrap() as u8))),
            Value::Number(u) if !u.is_u64() => {
                eprintln!("\t⚠️  Rebounds is a Number but not unsigned or integral.");

                Err(REB)
            }
            _ => {
                eprintln!("\t⚠️  Rebounds is not a Number (u8) or Null (null).");

                Err(REB)
            }
        }
    }

    fn assists(&self) -> Result<Assists, StatColumn> {
        let value = match self.get(&AST) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a Assists from the stat map.");

                return Err(AST);
            }
        };

        match value {
            Value::Null => Ok(Assists(None)),
            Value::Number(u) if u.is_u64() => Ok(Assists(Some(u.as_u64().unwrap() as u8))),
            Value::Number(u) if !u.is_u64() => {
                eprintln!("\t⚠️  Assists is a Number but not unsigned or integral.");

                Err(AST)
            }
            _ => {
                eprintln!("\t⚠️  Assists is not a Number (u8) or Null (null).");

                Err(AST)
            }
        }
    }

    fn steals(&self) -> Result<Steals, StatColumn> {
        let value = match self.get(&STL) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a Steals from the stat map.");

                return Err(STL);
            }
        };

        match value {
            Value::Null => Ok(Steals(None)),
            Value::Number(u) if u.is_u64() => Ok(Steals(Some(u.as_u64().unwrap() as u8))),
            Value::Number(u) if !u.is_u64() => {
                eprintln!("\t⚠️  Steals is a Number but not unsigned or integral.");

                Err(STL)
            }
            _ => {
                eprintln!("\t⚠️  Steals is not a Number (u8) or Null (null).");

                Err(STL)
            }
        }
    }

    fn blocks(&self) -> Result<Blocks, StatColumn> {
        let value = match self.get(&BLK) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a Blocks from the stat map.");

                return Err(BLK);
            }
        };

        match value {
            Value::Null => Ok(Blocks(None)),
            Value::Number(u) if u.is_u64() => Ok(Blocks(Some(u.as_u64().unwrap() as u8))),
            Value::Number(u) if !u.is_u64() => {
                eprintln!("\t⚠️  Blocks is a Number but not unsigned or integral.");

                Err(BLK)
            }
            _ => {
                eprintln!("\t⚠️  Blocks is not a Number (u8) or Null (null).");

                Err(BLK)
            }
        }
    }

    fn turnovers(&self) -> Result<Turnovers, StatColumn> {
        let value = match self.get(&TOV) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a Turnovers from the stat map.");

                return Err(TOV);
            }
        };

        match value {
            Value::Null => Ok(Turnovers(None)),
            Value::Number(u) if u.is_u64() => Ok(Turnovers(Some(u.as_u64().unwrap() as u8))),
            Value::Number(u) if !u.is_u64() => {
                eprintln!("\t⚠️  Turnovers is a Number but not unsigned or integral.");

                Err(TOV)
            }
            _ => {
                eprintln!("\t⚠️  Turnovers is not a Number (u8) or Null (null).");

                Err(TOV)
            }
        }
    }

    fn personal_fouls(&self) -> Result<PersonalFouls, StatColumn> {
        let value = match self.get(&PF) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a PersonalFouls from the stat map.");

                return Err(PF);
            }
        };

        match value {
            Value::Number(u) if u.is_u64() => Ok(PersonalFouls(u.as_u64().unwrap() as u8)),
            Value::Number(u) if !u.is_u64() => {
                eprintln!("\t⚠️  PersonalFouls is a Number but not unsigned or integral.");

                Err(PF)
            }
            Value::Null => {
                eprintln!("\t❕ PersonalFouls is a Null, but not typically permitted to be Null. Its value has been defaulted to 0.");

                Ok(PersonalFouls(0))
            }
            _ => {
                eprintln!("\t⚠️  PersonalFouls is not a Number when it is a non-optional field. (special null cases are permitted but emit warnings)");

                Err(PF)
            }
        }
    }

    fn points(&self) -> Result<Points, StatColumn> {
        let value = match self.get(&PTS) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a Points from the stat map.");

                return Err(PTS);
            }
        };

        match value {
            Value::Number(u) if u.is_u64() => Ok(Points(u.as_u64().unwrap() as u8)),
            Value::Number(u) if !u.is_u64() => {
                eprintln!("\t⚠️  Points is a Number but not unsigned or integral.");

                Err(PTS)
            }
            _ => {
                eprintln!("\t⚠️  Points is not a Number when it is a non-optional field.");

                Err(PTS)
            }
        }
    }

    fn plus_minus(&self) -> Result<PlusMinus, StatColumn> {
        let value = match self.get(&PLUS_MINUS) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a Points from the stat map.");

                return Err(PLUS_MINUS);
            }
        };

        match value {
            Value::Number(i) if i.is_i64() => Ok(PlusMinus(Some(i.as_i64().unwrap() as i16))),
            Value::Null => Ok(PlusMinus(None)),
            Value::Number(i) if !i.is_i64() => {
                eprintln!("\t⚠️  PlusMinus is a Number but not integral.");

                Err(PLUS_MINUS)
            }
            _ => {
                eprintln!("\t⚠️  PlusMinus is not a Number (i64) or Null (null).");

                Err(PLUS_MINUS)
            }
        }
    }

    //we can --delete-- IGNORE this function because we don't ask the map about fantasy points
    // as it is not a member of the

    fn fantasy_points(&self) -> Result<FantasyPoints, StatColumn> {
        let value = match self.get(&FANTASY_PTS) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a FantasyPoints from the stat map.");

                return Err(FANTASY_PTS);
            }
        };

        match value {
            Value::Number(f) if f.is_f64() => Ok(FantasyPoints(Some(f.as_f64().unwrap() as f32))),
            Value::Number(f) if !f.is_f64() => {
                eprintln!(
                    "\t⚠️  FantasyPoints is a Number but not a floating point (or subset of *f*)."
                );

                Err(FANTASY_PTS)
            }
            Value::Null => Ok(FantasyPoints(None)),
            _ => {
                eprintln!("\t⚠️  FantasyPoints is not a Number (f64) or Null (null).");

                Err(FANTASY_PTS)
            }
        }
    }

    fn video_available(&self) -> Result<BoolInt, StatColumn> {
        let value = match self.get(&VIDEO_AVAILABLE) {
            Some(x) => x,
            None => {
                eprintln!("\t⚠️  failed to get a VideoAvailable from the stat map.");

                return Err(VIDEO_AVAILABLE);
            }
        };

        fn number_to_bool(num: &Number) -> Option<BoolInt> {
            if !num.is_u64() {
                return None;
            }

            let u = num.as_u64().unwrap();

            if u == 0 || u == 1 {
                Some(BoolInt(u as u8))
            } else {
                None
            }
        }

        match value {
            Value::Number(i) => match number_to_bool(i) {
                Some(b) => Ok(b),
                None => Err(VIDEO_AVAILABLE),
            },
            _ => {
                eprintln!("\t⚠️  VideoAvailable is not Number or in the domain x∈{{0,1}}");

                Err(VIDEO_AVAILABLE)
            }
        }
    }
}

///
/// The MapReader trait describes the behavior of a `HashMap<StatColumn, Value>`. everything not
/// part of the identity returns a result which may contain a nested option if the field is optional,
///
/// If the field fails to be parsed or not read for any reason it will return an Err with internal type
/// StatColumn, specifying the column that will need to be corrected via the corrections API.
///
/// The identity functions will panic if it fails to generate a correct identity.
///
pub trait MapReader {
    fn season_id(&self) -> Result<SeasonId, StatColumn>; //identity
    fn player_id(&self) -> Result<PlayerId, StatColumn>; //identity
    fn player_name(&self) -> Result<PlayerName, StatColumn>;
    fn team_id(&self) -> Result<TeamId, StatColumn>; //identity
    fn team_abbreviation(&self) -> Result<TeamAbbreviation, StatColumn>; //identity
    fn team_name(&self) -> Result<TeamName, StatColumn>;
    fn game_id(&self) -> Result<GameId, StatColumn>; //identity
    fn game_date(&self) -> Result<GameDate, StatColumn>;
    fn matchup(&self) -> Result<Matchup, StatColumn>;
    fn game_result(&self) -> Result<GameResult, StatColumn>;
    fn minutes(&self) -> Result<Minutes, StatColumn>;
    fn field_goal_makes(&self) -> Result<FieldGoalMakes, StatColumn>;
    fn field_goal_attempts(&self) -> Result<FieldGoalAttempts, StatColumn>;
    fn field_goal_percent(&self) -> Result<FieldGoalPercentage, StatColumn>;
    fn three_point_makes(&self) -> Result<ThreePointMakes, StatColumn>;
    fn three_point_attempts(&self) -> Result<ThreePointAttempts, StatColumn>;
    fn three_point_percent(&self) -> Result<ThreePointPercentage, StatColumn>;
    fn free_throw_makes(&self) -> Result<FreeThrowMakes, StatColumn>;
    fn free_throw_attempts(&self) -> Result<FreeThrowAttempts, StatColumn>;
    fn free_throw_percent(&self) -> Result<FreeThrowPercentage, StatColumn>;
    fn offensive_rebounds(&self) -> Result<OffensiveRebounds, StatColumn>;
    fn defensive_rebounds(&self) -> Result<DefensiveRebounds, StatColumn>;
    fn rebounds(&self) -> Result<Rebounds, StatColumn>;
    fn assists(&self) -> Result<Assists, StatColumn>;
    fn steals(&self) -> Result<Steals, StatColumn>;
    fn blocks(&self) -> Result<Blocks, StatColumn>;
    fn turnovers(&self) -> Result<Turnovers, StatColumn>;
    fn personal_fouls(&self) -> Result<PersonalFouls, StatColumn>;
    fn points(&self) -> Result<Points, StatColumn>;
    fn plus_minus(&self) -> Result<PlusMinus, StatColumn>;
    fn fantasy_points(&self) -> Result<FantasyPoints, StatColumn>;
    fn video_available(&self) -> Result<BoolInt, StatColumn>;
}
