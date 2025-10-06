use crate::checksum::checksum::checksum;
use crate::format::path_manager::nba_data_path;
use crate::stats::domain::Domain;

pub fn checksum_pair(domain: Domain) -> Result<(Domain, u32), std::io::Error> {
    let (season, kind) = domain;
    let path = nba_data_path(season, kind);

    let data = std::fs::read(path)?;

    let checksum = checksum(&data);

    Ok((domain, checksum))
}
