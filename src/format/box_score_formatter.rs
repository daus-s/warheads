use crate::{
    dapi::{player_box_score::PlayerBoxScore, team_box_score::TeamBoxScore},
    stats::{box_score::BoxScore, visiting::Visiting},
    types::GameResult,
};
//grid layout via ints

fn whitespace_n(n: usize) -> String {
    (0..n).map(|_| " ").collect::<String>()
}

fn whitespace_range(start: usize, end: usize) -> String {
    (start..end).map(|_| " ").collect::<String>()
}

pub fn format_team_box_score(
    f: &mut std::fmt::Formatter<'_>,
    box_score: &TeamBoxScore,
) -> std::fmt::Result {
    writeln!(
        f,
        "{} {} ({})",
        box_score.team_name(),
        match box_score.box_score().wl() {
            GameResult::Win => "W",
            GameResult::Loss => "L",
            GameResult::Draw => "?",
        },
        match box_score.visiting() {
            Visiting::Away => {
                "away"
            }
            Visiting::Home => {
                "home"
            }
        },
    )?;
    for pbs in box_score.roster_box_scores() {
        writeln!(f, "{}", pbs)?;
    }
    if box_score.roster_box_scores().len() == 0 {
        writeln!(f, "\n{}no player records\n", whitespace_n((WIDTH - 17) / 2))?;
    }
    writeln!(f, "{}", "=".repeat(WIDTH))?;
    writeln!(
        f,
        "totals:{}{}",
        whitespace_range(7, COL_NAME),
        box_score.box_score()
    )
}

pub fn format_player_box_score(
    f: &mut std::fmt::Formatter<'_>,
    box_score: &PlayerBoxScore,
) -> std::fmt::Result {
    write!(
        f,
        "{}{}{} (fpts: {}) ",
        box_score.player_name(),
        whitespace_n(COL_NAME - box_score.player_name().0.len()),
        box_score.box_score(),
        box_score.box_score().calculate_fantasy()
    )
}

const WIDTH: usize = COL_NAME
    + COL_PTS
    + COL_FGS
    + COL_3PS
    + COL_FT
    + COL_AST
    + COL_REB
    + COL_STL
    + COL_BLK
    + COL_TOV
    + COL_FPTS
    + 11 * 2;

const COL_NAME: usize = 25;
const COL_PTS: usize = 8;
const COL_FGS: usize = 12;
const COL_3PS: usize = 12;
const COL_FT: usize = 12;
const COL_AST: usize = 7;
const COL_REB: usize = 27;
const COL_STL: usize = 7;
const COL_BLK: usize = 7;
const COL_TOV: usize = 7;
const COL_FPTS: usize = 11;

pub fn format_statistical_box_score(
    f: &mut std::fmt::Formatter<'_>,
    box_score: &BoxScore,
) -> std::fmt::Result {
    write!(
        f,
        " pts: {:>width$} ",
        box_score.pts().0,
        width = COL_PTS - 5
    )?;
    write!(
        f,
        " fgs: {:>3}/{:<3} ",
        box_score.fgm().0,
        box_score.fga().0.unwrap_or(0)
    )?;
    write!(
        f,
        " 3ps: {:>3}/{:<3} ",
        box_score.fg3m().0.unwrap_or(0),
        box_score.fg3a().0.unwrap_or(0)
    )?;
    write!(
        f,
        " ft: {:>3}/{:<3} ",
        box_score.ftm().0,
        box_score.fta().0.unwrap_or(0)
    )?;

    write!(
        f,
        " ast: {:>width$} ",
        box_score.ast().0.unwrap_or(0),
        width = COL_AST - 5
    )?;
    write!(
        f,
        " {:<COL_REB$} ",
        format_args!(
            "reb: {:>3} (oreb: {:>2}, dreb: {:>2})",
            box_score.reb().0.unwrap_or(0),
            box_score.oreb().0.unwrap_or(0),
            box_score.dreb().0.unwrap_or(0)
        )
    )?;
    write!(
        f,
        " stl: {:>width$} ",
        box_score.stl().0.unwrap_or(0),
        width = COL_STL - 5
    )?;
    write!(
        f,
        " blk: {:>width$} ",
        box_score.blk().0.unwrap_or(0),
        width = COL_BLK - 5
    )?;
    write!(
        f,
        " tov: {:>width$} ",
        box_score.tov().0.unwrap_or(0),
        width = COL_TOV - 5
    )?;
    Ok(())
}
