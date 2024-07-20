
/// Convert u64 to an array of 16 u8 bytes, using manual bit-shifts
pub fn build_array_manually(number: u64) -> [u8; 16] {
    let mut number = number;
    let mut rolls: [u8; 16] = [0_u8; 16];
    for i in 0..16 {
        rolls[i] = (number & 0x0f) as u8;
        number = number >> 4;
    }
    rolls
}


/// Convert u64 to an array of 16 u8 bytes, using stdlib
pub fn build_array_stdlib(number: u64) -> [u8; 16] {
    let mut rolls: [u8; 16] = [0_u8; 16];
    let bytes = number.to_le_bytes();
    let mut index = 0;
    for byte in bytes {
        // Extract bottom four bits
        rolls[index] = &byte & 0x0f;
        index += 1;

        // Extract top four bits
        rolls[index] = &byte >> 4;
        index += 1;
    }
    rolls
}


#[cfg(test)]
mod tests {
    use super::*;

    const ZERO: u64 = 0;
    const INCREMENTING: u64 = 0x0102030405060708;

    #[test]
    fn build_array_manually_zero() {
        let expected = [0_u8; 16];
        assert_eq!(build_array_manually(ZERO), expected);
    }

    #[test]
    fn build_array_manually_incrementing() {
        // Note that bytes are extracted in little-endian order
        let expected: [u8; 16] = [8, 0, 7, 0, 6, 0, 5, 0, 4, 0, 3, 0, 2, 0, 1, 0];
        assert_eq!(build_array_manually(INCREMENTING), expected);
    }

    #[test]
    fn build_array_stdlib_zero() {
        let expected = [0_u8; 16];
        assert_eq!(build_array_stdlib(ZERO), expected);
    }

    #[test]
    fn build_array_stdlib_incrementing() {
        // Little endian-order as we used `u64::to_le_bytes()`
        let expected: [u8; 16] = [8, 0, 7, 0, 6, 0, 5, 0, 4, 0, 3, 0, 2, 0, 1, 0];
        assert_eq!(build_array_stdlib(INCREMENTING), expected);
    }
}
