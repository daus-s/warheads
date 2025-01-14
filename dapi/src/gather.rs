
use std::fs::File;
use std::io::Read;
use NBAStatKind::{TEAM, PLAYER, LINEUP};

const PREFIX: &str = "/Users/daviscarmichael/Documents/warheads/data";
pub fn nba(season: u16, stat: NBAStatKind) -> String {
    let suffix: u16 = (season + 1) % 100;
    let filename = format!("{}/nba/{}_{:02}_{}", PREFIX, season, suffix, extension(stat));

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
        TEAM => "team",
        PLAYER => "player",
        LINEUP => panic!("lineup stats are not supported yet."),
    };

    format!(
        "ERROR: could not open file for the {}-{} season viewing {} box scores",
        season,
        season % 100,
        stat_description
    )
}

fn dbg_write(season: u16, stat: NBAStatKind) -> String {
    let stat_description = match stat {
        TEAM => "team",
        PLAYER => "player",
        LINEUP => panic!("lineup stats are not supported yet."),
    };

    format!(
        "ERROR:could not write the contents of the file for the {}-{} season viewing {} box scores",
        season,
        season % 100,
        stat_description
    )
}


fn extension(stat: NBAStatKind) -> &'static str {
    match stat {
        TEAM => "tg.json",
        PLAYER => "pg.json",
        LINEUP => panic!("lineup stats are not supported yet."),
    }
}
#[derive(Copy, Clone)]
pub enum NBAStatKind {
    TEAM,
    PLAYER,
    LINEUP //todo: develop this later
    // this is not a priority yet.
}