pub(crate) fn format_matchup(matchup: &String) -> String {
    let parts:Vec<&str> = matchup.split(" ").collect();

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
    let parts:Vec<&str> = matchup.split(" ").collect();

    if parts.len() != 3 {
        panic!("game name does not have three terms.")
    }

    match parts.as_slice() {
        [t1, _, t2] if &abbr == t1 => t2.to_string(),
        [t1, _, t2] if &abbr == t2 => t1.to_string(),
        _ => panic!("Could not parse the opponent; unexpected pattern.")
    }

}