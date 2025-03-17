use std::io;

pub fn input() -> io::Result<()> {
    let mut user_input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut user_input)?;

    std::println!("input: {} ", user_input);

    Ok(())
}