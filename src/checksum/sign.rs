use crate::checksum::generate::generate_checksums;

pub fn sign_nba() -> Result<(), ()> {
    let checksums = generate_checksums();

    checksums.save().map_err(|_| ())
}
