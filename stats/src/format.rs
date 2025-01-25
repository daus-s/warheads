pub(crate) fn format_matchup(matchup: &String, abbr: &String) -> String {
    let parts:Vec<&str> = matchup.split(" ").collect();

    if parts.len() != 3 { panic!("game name does not have three terms.") }

    match parts.as_slice() {
        [home, "vs.", away] => format!("{} vs. {}", home, away.to_ascii_lowercase()),
        [away, "@", home]  => format!("{} @ {}", home.to_ascii_lowercase(), away),
        _ => panic!("Could not reformat the game; unexpected pattern."),
    }
}