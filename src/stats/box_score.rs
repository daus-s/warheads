use crate::format::box_score_formatter::format_statistical_box_score;
use crate::ml::vector::Vector;
use crate::stats::nba_schema::*;
use crate::stats::stat_column::StatColumn;
use crate::types::{
    Assists, Blocks, DefensiveRebounds, FantasyPoints, FieldGoalAttempts, FieldGoalMakes,
    FreeThrowAttempts, FreeThrowMakes, GameResult, Minutes, OffensiveRebounds, PersonalFouls,
    PlusMinus, Points, Rebounds, Steals, ThreePointAttempts, ThreePointMakes, Turnovers,
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};
use wincode::{SchemaRead, SchemaWrite};

#[derive(Clone, Debug, Serialize, Deserialize, Builder, PartialEq, Eq, SchemaWrite, SchemaRead)]
pub struct BoxScore {
    wl: GameResult,

    min: Minutes,

    fgm: FieldGoalMakes,
    fga: FieldGoalAttempts,

    fg3m: ThreePointMakes,
    fg3a: ThreePointAttempts,

    ftm: FreeThrowMakes,
    fta: FreeThrowAttempts,

    oreb: OffensiveRebounds,
    dreb: DefensiveRebounds,
    reb: Rebounds,

    ast: Assists,

    stl: Steals,

    blk: Blocks,

    tov: Turnovers,

    pf: PersonalFouls, //personal fouls
    pts: Points,

    //advanced stats
    plus_minus: PlusMinus,
}

impl BoxScore {
    fn fingerprint(&self) -> u32 {
        let bit = |present: bool, pos: u32| if present { 1 << pos } else { 0 };

        bit(true, MIN_BIT)
            | bit(true, FGM_BIT)
            | bit(self.fga().0.is_some(), FGA_BIT)
            | bit(self.fg3m().0.is_some(), FG3M_BIT)
            | bit(self.fg3a().0.is_some(), FG3A_BIT)
            | bit(true, FTM_BIT)
            | bit(self.fta().0.is_some(), FTA_BIT)
            | bit(self.oreb().0.is_some(), OREB_BIT)
            | bit(self.dreb().0.is_some(), DREB_BIT)
            | bit(self.reb().0.is_some(), REB_BIT)
            | bit(self.ast().0.is_some(), AST_BIT)
            | bit(self.stl().0.is_some(), STL_BIT)
            | bit(self.blk().0.is_some(), BLK_BIT)
            | bit(self.tov().0.is_some(), TOV_BIT)
            | bit(true, PF_BIT)
            | bit(true, PTS_BIT)
            | bit(self.plus_minus().0.is_some(), PLUS_MINUS_BIT)
            | bit(true, WL_BIT)
    }

