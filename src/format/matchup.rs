pub fn format_matchup(matchup: String) -> String {
    let parts: Vec<&str> = matchup.split_whitespace().collect();

    if parts.len() != 3 {
        panic!("game name does not have three terms.")
    }

    match parts.as_slice() {
        [home, "vs.", away] => format!("{} vs. {}", home, away.to_ascii_lowercase()),
        [away, "@", home] => format!("{} @ {}", away, home.to_ascii_lowercase()),
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
/// let opponent = warheads::format::matchup::opponent("MEM @ LAL", "LAL");
///
/// assert_eq!("MEM", opponent);
/// ```
pub fn opponent(matchup: &str, abbr: &str) -> String {
    let parts: Vec<&str> = matchup.split_whitespace().collect();

    if parts.len() != 3 {
        panic!("game name does not have three terms.")
    }

    match parts.as_slice() {
        [t1, _, t2] if &abbr == t1 => t2.to_string(),
        [t1, _, t2] if &abbr == t2 => t1.to_string(),
        _ => panic!("Could not parse the opponent; unexpected pattern."),
    }
}

pub fn display_matchup(matchup: String, abbr: String) -> Result<String, String> {
    let parts: Vec<&str> = matchup.split_whitespace().collect();

    match parts.as_slice() {
        [home, "vs.", away] | [away, "@", home] => {
            let home_lower = home.to_ascii_lowercase();
            let away_lower = away.to_ascii_lowercase();
            let abbr_lower = abbr.to_ascii_lowercase();

            // eprintln!("home: {:#}\naway:{:#}\nself: {:#}", home_lower, away_lower, abbr_lower);
            // eprintln!("is home team: {}", home_lower==abbr_lower);
            // eprintln!("is away team: {}", away_lower==abbr_lower);

            if home_lower == abbr_lower {
                Ok(format!("{} @ {}", away_lower, abbr.to_ascii_uppercase()))
            } else if away_lower == abbr_lower {
                Ok(format!("{} @ {}", abbr.to_ascii_uppercase(), home_lower))
            } else {
                Err(format!(
                    "Abbreviation '{}' doesn't match either team in matchup '{}'",
                    abbr, matchup
                ))
            }
        }
        _ => Err(format!(
            "Invalid matchup format: '{}'. Expected either '[team1] vs. [team2]' or '[team1] @ [team2]'",
            matchup
        )),
    }
}
