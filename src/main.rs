use std::collections::{HashMap};
use corrections::correction::Correction;
use stats;

#[tokio::main]
async fn main() {
    println!("hello, {}!", "lisan al-gaib"); //TODO: make this say hi to the user with auth/name

    /*
    chronicle_nba().await;

    let res = ask_nba(1969, NBAStatKind::Player).await;

    match res {
        Ok(_) => println!("stats for 1969 successfully saved."),
        Err(e) => eprintln!("{}", e)
    }*/
    //
    // save_nba_season(1969).await;

    //
    // match res {
    //     Ok(_) => println!("nba stats were successfully corrected."),
    //     Err(e) => eprintln!("{}", e)
    // }

    let mut c = Correction::new("0024600331".to_string(), 77665, 1946, stats::kind::NBAStatKind::Player);

    c.corrections = {
        let mut m = HashMap::new();


        /*

        ["SEASON_ID","PLAYER_ID","PLAYER_NAME","TEAM_ID","TEAM_ABBREVIATION","TEAM_NAME","GAME_ID","GAME_DATE","MATCHUP","WL","MIN","FGM","FGA","FG_PCT","FG3M","FG3A","FG3_PCT","FTM","FTA","FT_PCT","OREB","DREB","REB","AST","STL","BLK","TOV","PF","PTS","PLUS_MINUS","FANTASY_PTS","VIDEO_AVAILABLE"]

        */
        m.insert(stats::stat_column::StatColumn::PLAYER_NAME, stats::stat_value::StatValue::new());

        // Team identification
        m.insert(stats::stat_column::StatColumn::TEAM_ABBREVIATION, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::TEAM_NAME, stats::stat_value::StatValue::new());

        // Game data
        m.insert(stats::stat_column::StatColumn::MATCHUP, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::GAME_DATE, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::WL, stats::stat_value::StatValue::new());

        // Classic box score

        //minutes
        m.insert(stats::stat_column::StatColumn::MIN, stats::stat_value::StatValue::new());

        //std field goals
        m.insert(stats::stat_column::StatColumn::FGM, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::FGA, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::FG_PCT, stats::stat_value::StatValue::new());

        //3-pt field goals
        m.insert(stats::stat_column::StatColumn::FG3M, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::FG3A, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::FG3_PCT, stats::stat_value::StatValue::new());

        //free throws
        m.insert(stats::stat_column::StatColumn::FTM, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::FTA, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::FT_PCT, stats::stat_value::StatValue::new());

        //rebounds
        m.insert(stats::stat_column::StatColumn::OREB, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::DREB, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::REB, stats::stat_value::StatValue::new());


        m.insert(stats::stat_column::StatColumn::AST, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::STL, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::BLK, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::TOV, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::PF, stats::stat_value::StatValue::new());
        m.insert(stats::stat_column::StatColumn::PTS, stats::stat_value::StatValue::new());

        m.insert(stats::stat_column::StatColumn::PLUS_MINUS, stats::stat_value::StatValue::new());

        m
    };


    c.create();

    match c.apply() {
        Ok(()) => println!("nba stats were is successfully corrected"),
        Err(e) => println!("failed to apply corrections: {}", e)
    };


}