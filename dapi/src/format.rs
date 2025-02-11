use stats::kind::NBAStatKind;
use stats::kind::NBAStatKind::{LineUp, Player, Team};


pub(crate) fn dbg_open(season: i32, stat: NBAStatKind) -> String {
    let stat_description = match stat {
        Team => "team",
        Player => "player",
        LineUp => panic!("lineup stats are not supported yet."),
    };

    format!(
        "ERROR: could not open file for the {}-{} season viewing {} box scores",
        season,
        (season+1) % 100,
        stat_description
    )
}

pub(crate) fn dbg_write(season: i32, stat: NBAStatKind) -> String {
    let stat_description = match stat {
        Team => "team",
        Player => "player",
        LineUp => panic!("lineup stats are not supported yet."),
    };

    format!(
        "ERROR:could not write the contents of the file for the {}-{} season viewing {} box scores",
        season,
        season % 100,
        stat_description
    )
}


pub(crate) fn ext(stat: NBAStatKind) -> &'static str {
    match stat {
        Team => "tg.json",
        Player => "pg.json",
        LineUp => panic!("lineup stats are not supported yet."),
    }
}

pub(crate) fn epath(stat: NBAStatKind) -> &'static str {
    match stat {
        Team => "teamgames",
        Player => "playergames",
        LineUp => panic!("lineup stats are not supported yet."),
    }
}

