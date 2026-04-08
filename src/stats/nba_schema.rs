pub(super) const MIN_BIT: u32 = 0;
pub(super) const FGM_BIT: u32 = 1;
pub(super) const FGA_BIT: u32 = 2;
pub(super) const FG3M_BIT: u32 = 3;
pub(super) const FG3A_BIT: u32 = 4;
pub(super) const FTM_BIT: u32 = 5;
pub(super) const FTA_BIT: u32 = 6;
pub(super) const OREB_BIT: u32 = 7;
pub(super) const DREB_BIT: u32 = 8;
pub(super) const REB_BIT: u32 = 9;
pub(super) const AST_BIT: u32 = 10;
pub(super) const STL_BIT: u32 = 11;
pub(super) const BLK_BIT: u32 = 12;
pub(super) const TOV_BIT: u32 = 13;
pub(super) const PF_BIT: u32 = 14;
pub(super) const PTS_BIT: u32 = 15;
pub(super) const PLUS_MINUS_BIT: u32 = 16;
pub(super) const WL_BIT: u32 = 17;

pub const INITIAL_NBA_SCHEMA: u32 = (1 << MIN_BIT)
    | (1 << FGM_BIT)
    | (1 << FGA_BIT)
    | (1 << FTM_BIT)
    | (1 << FTA_BIT)
    | (1 << REB_BIT)
    | (1 << PF_BIT)
    | (1 << PTS_BIT);

pub const INITIAL_NBA_NO_FGA: u32 = INITIAL_NBA_SCHEMA - (1 << FGA_BIT);

pub const INITIAL_NBA_NO_FTA: u32 = INITIAL_NBA_SCHEMA - (1 << FTA_BIT);

pub const INITIAL_NBA_NO_ATT: u32 = INITIAL_NBA_SCHEMA - (1 << FGA_BIT) - (1 << FTA_BIT);

/// # Schema
/// this enum defines the type of data provided as different eras of nba have yielded different recorded
/// schemas. (e.g. early NBA stats only included points, pfs and makes but not attempts.)
#[derive(Debug, Clone)]
pub(super) enum NBASchema {
    //1946
    InitialNBASchema,
    InitialMinusFGA,
    InitialMinusFTA,
    InitialMinusATT,
    //1947
    EarlyNBASansFGA,
    EarlyNBASansFTA,
    EarlyNBASansFGAFTA,
    Pre3PtEra,     //79-
    PlayByPlayEra, //96-present
}

impl From<u32> for NBASchema {
    fn from(value: u32) -> Self {
        match value {
            INITIAL_NBA_SCHEMA => Self::InitialNBASchema,
            INITIAL_NBA_NO_FGA => Self::InitialMinusFGA,
            INITIAL_NBA_NO_FTA => Self::InitialMinusFTA,
            INITIAL_NBA_NO_ATT => Self::InitialMinusATT,
            x => panic!("💀 unknown variant for NBA schema: {:014b}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        dapi::season_manager::nba_lifespan_period,
        stats::{chronology::Chronology, season_period::SeasonPeriod},
    };

    #[test]
    fn calculate_schema_from_year() {
        let mut chrono = Chronology::new();
        let columns = vec![
            "min", "fgm", "fga", "fg3m", "fg3a", "ftm", "fta", "oreb", "dreb", "reb", "ast", "stl",
            "blk", "tov", "pf", "pts", "+-",
        ];
        let col_width = 5;
        println!(
            "+------+{}+",
            (0..columns.len())
                .map(|_| "-".repeat(col_width))
                .collect::<Vec<_>>()
                .join("+")
        );
        for era in nba_lifespan_period() {
            chrono.load_era(era).expect("failed to load chronology");
            let games = chrono.games().as_ref().unwrap();
            let mut schema_map = HashMap::<u32, u32>::new();
            for game in games {
                let boxscore = game.home().box_score();
                let k: u32 = 0
                        | (1 << 0)  // min (always present)
                        | (1 << 1)  // fgm (always present)
                        | (if boxscore.fga().0.is_some() { 1 } else { 0 } << 2)
                        | (if boxscore.fg3m().0.is_some() { 1 } else { 0 } << 3)
                        | (if boxscore.fg3a().0.is_some() { 1 } else { 0 } << 4)
                        | (1 << 5)  // ftm (always present)
                        | (if boxscore.fta().0.is_some() { 1 } else { 0 } << 6)
                        | (if boxscore.oreb().0.is_some() { 1 } else { 0 } << 7)
                        | (if boxscore.dreb().0.is_some() { 1 } else { 0 } << 8)
                        | (if boxscore.reb().0.is_some() { 1 } else { 0 } << 9)
                        | (if boxscore.ast().0.is_some() { 1 } else { 0 } << 10)
                        | (if boxscore.stl().0.is_some() { 1 } else { 0 } << 11)
                        | (if boxscore.blk().0.is_some() { 1 } else { 0 } << 12)
                        | (if boxscore.tov().0.is_some() { 1 } else { 0 } << 13)
                        | (1 << 14)  // pf (always present)
                        | (1 << 15)  // pts (always present)
                        | (if boxscore.plus_minus().0.is_some() { 1 } else { 0 } << 16);
                schema_map.entry(k).and_modify(|v| *v += 1).or_insert(1);
            }

            let season_id = era;
            let schema_count: isize = (schema_map.len() + 2) as isize;
            let mid_row = schema_count / 2;

            println!(
                "|      |{}",
                columns
                    .iter()
                    .map(|col| format!("{:^width$}|", col, width = col_width))
                    .collect::<Vec<_>>()
                    .join("")
            );

            println!(
                "|{}|{}+",
                if mid_row - 2 == -1 {
                    format!("{:^6}", era.year())
                } else {
                    "      ".to_string()
                },
                (0..columns.len())
                    .map(|_| "-".repeat(col_width))
                    .collect::<Vec<_>>()
                    .join("+")
            );

            for (idx, (schema, _count)) in schema_map.iter().enumerate() {
                let idx = idx as isize;

                let prefix = if idx == mid_row - 2 {
                    format!("|{:^6}|", season_id.year())
                } else if idx == mid_row - 1 {
                    match season_id.period() {
                        SeasonPeriod::PreSeason => format!("|{:^6}|", "PRE"),
                        SeasonPeriod::RegularSeason => format!("|{:^6}|", "REG"),
                        SeasonPeriod::NBACup => format!("|{:^6}|", "CUP"),
                        SeasonPeriod::PlayIn => format!("|{:^6}|", "IST"),
                        SeasonPeriod::PostSeason => format!("|{:^6}|", "POST"),
                        SeasonPeriod::AllStarGame => format!("|{:^6}|", "ASG"),
                    }
                } else {
                    "|      |".to_string()
                };
                println!(
                    "{}{}",
                    prefix,
                    columns
                        .iter()
                        .enumerate()
                        .map(|(i, _)| {
                            let marker = if (schema >> i) & 1 == 1 { "✓" } else { "✗" };
                            format!("{:^width$}|", marker, width = col_width)
                        })
                        .collect::<Vec<_>>()
                        .join("")
                );
            }
            println!(
                "+------+{}+",
                (0..columns.len())
                    .map(|_| "-".repeat(col_width))
                    .collect::<Vec<_>>()
                    .join("+")
            );
        }
    }
}
