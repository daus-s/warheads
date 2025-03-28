pub fn columns(s: String) -> Vec<String> {
    let columns = s.replace("[", "");

    let columns = columns.replace("]", "");

    columns.split(",").map(|x| x.to_string()).collect()
}

pub fn partition(txt: String, new_data: String) -> String {
    let beginning = "\"rowSet\":";

    let end_of_start = txt.find(beginning).unwrap() + beginning.len();

    let (before, _) = txt.split_at(end_of_start);

    format!("{}{}{}", before, new_data, "}]}")
}
