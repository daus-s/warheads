use crate::checksum::checksum::checksum;
use std::path::PathBuf;

pub fn read_checksum(path: &PathBuf) -> Result<u32, std::io::Error> {
    let data = std::fs::read(path)?;

    let checksum = checksum(&data);

    Ok(checksum)
}
