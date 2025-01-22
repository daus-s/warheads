use dapi::rip::{process_nba};
use dapi::gather::nba;
use dapi::gather::NBAStatKind::*;

fn main() {
    println!("hello, lisan al-gaib!"); //TODO: make this say hi to the user with auth/name

    let formatted = process_nba(&nba(2023, PLAYER));
    match formatted {
        Ok(csv) => println!("formatted csv: \n{}", "SUCCESS"),
        Err(e) => eprintln!("error: {}", e),
    }
}
