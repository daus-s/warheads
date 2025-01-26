pub(crate) fn format_matchup(matchup: &String, abbr: &String) -> String {
    let parts:Vec<&str> = matchup.split(" ").collect();

    if parts.len() != 3 {
        panic!("game name does not have three terms.")
    }

    match parts.as_slice() {
        [home, "vs.", away] => format!("{} vs. {}", home, away.to_ascii_lowercase()),
        [away, "@", home]  => format!("{} @ {}", home.to_ascii_lowercase(), away),
        _ => panic!("Could not reformat the game; unexpected pattern."),
    }
}

pub(crate) fn opponent(matchup: &String, abbr: &String) -> String {
    let parts:Vec<&str> = matchup.split(" ").collect();

    if parts.len() != 3 {
        panic!("game name does not have three terms.")
    }

    match parts.as_slice() {
        [t1, _, t2] if abbr == t1 => t2.to_string(),
        [t1, _, t2] if abbr == t2 => t1.to_string(),
        _ => panic!("Could not parse the opponent; unexpected pattern.")
    }

}