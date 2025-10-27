//not money, is the data current
//

use std::fs;

use crate::stats::nba_kind::NBAStatKind;
use crate::types::GameDate;
use crate::{dapi::season_manager::get_current_era, format::path_manager::nba_source_path};

pub async fn source_data_current() -> bool {
    let era = get_current_era();

    let team_path = nba_source_path(era, NBAStatKind::Team);
    let player_path = nba_source_path(era, NBAStatKind::Player);

    let team_contents = fs::read_to_string(team_path);
    let player_contents = fs::read_to_string(player_path);

    let today = GameDate::today().to_string(); // yyyy-mm-dd

    if let Err(_) = team_contents {
        return false;
    } else if let Ok(contents) = team_contents {
        return contents.contains(&today.to_string());
    };
    if let Err(_) = player_contents {
        return false;
    } else if let Ok(contents) = player_contents {
        return contents.contains(&today.to_string());
    };

    false
}

#[tokio::test]
async fn test_source_data_current() {
    println!("source_data_current(): {}", source_data_current().await);
}
