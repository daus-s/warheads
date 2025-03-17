use std::any::type_name;
use std::fmt::Debug;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::str::FromStr;
use serde::Serialize;
use serde_json::{json, Value};

pub fn prompt_and_validate<T>(prompt: &str) -> Value
where
    T: FromStr,
    T: Serialize,
    T::Err: Debug,
{
    let t = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(|input: &String| { validate_t::<T>(input) })
        .interact_text()
        .unwrap()
        .parse::<T>(); //this may be the goat code ive ever written

    match t {
        Ok(t) =>
            match type_name::<T>() {
                "i32" | "alloc::string::String" | "u32" | "f32" | "u64" => json!(t),
                "chrono::naive::date::NaiveDate" => json!(prompt),
                "stats::nba::MatchupString" => json!(t),
                _ => panic!("Unrecognized type: {}", type_name::<T>()), // Panic for unrecognized types
            }
        Err(e) => json!(None::<T>),
    }
}


fn validate_t<T>(input: &String) -> Result<(), String>
where
    T: FromStr
{
    println!("{}", type_name::<T>());

    match type_name::<T>() {
        _ => input.
        parse::<T>()
        .map(| _ | ())
        .map_err(| _ | format!("could not parse a {} from \"{}\".", type_name::<T>(), input))
    }
}

pub fn prompt_and_select(prompt: &str) -> Value {
    let choices = &["W", "L", "D"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(choices)
        .default(0)
        .interact()
        .unwrap();

    json!(choices[selection].to_string())
}