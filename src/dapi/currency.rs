//not money, is the data current

use crate::proc::refresher::get_daily_gamecard;

use crate::stats::gamecard::GameCard;

pub async fn source_data_current() -> bool {
    let response = get_daily_gamecard().await;

    if let Ok(gamecards) = response {
        check_source_data_for_games(&gamecards)
    } else {
        return false;
    }
}

fn check_source_data_for_games(gamecards: &[GameCard]) -> bool {
    let mut present = true;

    for gamecard in gamecards {
        present &= gamecard.check_source_data();
    }
    present
}

#[tokio::test]
async fn test_source_data_current() {
    println!("source_data_current(): {}", source_data_current().await);
}
