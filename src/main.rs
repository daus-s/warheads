use chrono::Timelike;
use warheads::proc::historian::{chronicle_nba, observe_nba, rate_nba};
use warheads::proc::refresher::update_source_data;

#[tokio::main]
async fn main() {
    println!("hello, {}!", "lisan al-gaib"); //TODO: make this say hi to the user with auth/name

    observe_nba().await;

    chronicle_nba();

    rate_nba();

    loop {
        let time = chrono::Utc::now();

        if time.minute() == 0 {
            if update_source_data().await {};
            //update_ratings();
        }
    }
}
