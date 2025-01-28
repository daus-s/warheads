use dotenv;
use std::fs::File;
use std::io::Read;
use once_cell::sync::Lazy;
use stats::kind::NBAStatKind;
use stats::kind::NBAStatKind::{Team, Player, LineUp};

static PREFIX: Lazy<String> = Lazy::new(prefix);
pub fn read_nba(season: u16, stat: NBAStatKind) -> String {
    let suffix: u16 = (season + 1) % 100;
    let filename = format!("{}/nba/{}/{}_{:02}_{}", *PREFIX, epath(stat), season, suffix, ext(stat));

    let mut file = File::open(&filename).expect(&dbg_open(
        season,
        stat
    ));
    let mut data = String::new();

    file.read_to_string(&mut data).expect(&dbg_write(season, stat));

    data

}

fn dbg_open(season: u16, stat: NBAStatKind) -> String {
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

fn dbg_write(season: u16, stat: NBAStatKind) -> String {
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


fn ext(stat: NBAStatKind) -> &'static str {
    match stat {
        Team => "tg.json",
        Player => "pg.json",
        LineUp => panic!("lineup stats are not supported yet."),
    }
}

fn epath(stat: NBAStatKind) -> &'static str {
    match stat {
        Team => "teamgames",
        Player => "playergames",
        LineUp => panic!("lineup stats are not supported yet."),
    }
}


fn prefix() -> String {
    dotenv::dotenv().ok();

    dotenv::var("PREFIX").unwrap()
}

