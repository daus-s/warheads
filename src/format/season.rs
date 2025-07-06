use crate::types::SeasonId;

pub fn season_fmt(year: i32) -> String {
    let following_suffix = (year + 1) % 100;

    format!("{}-{:02}", year, following_suffix)
}

pub fn season_path(season: &SeasonId) -> String {
    let year = season.year();

    let following_suffix = (year + 1) % 100;

    format!("{}_{:02}", year, following_suffix)
}
