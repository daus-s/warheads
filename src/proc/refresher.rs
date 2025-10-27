use crate::format::parse::parse_gamecards;

use crate::proc::query::{daily_gamecard_json, NBAQueryError};

use crate::stats::gamecard::GameCard;
use crate::types::GameDate;

// game schedule tracker
pub async fn get_daily_gamecard() -> Result<Vec<GameCard>, NBAQueryError> {
    let response = daily_gamecard_json(GameDate::today()).await?;

    let gamecards =
        parse_gamecards(response).map_err(|e| NBAQueryError::ObjectStructureError(e))?;

    Ok(gamecards)
}

#[cfg(test)]
mod test_get_daily_gamecard {
    use super::*;

    #[tokio::test]
    async fn test_get_daily_gamecard() {
        let result = get_daily_gamecard().await;
        match &result {
            Ok(gamecards) => {
                assert!(!gamecards.is_empty());
            }
            Err(err) => {
                panic!("💀 failed to get daily gamecard: {}", err);
            }
        }
    }
}
