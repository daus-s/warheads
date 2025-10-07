#[cfg(test)]
mod test_save {
    use crate::checksum::checksum_map::ChecksumMap;

    #[test]
    fn test_save() {
        let map = ChecksumMap::new();
        let result = map.save();
        match result.clone() {
            Ok(_) => println!("ChecksumMap saved successfully"),
            Err(e) => println!("Failed to save ChecksumMap: {}", e),
        }
        assert!(result.is_ok());
    }
}

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
