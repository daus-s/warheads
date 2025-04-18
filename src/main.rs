use corrections::correction_loader::load_corrections;
use corrections::corrector::Corrector;
use dapi::hunting::{observe_nba};
use stats;
use stats::kind::NBAStatKind::Player;

#[tokio::main]
async fn main() {
    println!("hello, {}!", "lisan al-gaib"); //TODO: make this say hi to the user with auth/name

    observe_nba().await;

}