    pub(super) fn schema(&self) -> Option<NBASchema> {
        match self.fingerprint().try_into() {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }

    /// document this function and test please
    pub fn calculate_fantasy(&self) -> FantasyPoints {
        // three pointers
        let fg3m = self.fg3m.0.unwrap_or(0); // if the three pointer doesn't exist
        let fg3a = self.fg3a.0.unwrap_or(0); // you took 0 3s

        // field goals
        let fgm = self.fgm.0;
        let fga = match self.fga.0 {
            Some(u) => u,
            None => return FantasyPoints(None),
        };

        // free throws
        let ftm = self.ftm.0;
        let fta = match self.fta.0 {
            Some(u) => u,
            None => return FantasyPoints(None),
        };

        // rebounds
        let reb = match self.reb.0 {
            Some(u) => u,
            None => return FantasyPoints(None),
        };

        // assists
        let ast = match self.ast.0 {
            Some(u) => u,
            None => return FantasyPoints(None),
        };

        // blocks
        let blk = match self.blk.0 {
            Some(u) => u,
            None => return FantasyPoints(None),
        };

        // steals
        let stl = match self.stl.0 {
            Some(u) => u,
            None => return FantasyPoints(None),
        };

        //turnovers
        let tov = match self.tov.0 {
            Some(u) => u,
            None => return FantasyPoints(None),
        };

        let f = (4f32 * fg3m as f32)
            + (-1f32 * fg3a as f32)
            + (3f32 * fgm as f32)
            + (-1f32 * fga as f32)
            + (2f32 * ftm as f32)
            + (-1f32 * fta as f32)
            + (1.2f32 * reb as f32)
            + (1.5f32 * ast as f32)
            + (2f32 * blk as f32)
            + (2f32 * stl as f32)
            + (-1f32 * tov as f32);

        FantasyPoints(Some(f))
    }

    pub fn try_set_col(&mut self, col: &StatColumn, val: &Value) -> Result<(), StatColumn> {
        match col {
            // required u-ints
            StatColumn::MIN => {
                if let Some(u) = val.as_u64() {
                    self.min = Minutes(u as u8);

                    Ok(())
                } else {
                    Err(StatColumn::MIN)
                }
            }
            StatColumn::FGM => {
                if let Some(u) = val.as_u64() {
                    self.fgm = FieldGoalMakes(u as u8);

                    Ok(())
                } else {
                    Err(StatColumn::FGM)
                }
            }
            StatColumn::FTM => {
                if let Some(u) = val.as_u64() {
                    self.ftm = FreeThrowMakes(u as u8);

                    Ok(())
                } else {
                    Err(StatColumn::FTM)
                }
            }
            StatColumn::PTS => {
                if let Some(u) = val.as_u64() {
                    self.pts = Points(u as u8);

                    Ok(())
                } else {
                    Err(StatColumn::PTS)
                }
            }
            StatColumn::PF => {
                if let Some(u) = val.as_u64() {
                    self.pf = PersonalFouls(u as u8);

                    Ok(())
                } else {
                    Err(StatColumn::PF)
                }
            }
            StatColumn::FGA => {
                if let Some(u) = val.as_u64() {
                    self.fga = FieldGoalAttempts(Some(u as u8));

                    Ok(())
                } else if let Some(_) = val.as_null() {
                    self.plus_minus = PlusMinus(None);

                    Ok(())
                } else {
                    Err(StatColumn::PLUS_MINUS)
                }
            }
            StatColumn::FG3M => {
                if let Some(u) = val.as_u64() {
                    self.fg3m = ThreePointMakes(Some(u as u8));

                    Ok(())
                } else if let Some(_) = val.as_null() {
                    self.fg3m = ThreePointMakes(None);

                    Ok(())
                } else {
                    Err(StatColumn::FG3M)
                }
            }
            StatColumn::FG3A => {
                if let Some(u) = val.as_u64() {
                    self.fg3a = ThreePointAttempts(Some(u as u8));

                    Ok(())
                } else if let Some(_) = val.as_null() {
                    self.fg3a = ThreePointAttempts(None);

                    Ok(())
                } else {
                    Err(StatColumn::FG3A)
                }
            }
            StatColumn::FTA => {
                if let Some(u) = val.as_u64() {
                    self.fta = FreeThrowAttempts(Some(u as u8));

                    Ok(())
                } else if let Some(_) = val.as_null() {
                    self.fta = FreeThrowAttempts(None);

                    Ok(())
                } else {
                    Err(StatColumn::FTA)
                }
            }
            StatColumn::OREB => {
                if let Some(u) = val.as_u64() {
                    self.oreb = OffensiveRebounds(Some(u as u8));

                    Ok(())
                } else if let Some(_) = val.as_null() {
                    self.oreb = OffensiveRebounds(None);

                    Ok(())
                } else {
                    Err(StatColumn::OREB)
                }
            }
            StatColumn::DREB => {
                if let Some(u) = val.as_u64() {
                    self.dreb = DefensiveRebounds(Some(u as u8));

                    Ok(())
                } else if let Some(_) = val.as_null() {
                    self.dreb = DefensiveRebounds(None);

                    Ok(())
                } else {
                    Err(StatColumn::DREB)
                }
            }
            StatColumn::REB => {
                if let Some(u) = val.as_u64() {
                    self.reb = Rebounds(Some(u as u8));

                    Ok(())
                } else if let Some(_) = val.as_null() {
                    self.reb = Rebounds(None);

                    Ok(())
                } else {
                    Err(StatColumn::REB)
                }
            }
            StatColumn::AST => {
                if let Some(u) = val.as_u64() {
                    self.ast = Assists(Some(u as u8));

                    Ok(())
                } else if let Some(_) = val.as_null() {
                    self.ast = Assists(None);

                    Ok(())
                } else {
                    Err(StatColumn::AST)
                }
            }
            StatColumn::STL => {
                if let Some(u) = val.as_u64() {
                    self.stl = Steals(Some(u as u8));

                    Ok(())
                } else if let Some(_) = val.as_null() {
                    self.stl = Steals(None);

                    Ok(())
                } else {
                    Err(StatColumn::STL)
                }
            }
            StatColumn::BLK => {
                if let Some(u) = val.as_u64() {
                    self.blk = Blocks(Some(u as u8));

                    Ok(())
                } else if let Some(_) = val.as_null() {
                    self.blk = Blocks(None);

                    Ok(())
                } else {
                    Err(StatColumn::BLK)
                }
            }
            StatColumn::TOV => {
                if let Some(u) = val.as_u64() {
                    self.tov = Turnovers(Some(u as u8));

                    Ok(())
                } else if let Some(_) = val.as_null() {
                    self.tov = Turnovers(None);

                    Ok(())
                } else {
                    Err(StatColumn::TOV)
                }
            }

            // ints
            StatColumn::PLUS_MINUS => {
                if let Some(i) = val.as_i64() {
                    self.plus_minus = PlusMinus(Some(i as i16));

                    Ok(())
                } else if let Some(_) = val.as_null() {
                    self.plus_minus = PlusMinus(None);

                    Ok(())
                } else {
                    Err(StatColumn::PLUS_MINUS)
                }
            }

            // special parsing case
            StatColumn::WL => self.try_set_wl(val).map_err(|_| StatColumn::WL),

            // not box score stat items
            StatColumn::SEASON_ID
            | StatColumn::PLAYER_ID
            | StatColumn::PLAYER_NAME
            | StatColumn::TEAM_ID
            | StatColumn::FANTASY_PTS
            | StatColumn::VIDEO_AVAILABLE
            | StatColumn::TEAM_ABBREVIATION
            | StatColumn::TEAM_NAME
            | StatColumn::GAME_ID
            | StatColumn::GAME_DATE
            | StatColumn::FG_PCT
            | StatColumn::FG3_PCT
            | StatColumn::FT_PCT
            | StatColumn::MATCHUP => {
                panic!("💀 illegal assignment operation on a box score. {col} is not a field of BoxScore")
            }
        }
    }

    fn try_set_wl(&mut self, value: &Value) -> Result<(), ()> {
        match value.as_str().unwrap_or("X").parse::<GameResult>() {
            Ok(res) => {
                self.wl = res;

                Ok(())
            }
            Err(e) => {
                eprintln!("❌ WL as JSON must be type String and in [W, L]: {e}");

                Err(())
            }
        }
    }

    //stats

    pub fn wl(&self) -> &GameResult {
        &self.wl
    }
    pub fn min(&self) -> &Minutes {
        &self.min
    }
    pub fn fgm(&self) -> &FieldGoalMakes {
        &self.fgm
    }
    pub fn fga(&self) -> &FieldGoalAttempts {
        &self.fga
    }
    pub fn fg3m(&self) -> &ThreePointMakes {
        &self.fg3m
    }
    pub fn fg3a(&self) -> &ThreePointAttempts {
        &self.fg3a
    }
    pub fn ftm(&self) -> &FreeThrowMakes {
        &self.ftm
    }
    pub fn fta(&self) -> &FreeThrowAttempts {
        &self.fta
    }
    pub fn oreb(&self) -> &OffensiveRebounds {
        &self.oreb
    }
    pub fn dreb(&self) -> &DefensiveRebounds {
        &self.dreb
    }
    pub fn reb(&self) -> &Rebounds {
        &self.reb
    }
    pub fn ast(&self) -> &Assists {
        &self.ast
    }
    pub fn stl(&self) -> &Steals {
        &self.stl
    }
    pub fn blk(&self) -> &Blocks {
        &self.blk
    }
    pub fn tov(&self) -> &Turnovers {
        &self.tov
    }
    pub fn pf(&self) -> &PersonalFouls {
        &self.pf
    }
    pub fn pts(&self) -> &Points {
        &self.pts
    }
    pub fn plus_minus(&self) -> &PlusMinus {
        &self.plus_minus
    }
}

impl Display for BoxScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_statistical_box_score(f, self)
    }
}

