use crate::dapi::archive::Archive;
use crate::format::language::partition;
use crate::format::season::season_fmt;
use crate::stats::domain::Domain;

pub fn overwrite(
    domain: Domain,
    corrected_data: Vec<String>,
    archive: &mut impl Archive,
) -> Result<(), String> {
    let new_content = partition(archive.contents(), corrected_data);

    match archive.write(new_content) {
        Ok(_) => {
            println!(
                "✅ successfully saved corrected data for the {} season the in the archive: {}",
                season_fmt(domain.0.year()),
                archive.path()
            );

            Ok(())
        }
        Err(_) => Err(format!(
            "❌ failed to write to the archive: {}",
            archive.path()
        )),
    }
}
