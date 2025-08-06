use crate::corrections::correction_builder::CorrectionBuilder;
use crate::dapi::box_score_stat::BoxScoreStat;
use crate::format::language::box_score_value_to_string;
use crate::stats::box_score::BoxScoreBuilder;
use crate::stats::id::{Identifiable, Identity};
use crate::stats::stat_column::StatColumn;
use crate::stats::stat_value::StatValue;
use serde_json::Value;
use std::collections::HashMap;

pub fn json_to_hashmap(value: &Value) -> Result<HashMap<Identity, String>, String> {
    let result_set = get_result_set(&value)?;

    let rows = get_rows(&result_set)?;

    Ok((&rows)
        .iter()
        .map(|v| (v.identity(), box_score_value_to_string(v)))
        .collect())
}

pub fn get_result_set(v: &Value) -> Result<Value, String> {
    let result_sets = v
        .get("resultSets")
        .and_then(|rs| rs.as_array())
        .ok_or_else(|| "resultSets is not an array or is missing")?;

    let result_set = result_sets
        .get(0)
        .ok_or_else(|| "resultSets array is empty")?;

    Ok(result_set.clone())
}

pub fn headers(s: &Value) -> Result<Vec<String>, String> {
    Ok(s.get("headers")
        .and_then(|h| h.as_array())
        .ok_or_else(|| "Missing or invalid 'headers' field".to_string())?
        .iter()
        .filter_map(|h| Option::from(h.to_string()))
        .collect())
}

pub fn get_rows(set: &Value) -> Result<Vec<Value>, String> {
    Ok(set
        .get("rowSet")
        .and_then(|r| r.as_array())
        .ok_or_else(|| "Missing or invalid 'rowSet' field")?
        .clone())
}

pub fn record_stat<T>(
    entry: Result<T, StatColumn>,
    box_score: &mut BoxScoreBuilder,
    correction: &mut CorrectionBuilder,
) where
    T: Into<BoxScoreStat>,
{
    match entry {
        Ok(stat) => match stat.into() {
            BoxScoreStat::GameResult(wl) => {
                box_score.wl(wl);
            }
            BoxScoreStat::Minutes(min) => {
                box_score.min(min);
            }
            BoxScoreStat::FieldGoalMakes(fgm) => {
                box_score.fgm(fgm);
            }
            BoxScoreStat::FieldGoalAttempts(fga) => {
                box_score.fga(fga);
            }
            BoxScoreStat::ThreePointMakes(fg3m) => {
                box_score.fg3m(fg3m);
            }
            BoxScoreStat::ThreePointAttempts(fg3a) => {
                box_score.fg3a(fg3a);
            }
            BoxScoreStat::FreeThrowMakes(ftm) => {
                box_score.ftm(ftm);
            }
            BoxScoreStat::FreeThrowAttempts(fta) => {
                box_score.fta(fta);
            }
            BoxScoreStat::OffensiveRebounds(oreb) => {
                box_score.oreb(oreb);
            }
            BoxScoreStat::DefensiveRebounds(dreb) => {
                box_score.dreb(dreb);
            }
            BoxScoreStat::Rebounds(reb) => {
                box_score.reb(reb);
            }
            BoxScoreStat::Assists(ast) => {
                box_score.ast(ast);
            }
            BoxScoreStat::Steals(stl) => {
                box_score.stl(stl);
            }
            BoxScoreStat::Blocks(blk) => {
                box_score.blk(blk);
            }
            BoxScoreStat::Turnovers(tov) => {
                box_score.tov(tov);
            }
            BoxScoreStat::PersonalFouls(pf) => {
                box_score.pf(pf);
            }
            BoxScoreStat::Points(pts) => {
                box_score.pts(pts);
            }
            BoxScoreStat::PlusMinus(plus_minus) => {
                box_score.plus_minus(plus_minus);
            }
        },
        Err(col) => {
            correction.add_missing_field(col, Value::Null);
        }
    };
}

/// ## record_usable_stat
/// `record_usable_stat` takes a result of a type (must be a member of the BoxScoreStat enum) and
/// the column (StatColumn) and returns the wrapped Result value `entry: Result<T, StatColumn>`.
pub fn record_usable_stat<T>(
    entry: Result<T, StatColumn>,
    box_score: &mut BoxScoreBuilder,
    correction: &mut CorrectionBuilder,
) -> Result<T, StatColumn>
where
    T: Clone,
    T: Into<BoxScoreStat>,
{
    record_stat(entry.clone(), box_score, correction);

    entry
}