impl From<BoxScore> for Vector {
    fn from(boxscore: BoxScore) -> Self {
        //match on schema
        let vec = vec![
            boxscore.min().0 as f64,                     // 0
            boxscore.fgm().0 as f64,                     // 1
            boxscore.fga().0.unwrap_or(0) as f64,        // 2
            boxscore.fg3m().0.unwrap_or(0) as f64,       // 3
            boxscore.fg3a().0.unwrap_or(0) as f64,       // 4
            boxscore.ftm().0 as f64,                     // 5
            boxscore.fta().0.unwrap_or(0) as f64,        // 6
            boxscore.oreb().0.unwrap_or(0) as f64,       // 7
            boxscore.dreb().0.unwrap_or(0) as f64,       // 8
            boxscore.reb().0.unwrap_or(0) as f64,        // 9
            boxscore.ast().0.unwrap_or(0) as f64,        // 10
            boxscore.stl().0.unwrap_or(0) as f64,        // 11
            boxscore.blk().0.unwrap_or(0) as f64,        // 12
            boxscore.tov().0.unwrap_or(0) as f64,        // 13
            boxscore.pf().0 as f64,                      // 14
            boxscore.pts().0 as f64,                     // 15
            boxscore.plus_minus().0.unwrap_or(0) as f64, // 16
            match boxscore.wl() {
                GameResult::Win => 1.0,
                GameResult::Loss => 0.0,
                GameResult::Draw => 0.5,
            },
        ];

        Vector::from(vec)
    }
}

#[cfg(test)]
mod tests {
    use crate::{dapi::season_manager::nba_lifespan_period, stats::chronology::Chronology};

    #[test]
    fn test_boxscore_schemas() {
        for era in nba_lifespan_period() {
            let mut chrono = Chronology::new();

            chrono.load_era(era).expect("failed to load games in test");

            let games = chrono
                .games()
                .as_ref()
                .expect("failed to load games in test");

            for game in games {
                println!("{game}");
                println!("({:?})", game.home().box_score().schema())
            }
        }
    }
}
