const MOD_ADLER: u32 = 65521;

// adler u32 checksum. i saw the zlib implementation
// and i am gonna have to print it out and look at wtf is happening
pub fn checksum(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;

    for byte in data {
        a = (a + *byte as u32) % MOD_ADLER;
        b = (b + a) % MOD_ADLER;
    }

    b << 16 | a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        let data = "Wikipedia".as_bytes();
        let c = checksum(data);
        assert_eq!(c, 0x11E60398);
    }
}
