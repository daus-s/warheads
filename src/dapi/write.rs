use std::path::PathBuf;
use std::{fs, io};

use serde_json::Value;

pub fn write_games(file_path: &PathBuf, json: &Value) -> io::Result<()> {
    if let Some(parent) = file_path.parent() {
        //this creates the directory from the ground up.
        fs::create_dir_all(parent)?;
    }

    let json_str = serde_json::to_string(json)?;

    fs::write(&file_path, json_str)
}
