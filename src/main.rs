use dapi::rip::{nba_json_to_csv};
use dapi::gather::nba;
use dapi::gather::NBAStatKind::*;

fn main() {
    println!("hello, lisan al-gaib!"); //TODO: make this say hi to the user with auth/name

    let formatted = nba_json_to_csv(&nba(2003, PLAYER));
    match formatted {
        Ok(csv) => println!("formatted csv: \n{}", csv),
        Err(e) => eprintln!("error: {}", e),
    }
}
