use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::Serialize;
use serde_json::{json, Value};
use crate::stats::se::SerdeEnum;
use std::any::type_name;
use std::fmt::Debug;
use std::str::FromStr;
use dialoguer::console::{style, Term};

pub fn prompt_and_validate<T>(prompt: &str) -> Value
where
    T: FromStr,
    T: Serialize,
    T::Err: Debug,
{
    let t = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(|input: &String| validate_t::<T>(input))
        .interact_text()
        .unwrap()
        .parse::<T>(); //this may be the goat code ive ever written

    match t {
        Ok(t) => match type_name::<T>() {
            "i32" | "alloc::string::String" | "u32" | "f32" | "u64" | "stats::percent::Percent" => {
                json!(t)
            }
            "chrono::naive::date::NaiveDate" => json!(t),
            "stats::types::MatchupString" => json!(t),
            _ => panic!("Unrecognized type: {}", type_name::<T>()), // Panic for unrecognized types
        },
        Err(_) => json!(None::<T>),
    }
}

fn validate_t<T>(input: &String) -> Result<(), String>
where
    T: FromStr,
{
    if input == "null" {
        return Ok(());
    }

    match type_name::<T>() {
        _ => input
            .parse::<T>()
            .map(|_| ())
            .map_err(|_| format!("could not parse a {} from \"{}\".", type_name::<T>(), input)),
    }
}

pub fn prompt_and_select<S>(prompt: &str) -> Value
where
    S: SerdeEnum,
{
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(
            &S::enumerate()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        )
        .default(0)
        .interact()
        .unwrap();

    S::evaluate()[selection].clone()
}

pub fn prompt_and_delete(comparator: &str) -> bool
{
    let term = Term::stderr();

    let should_delete = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Should this entry be deleted? (uncommon)")
        .items(&["No", "Yes"])
        .default(0)
        .interact()
        .unwrap() == 1; // 1 is "Yes"

    if !should_delete {
        return false;
    }


    // prompt and validate that the newly typed thing is the comparator.
    // repeat until correct or canceled
    loop {
        println!(
            "{} Type '{}' to confirm deletion or press ESC to cancel",
            style("WARNING:").red().bold(),
            style(comparator).yellow().bold()
        );


        match Input::<String>::new()
            .with_prompt("Confirmation")
            .interact_text()
        {
            Ok(input) if input == comparator => {

                return true
            },
            Ok(_) => {
                println!("{} Incorrect input. Try again or press ESC to cancel.",
                         style("Error:").red());
            },
            Err(_) => {
                // User pressed ESC or had input error

                return prompt_and_delete(comparator)
            }
        }
    }
}