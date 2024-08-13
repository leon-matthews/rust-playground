#![allow(dead_code)]

use rand::RngCore;
use rand::rngs::SmallRng;


#[derive(Debug)]
struct DiceRoller {
    rng: SmallRng,
    buffer: [u8; 21],
    current: usize,
}


impl DiceRoller {
    fn new(rng: SmallRng) -> Self {
        let buffer = [0_u8; 21];
        let mut new = Self {
            rng,
            buffer,
            current: 0,
        };
        new.refill_buffer();
        new
    }

    pub fn d6(&mut self) -> u8 {
        loop {
            if self.current == self.buffer.len() {
                self.refill_buffer();
            }

            let roll = self.buffer[self.current] + 1;
            self.current += 1;

            if roll < 7 {
                break roll;
            }
        }
    }

    fn refill_buffer(&mut self) {
        let mut number = self.rng.next_u64();
        for i in 0..21 {
            self.buffer[i] = (number & 0x07) as u8;
            number = number >> 3;
        }
        self.current = 0;
    }
}


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

    use rand::SeedableRng;

    const ZERO: u64 = 0;
    const INCREMENTING: u64 = 0x0102030405060708;

    /// Create dice roller using deterministic (and terrible) seed for testing.
    fn create_roller() -> DiceRoller {
        let rng_bad = SmallRng::seed_from_u64(0x00);
        DiceRoller::new(rng_bad)
    }

    /// Build a vector of `count` d6 rolls
    fn roll_many(count: usize) -> Vec<u8> {
        let mut roller = create_roller();
        std::iter::from_fn(|| Some(roller.d6()))
            .take(count)
            .collect()
    }

    #[test]
    fn roll_d6() {
        let mut roller = create_roller();
        assert_eq!(roller.d6(), 5);
        assert_eq!(roller.d6(), 2);
        assert_eq!(roller.d6(), 3);
    }

    #[test]
    fn forty_rolls() {
        let expected = [
            5, 2, 3, 5, 2, 5, 6, 5, 1, 6, 6, 5, 5, 2, 2, 4, 1, 3, 2, 5,
            4, 4, 5, 2, 1, 5, 3, 4, 4, 6, 2, 1, 3, 3, 1, 1, 3, 4, 1, 1,
        ];
        assert_eq!(roll_many(40), expected);
    }

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
