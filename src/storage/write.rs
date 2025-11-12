use std::path::Path;
use std::{fs, io};

use serde::Serialize;

pub fn write_serializable_with_directory<P: AsRef<Path>, S: Serialize>(
    file_path: P,
    content: &S,
) -> io::Result<()> {
    if let Some(parent) = file_path.as_ref().parent() {
        //this creates the directory from the ground up.
        fs::create_dir_all(parent)?;
    }

    let json_str = serde_json::to_string(content)?;

    fs::write(file_path.as_ref(), json_str)
}
