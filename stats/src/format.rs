use crate::team_box_score::TeamBoxScore;

pub(crate) fn format_matchup(matchup: &String) -> String {
    let parts:Vec<&str> = matchup.split_whitespace().collect();

    if parts.len() != 3 {
        panic!("game name does not have three terms.")
    }

    match parts.as_slice() {
        [home, "vs.", away] => format!("{} vs. {}", home, away.to_ascii_lowercase()),
        [away, "@", home]  => format!("{} @ {}", away, home.to_ascii_lowercase()),
        _ => panic!("Could not reformat the game; unexpected pattern."),
    }
}

/// Returns the opposing team's abbreviation unmodified.
/// It will be a 3 character capitalized string, this rule can
/// and should be used for validation.
///
/// # Arguments
///
/// * `matchup`:matchup string provided by nba.com api
/// * `abbr`: team abbreviation (3 characters)
///
/// returns: String
///
/// # Examples
///
/// ```
/// let opponent = opponent("MEM @ LAL", "LAL");
///
/// assert_eq!("MEM", opponent);
/// ```
pub(crate) fn opponent(matchup: &str, abbr: &str) -> String {
    let parts:Vec<&str> = matchup.split_whitespace().collect();

    if parts.len() != 3 {
        panic!("game name does not have three terms.")
    }

    match parts.as_slice() {
        [t1, _, t2] if &abbr == t1 => t2.to_string(),
        [t1, _, t2] if &abbr == t2 => t1.to_string(),
        _ => panic!("Could not parse the opponent; unexpected pattern.")
    }

}

pub(crate) fn percent(num: Option<u32>, den: Option<u32>) -> String {
    match [num, den] {
        [Some(n), Some(d)] => format!("({:.1}%)", (n as f32 * 100.0) / d as f32),
        _ => "-".to_string(),
    }
}

pub fn season_str(games: Vec<TeamBoxScore>) -> String {
    if games.len() <= 0 {
        panic!("no games in a season");
    }

    let det = &games[0];

    det.season_str()
}

pub fn season_fmt(year: i32) -> String {

    let following_suffix = (year + 1) % 100;

    format!("{}-{}", year, following_suffix)
}

pub fn season_file(year: i32) -> String {
    let following_suffix = (year + 1) % 100;

    format!("{}_{}", year, following_suffix)
}