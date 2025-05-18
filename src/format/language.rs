pub trait Columnizable {
    fn columns(&self) -> Vec<String>;
}

impl Columnizable for String {
    fn columns(&self) -> Vec<String> {
        self.replace(['[', ']'], "")
            .split(",")
            .map(|x| x.to_string().trim().into())
            .collect()
    }
}

pub fn partition(txt: String, list: Vec<String>) -> String {
    let new_data = format!("[\n        {}\n      ]\n", list.join(",\n        "));

    let beginning = "\"rowSet\": ";

    let end_of_start = txt.find(beginning).unwrap() + beginning.len(); /* Ex.

                                                                                 "rowSet:["
                                                                                         ^
                                                                                         Starting from and including [
                                                                                */

    let (prefix, _) = txt.split_at(end_of_start);

    let suffix = "    }\n  ]\n}\n";

    format!("{}{}{}", prefix, new_data, suffix)
}
