#[cfg(test)]
mod test_save_load_verify {
    use crate::checksum::{checksum_map::ChecksumMap, generate::generate_checksums};

    #[test]
    fn test_generate_save_load_verify() {
        let map = generate_checksums();
        let result = map.save();
        match result.clone() {
            Ok(_) => println!("ChecksumMap saved successfully"),
            Err(e) => println!("Failed to save ChecksumMap: {}", e),
        }
        assert!(result.is_ok());

        let loaded_map = ChecksumMap::load();
        match &loaded_map {
            Ok(_) => println!("ChecksumMap loaded successfully"),
            Err(e) => println!("Failed to load ChecksumMap: {}", e),
        }
        assert!(loaded_map.is_ok());

        let verify_result = map.verify_checksums(&loaded_map.unwrap());
        match verify_result.len() == 0 {
            true => println!("no checksum errors."),
            false => println!("checksum errors detected."),
        }
        assert_eq!(verify_result.len(), 0);
    }
}

/// this test was created to check whether my correction workflow was correctly updating the box scores solely in memroy.
/// it can be run ahead of runtime to verify accuracy of starting data.
#[cfg(test)]
mod assert_checksums {
    use crate::checksum::{checksum_map::ChecksumMap, generate::generate_checksums};

    #[test]
    fn test_assert_checksums() {
        let expected_map = ChecksumMap::load().expect("failed to load checksum from checksum file. check that data has been initialized properly");

        let actual_map = generate_checksums();

        pretty_assertions::assert_eq!(expected_map, actual_map);
    }
}
