use std::collections::{HashMap, HashSet};
use corrections::correction::Correction;
use dapi::gather::ask_nba;
use dapi::hunting::{chronicle_nba, save_nba_season};
use stats::kind::NBAStatKind;
use stats::stat_column::{StatColumn, StatEntry};

#[tokio::main]
async fn main() {
    println!("hello, lisan al-gaib!"); //TODO: make this say hi to the user with auth/name

    chronicle_nba().await;

    let res = ask_nba(1969, NBAStatKind::Player).await;

    match res {
        Ok(_) => println!("stats for 1969 successfully saved."),
        Err(e) => eprintln!("{}", e)
    }
    //
    // save_nba_season(1969).await;

    //
    // match res {
    //     Ok(_) => println!("nba stats were successfully corrected."),
    //     Err(e) => eprintln!("{}", e)
    // }

    println!("nba stats were is successfully saved to the library.");

}